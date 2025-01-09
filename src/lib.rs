use pyo3::prelude::*;

mod encoder;
use encoder::H264Encoder;

mod decoder;
use decoder::H264Decoder;

/// A Python module implemented in Rust.
#[pymodule]
fn frame_by_frame_h264(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<H264Encoder>()?;

    m.add_class::<H264Decoder>()?;
    Ok(())
}
