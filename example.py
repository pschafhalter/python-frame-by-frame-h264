import argparse

import cv2
from frame_by_frame_h264 import H264Decoder, H264Encoder


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
    """Compresses a video file with H.264 and displays the compressed video."""
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

        if decoded_frame is not None:
            cv2.imshow(f"H.264: {filename}", decoded_frame)

        if cv2.waitKey(25) & 0xFF == ord("q"):
            break
        i += 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Compresses a video file with H.264 and displays the compressed video."
    )
    parser.add_argument("filename", type=str, help="Path to the video file")
    parser.add_argument(
        "--bitrate_bps", type=int, default=100_000, help="Bitrate in bits per second"
    )
    parser.add_argument(
        "--enable_skip_frame",
        action="store_true",
        help="Enables the encoder to skip frames to maintain bitrate",
    )
    parser.add_argument(
        "--max_frame_rate", type=int, default=30, help="Maximum frame rate"
    )
    parser.add_argument(
        "--num_encoder_threads", type=int, default=0, help="Number of encoder threads"
    )
    parser.add_argument(
        "--usage_type", type=str, default="CameraVideoRealTime", help="Usage type"
    )
    parser.add_argument(
        "--rate_control_mode", type=str, default="Quality", help="Rate control mode"
    )
    parser.add_argument(
        "--sps_pps_strategy", type=str, default="ConstantId", help="SPS PPS strategy"
    )
    parser.add_argument(
        "--flush_after_decode", type=str, default="Flush", help="Flush after decode"
    )
    parser.add_argument(
        "--num_decoder_threads", type=int, default=0, help="Number of decoder threads"
    )
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    parser.add_argument(
        "--force_iframe",
        action="store_true",
        help="Force each frame to be compressed as an I-frame",
    )

    args = parser.parse_args()

    main(
        filename=args.filename,
        bitrate_bps=args.bitrate_bps,
        enable_skip_frame=args.enable_skip_frame,
        max_frame_rate=args.max_frame_rate,
        num_encoder_threads=args.num_encoder_threads,
        usage_type=args.usage_type,
        rate_control_mode=args.rate_control_mode,
        sps_pps_strategy=args.sps_pps_strategy,
        flush_after_decode=args.flush_after_decode,
        num_decoder_threads=args.num_decoder_threads,
        debug=args.debug,
        force_iframe=args.force_iframe,
    )
