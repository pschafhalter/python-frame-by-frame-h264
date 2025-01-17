# Frame by Frame H264

An interface to encode and decode videos frame-by-frame using H264.

Under the hood, this tool relies on [Rust bindings](https://github.com/ralfbiedert/openh264-rs) for [OpenH264](https://github.com/cisco/openh264).


## Encoding a Video

```python
encoder = H264Encoder()

for frame in video:
    encoded_frame = encoder.encode(frame)
```

## Decoding a Video

```python
decoder = H264Decoder()

for bytes_array in packets:
    decoded_frame = decoder.decode(bytes_array)
```

## Example

The provided example encodes and decodes a video file, displays the output, and prints the size of each encoded frame.

```bash
python3 example.py $filename
```
