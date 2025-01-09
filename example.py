import cv2
import fire

from frame_by_frame_h264 import H264Encoder, H264Decoder


def main(
    filename: str,
    bitrate_bps: int = 100_000,
    enable_skip_frame: bool = False,
    max_frame_rate: int = 30,
    num_encoder_threads: int = 0,
    usage_type: str = "CameraVideoRealTime",
    rate_control_mode: str = "Quality",
    sps_pps_strategy: str = "ConstantId",
    flush_after_decode: str = "Flush",
    num_decoder_threads: int = 0,
    debug=False,
    force_iframe=False,
):
    """Compresses a video file with H264 and displays the compressed video."""
    encoder = H264Encoder(
        bitrate_bps=bitrate_bps,
        enable_skip_frame=enable_skip_frame,
        max_frame_rate=max_frame_rate,
        num_threads=num_encoder_threads,
        usage_type=usage_type,
        rate_control_mode=rate_control_mode,
        sps_pps_strategy=sps_pps_strategy,
        debug=debug,
    )

    decoder = H264Decoder(
        flush_after_decode=flush_after_decode,
        num_threads=num_decoder_threads,
        debug=debug,
    )

    cap = cv2.VideoCapture(filename)
    fps = cap.get(cv2.CAP_PROP_FPS)
    i = 0
    while cap.isOpened():
        ret, frame = cap.read()

        if not ret:
            break

        timestamp = int(i * 1e3 / fps)
        encoded_frame = encoder.encode(
            frame, timestamp=timestamp, force_iframe=force_iframe
        )
        encoded_size = len(encoded_frame)
        print(f"Frame {i + 1}: {encoded_size} B")

        decoded_frame = decoder.decode(encoded_frame)

        cv2.imshow(f"H264: {filename}", decoded_frame)

        if cv2.waitKey(25) & 0xFF == ord("q"):
            break


if __name__ == "__main__":
    fire.Fire(main)
