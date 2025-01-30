import numpy as np
import pytest
from frame_by_frame_h264 import H264Encoder


def test_encode():
    encoder = H264Encoder(
        bitrate_bps=100_000,
        enable_skip_frame=False,
        max_frame_rate=30,
        num_threads=0,
        usage_type="CameraVideoRealTime",
        rate_control_mode="Quality",
        sps_pps_strategy="ConstantId",
        debug=False,
    )

    frame = np.random.randint(0, 255, (1080, 1920, 3), dtype=np.uint8)
    timestamp = 0

    encoded = encoder.encode(frame, timestamp=timestamp)

    assert isinstance(encoded, bytes)
    assert len(encoded) > 0


def test_encode_force_iframe():
    encoder = H264Encoder(
        bitrate_bps=100_000,
        enable_skip_frame=False,
        max_frame_rate=30,
        num_threads=0,
        usage_type="CameraVideoRealTime",
        rate_control_mode="Quality",
        sps_pps_strategy="ConstantId",
        debug=False,
    )

    for i in range(10):
        frame = np.random.randint(0, 255, (1080, 1920, 3), dtype=np.uint8)
        t = int(i * 1e3 / 30)
        encoded = encoder.encode(frame, timestamp=t, force_iframe=True)
        assert len(encoded) > 0


def test_bitrate():
    bitrate = 100_000
    encoder = H264Encoder(
        bitrate_bps=bitrate,
        enable_skip_frame=False,
        max_frame_rate=30,
        num_threads=0,
        usage_type="CameraVideoRealTime",
        rate_control_mode="Quality",
        sps_pps_strategy="ConstantId",
        debug=False,
    )

    # Need to encoded a frame to fully initialize the encoder.
    with pytest.raises(RuntimeError):
        encoder.bitrate_bps = 10_000

    frame = np.random.randint(0, 255, (1080, 1920, 3), dtype=np.uint8)
    encoder.encode(frame, timestamp=0)
    assert encoder.bitrate_bps == bitrate

    encoder.bitrate_bps = 10_000
    assert encoder.bitrate_bps == 10_000
