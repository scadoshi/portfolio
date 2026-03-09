# Upsee

## Headline

Real-time pullup counter using on-device pose estimation — webcam + MoveNet + tract in Rust.

## Category

Learning Project — ML Inference / Computer Vision

## What It Is

A real-time rep counter that uses a webcam and MoveNet pose estimation model (via tract ONNX runtime) to detect and count pullup repetitions. Runs entirely on-device with no cloud inference. Targeted at Raspberry Pi deployment.

## What It Proves

- End-to-end ML inference pipeline in Rust: frame capture → preprocessing → inference → postprocessing → state machine
- Tensor manipulation: reshaping webcam frames into [1, 3, 192, 192] NCHW tensors, normalizing pixel values to 0-1
- Keypoint extraction from model output: [1, 1, 17, 3] tensor → shoulder/wrist y-coordinates
- Hysteresis-based state machine: separate UP and DOWN thresholds prevent noise-induced false counts
- Confidence filtering: skip frames where average keypoint confidence < 0.4
- Trait-based image preprocessing: Square trait for center-cropping arbitrary aspect ratios

## Key Technical Highlights

### Pipeline
```
[Webcam] → nokhwa (highest framerate)
    ↓
[Square crop] → center-crop to square via Square trait
    ↓
[Resize + normalize] → 192x192, pixel values / 255.0
    ↓
[MoveNet inference] → tract ONNX runtime
    ↓
[Keypoint extraction] → shoulders (5,6) and wrists (9,10)
    ↓
[State machine] → DOWN (diff > 0.15) ↔ UP (diff < 0.05)
    ↓
[Rep counter] → increment on DOWN → UP transition
```

### Hysteresis
Two separate thresholds prevent oscillation at the boundary:
- Transition to UP: shoulder-wrist diff < 0.05 (arms pulled up high)
- Transition to DOWN: shoulder-wrist diff > 0.15 (arms extended)
- Gap between thresholds = dead zone that absorbs noise

## What I Learned

- How ONNX models work as a portable inference format
- Tensor shapes and NCHW format for image models
- tract as a Rust-native alternative to Python inference runtimes
- Hysteresis as a signal processing concept for noisy real-time data
- Camera warmup frames — first ~30 frames have unstable exposure/white balance

## Status

Working proof of concept. Next: threshold tuning with more data, temporal smoothing, Raspberry Pi testing, multi-threaded capture + inference.

## Repo

~/Work/upsee
