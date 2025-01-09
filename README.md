# Frame by Frame H264

A Python tool to encode and decode videos frame-by-frame using H264.

Under the hood, this tools uses [OpenH264](https://github.com/cisco/openh264).


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