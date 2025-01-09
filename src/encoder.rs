use numpy::{PyReadonlyArray3, PyUntypedArrayMethods};
use openh264::{
    encoder::{Encoder, EncoderConfig},
    formats::{RgbSliceU8, YUVBuffer},
    Timestamp,
};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

fn parse_usage_type(usage_type: &str) -> Result<openh264::encoder::UsageType, &'static str> {
    match usage_type {
        "CameraVideoRealTime" => Ok(openh264::encoder::UsageType::CameraVideoRealTime),
        "ScreenContentRealTime" => Ok(openh264::encoder::UsageType::ScreenContentRealTime),
        "CameraVideoNonRealTime" => Ok(openh264::encoder::UsageType::CameraVideoNonRealTime),
        "ScreenContentNonRealTime" => Ok(openh264::encoder::UsageType::ScreenContentNonRealTime),
        "InputContentTypeAll" => Ok(openh264::encoder::UsageType::InputContentTypeAll),
        _ => Err("Invalid usage type"),
    }
}

fn parse_rate_control_mode(
    rate_control_mode: &str,
) -> Result<openh264::encoder::RateControlMode, &'static str> {
    match rate_control_mode {
        "Quality" => Ok(openh264::encoder::RateControlMode::Quality),
        "Bitrate" => Ok(openh264::encoder::RateControlMode::Bitrate),
        "Bufferbased" => Ok(openh264::encoder::RateControlMode::Bufferbased),
        "Timestamp" => Ok(openh264::encoder::RateControlMode::Timestamp),
        "Off" => Ok(openh264::encoder::RateControlMode::Off),
        _ => Err("Invalid rate control mode"),
    }
}

fn parse_sps_pps_strategy(
    sps_pps_strategy: &str,
) -> Result<openh264::encoder::SpsPpsStrategy, &'static str> {
    match sps_pps_strategy {
        "ConstantId" => Ok(openh264::encoder::SpsPpsStrategy::ConstantId),
        "IncreasingId" => Ok(openh264::encoder::SpsPpsStrategy::IncreasingId),
        "SpsListing" => Ok(openh264::encoder::SpsPpsStrategy::SpsListing),
        "SpsListingAndPpsIncreasing" => {
            Ok(openh264::encoder::SpsPpsStrategy::SpsListingAndPpsIncreasing)
        }
        "SpsPpsListing" => Ok(openh264::encoder::SpsPpsStrategy::SpsPpsListing),
        _ => Err("Invalid SPS/PPS strategy"),
    }
}

/// An H264 encoder which converts RGB frames to bytes.
/// /// Configures an [H264Encoder].
///
/// # Options
/// * `bitrate_bps` - The target bitrate in bits per second.
/// * `enable_skip_frame` - Whether to allow the encoder to skip frames to conserve bitrate.
/// * `max_frame_rate` - The maximum frame rate in frames per second.
/// * `num_threads` - The number of threads to use for encoding. 0 automatically selects the number
///   of threads.
/// * `usage_type` - The type of video content to encode. Options are "CameraVideoRealTime",
///   "ScreenContentRealTime", "CameraVideoNonRealTime", "ScreenContentNonRealTime", and
///   "InputContentTypeAll".
/// * `rate_control_mode` - The rate control mode to use. Options are "Quality", "Bitrate",
///   "Bufferbased", "Timestamp", and "Off".
/// * `sps_pps_strategy` - The SPS/PPS strategy to use. Options are "ConstantId", "IncreasingId",
///   "SpsListing", "SpsListingAndPpsIncreasing", and "SpsPpsListing".
/// * `debug` - Whether to enable console debug logging in OpenH264.
#[pyclass]
pub struct H264Encoder {
    encoder: Encoder,
}

#[pymethods]
impl H264Encoder {
    #[new]
    #[pyo3(signature = (bitrate_bps=120_000, enable_skip_frame=true, max_frame_rate=0.0, num_threads=0, usage_type="CameraVideoRealTime", rate_control_mode="Quality", sps_pps_strategy="ConstantId", debug=false))]
    fn new(
        bitrate_bps: u32,
        enable_skip_frame: bool,
        max_frame_rate: f32,
        num_threads: u16,
        usage_type: &str,
        rate_control_mode: &str,
        sps_pps_strategy: &str,
        debug: bool,
    ) -> PyResult<Self> {
        // Construct the config.
        let usage_type_enum = parse_usage_type(usage_type).map_err(PyValueError::new_err)?;
        let rate_control_mode_enum =
            parse_rate_control_mode(rate_control_mode).map_err(PyValueError::new_err)?;
        let sps_pps_strategy_enum =
            parse_sps_pps_strategy(sps_pps_strategy).map_err(PyValueError::new_err)?;
        let config = EncoderConfig::new()
            .set_bitrate_bps(bitrate_bps)
            .enable_skip_frame(enable_skip_frame)
            .max_frame_rate(max_frame_rate)
            .set_multiple_thread_idc(num_threads)
            .usage_type(usage_type_enum)
            .rate_control_mode(rate_control_mode_enum)
            .sps_pps_strategy(sps_pps_strategy_enum)
            .debug(debug);

        // Initialize the encoder.
        let encoder = Encoder::with_api_config(openh264::OpenH264API::from_source(), config)
            .map_err(|e| PyRuntimeError::new_err(format!("Error initializing encoder: {:?}", e)))?;
        Ok(H264Encoder { encoder })
    }

    /// Encode an RGB frame to bytes.
    /// H264 may skip frames to conserve bitrate, in which case an empty byte array is returned.
    #[pyo3(signature = (frame, timestamp=0, force_iframe=false))]
    fn encode(
        &mut self,
        frame: PyReadonlyArray3<u8>,
        timestamp: u64,
        force_iframe: bool,
    ) -> PyResult<Vec<u8>> {
        let rgb_data = frame.as_slice()?;
        let shape = frame.shape();
        let rgb_slice = RgbSliceU8::new(rgb_data, (shape[0], shape[1]));
        let yuv_buffer = YUVBuffer::from_rgb8_source(rgb_slice);

        if force_iframe {
            self.encoder.force_intra_frame();
        }

        let timestamp_internal = Timestamp::from_millis(timestamp);
        let bitstream = self
            .encoder
            .encode_at(&yuv_buffer, timestamp_internal)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error encoding: {:?}",
                    e
                ))
            })?;
        Ok(bitstream.to_vec())
    }
}
