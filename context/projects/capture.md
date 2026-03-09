# Capture

## Headline

Cross-platform security camera — grabs all input devices, snaps intruder photos on any interaction.

## Category

Learning Project — Systems Programming

## What It Is

A physical security tool that grabs all keyboard and mouse input (making the desktop unresponsive), then captures a timestamped webcam photo whenever someone tries to interact. Press a secret key to unlock. Works on macOS and Linux with completely different I/O strategies per platform.

## What It Proves

- Cross-platform systems programming with conditional compilation (cfg(target_os))
- Platform-specific dependencies in Cargo.toml via [target.'cfg(...)'.dependencies]
- Linux: raw evdev device enumeration, capability-based filtering (keyboard vs mouse heuristics), nix::poll for multiplexed I/O across multiple grabbed devices, proper ungrab lifecycle
- macOS: rdev callback-based grab via Accessibility API
- Real bug discovery and workaround: rdev on Linux grabs ALL evdev devices (including Bluetooth/network controllers, causing disconnects) — dropped down to raw evdev with selective grabbing
- Interior mutability pattern: Rc<Mutex<CaptureState>> for sharing state across event callbacks
- Debounce logic: max 1 capture per second using jiff timestamps

## Key Technical Highlights

### Linux Device Identification
```rust
trait Identify for Device {
    is_probably_keyboard() → EV_REPEAT + KEY_A + KEY_ENTER + KEY_SPACE
    is_probably_mouse()    → REL_X + REL_Y relative axes
}
```
Capability-based heuristics instead of name matching — works with any hardware.

### Platform Divergence
| Concern | macOS | Linux |
|---------|-------|-------|
| Grab mechanism | rdev::grab (Accessibility API) | evdev device.grab() per device |
| Event loop | Callback-based (returns None to swallow) | nix::poll across file descriptors |
| Permissions | Accessibility API approval | input group membership |
| Shutdown | process::exit(0) (no clean stop API) | device.ungrab() on all devices |

## What I Learned

- How input devices work at the OS level (evdev on Linux, CGEventTap on macOS)
- poll() for multiplexed I/O — waiting on multiple file descriptors simultaneously
- Why you can't just grab "all devices" — Bluetooth controllers, network adapters, and power buttons are also evdev devices
- The difference between rdev's abstraction and raw evdev — sometimes you need to drop down a level
- Conditional compilation as a strategy for cross-platform Rust

## Status

Working on both platforms. Next: configurable secret key sequence, configurable debounce interval.

## Repo

~/Work/capture
