use numpy::{
    ndarray::{Array, Array3},
    IntoPyArray, PyArray3, PyReadwriteArray3,
};
use openh264::{
    decoder::{DecodeOptions, DecodedYUV, Decoder, DecoderConfig},
    formats::YUVSource,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

fn parse_flush_behavior(flush_behavior: &str) -> Result<openh264::decoder::Flush, &'static str> {
    match flush_behavior {
        "Auto" => Ok(openh264::decoder::Flush::Auto),
        "Flush" => Ok(openh264::decoder::Flush::Flush),
        "NoFlush" => Ok(openh264::decoder::Flush::NoFlush),
        _ => Err("Invalid flush behavior"),
    }
}

fn yuv_to_rgb_array(yuv: DecodedYUV) -> Result<Array3<u8>, &'static str> {
    let (width, height) = yuv.dimensions();
    let mut rgb = Array::zeros((width, height, 3));
    let rgb_slice = rgb.as_slice_mut().ok_or("Failed to get mutable slice")?;
    yuv.write_rgb8(rgb_slice);
    Ok(rgb)
}

/// Decodes H.264 packets to RGB frames.
///
/// # Options
/// * `flush_after_decode` - The behavior of the decoder after decoding a frame. Options are
///   "Auto", "Flush", and "NoFlush".
/// * `num_threads` - The number of threads to use for decoding. This setting is unsafe and may
///   cause seg-faults if set to a nonzero value.
/// * `debug` - Whether to enable debug console logging in OpenH264.
#[pyclass]
pub struct H264Decoder {
    decoder: Decoder,
}

#[pymethods]
impl H264Decoder {
    #[new]
    #[pyo3(signature = (flush_after_decode="Flush", num_threads=0, debug=false))]
    fn new(flush_after_decode: &str, num_threads: u32, debug: bool) -> PyResult<Self> {
        // Construct the config.
        let flush_behavior =
            parse_flush_behavior(flush_after_decode).map_err(PyValueError::new_err)?;
        let config = unsafe {
            DecoderConfig::new()
                .flush_after_decode(flush_behavior)
                .num_threads(num_threads) // Unsafe and may cause seg-faults. See documentatoin.
                .debug(debug)
        };

        // Initialize the decoder.
        let decoder = Decoder::with_api_config(openh264::OpenH264API::from_source(), config)
            .map_err(|e| PyRuntimeError::new_err(format!("Error initializing decoder: {:?}", e)))?;
        Ok(Self { decoder })
    }

    #[pyo3(signature = (packet, flush_after_decode="Auto"))]
    fn decode<'py>(
        &mut self,
        py: Python<'py>,
        packet: &[u8],
        flush_after_decode: &str,
    ) -> PyResult<Option<Bound<'py, PyArray3<u8>>>> {
        // Decode to YUV.
        let flush_behavior =
            parse_flush_behavior(flush_after_decode).map_err(PyValueError::new_err)?;
        let decode_options = DecodeOptions::new().flush_after_decode(flush_behavior);
        let decoded_yuv_opt = self
            .decoder
            .decode_with_options(packet, decode_options)
            .map_err(|e| PyValueError::new_err(format!("Error decoding packet: {:?}", e)))?;

        // Convert YUV to RGB.
        if let Some(decoded_yuv) = decoded_yuv_opt {
            let rgb = yuv_to_rgb_array(decoded_yuv).map_err(PyRuntimeError::new_err)?;
            Ok(Some(rgb.into_pyarray(py)))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (packet, buffer, flush_after_decode="Auto"))]
    fn decode_into_buffer(
        &mut self,
        packet: &[u8],
        mut buffer: PyReadwriteArray3<u8>,
        flush_after_decode: &str,
    ) -> PyResult<bool> {
        // Decode to YUV.
        let flush_behavior =
            parse_flush_behavior(flush_after_decode).map_err(PyValueError::new_err)?;
        let decode_options = DecodeOptions::new().flush_after_decode(flush_behavior);
        let decoded_yuv_opt = self
            .decoder
            .decode_with_options(packet, decode_options)
            .map_err(|e| PyValueError::new_err(format!("Error decoding packet: {:?}", e)))?;

        // Convert YUV to RGB.
        if let Some(decoded_yuv) = decoded_yuv_opt {
            let buf = buffer.as_slice_mut().map_err(PyValueError::new_err)?;
            if buf.len() != decoded_yuv.estimate_rgb_u8_size() {
                return Err(PyValueError::new_err(format!(
                    "Buffer size mismatch: expected {} but got {}",
                    decoded_yuv.estimate_rgb_u8_size(),
                    buf.len()
                )));
            }
            decoded_yuv.write_rgb8(buf);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn flush_remaining<'py>(&mut self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyArray3<u8>>>> {
        let mut frames = Vec::new();
        let remaining_yuv = self
            .decoder
            .flush_remaining()
            .map_err(|e| PyRuntimeError::new_err(format!("Corrupted bitstream: {:?}", e)))?;

        for yuv in remaining_yuv {
            let rgb = yuv_to_rgb_array(yuv).map_err(PyRuntimeError::new_err)?;
            frames.push(rgb.into_pyarray(py));
        }

        Ok(frames)
    }
}
