# Gotcha

## Headline

Cross-platform security camera: grabs all input devices, snaps intruder photos on any interaction.

## Category

Learning Project: Systems Programming

## What It Is

A physical security tool that grabs all keyboard and mouse input (making the desktop unresponsive), then captures a timestamped webcam photo whenever someone tries to interact. Type the secret to unlock: `test`, or whatever `--password` names. Works on macOS and Linux with completely different I/O strategies per platform.

## What It Proves

- Cross-platform systems programming with conditional compilation (cfg(target_os))
- Platform-specific dependencies in Cargo.toml via [target.'cfg(...)'.dependencies]
- Linux: raw evdev device enumeration, capability-based filtering (keyboard vs mouse heuristics), nix::poll for multiplexed I/O across multiple grabbed devices, proper ungrab lifecycle
- macOS: a Core Graphics event tap at the HID level, on its own thread and run loop, answering `Drop` for every keyboard and mouse event (needs Accessibility approval)
- Real bug discovery and workaround: rdev on Linux grabs ALL evdev devices (including Bluetooth/network controllers, causing disconnects), so dropped down to raw evdev with selective grabbing
- Threads with a channel: the tap only sends a message per input, and the camera shoots on the main thread, so a slow frame never stalls input
- Fixing a dependency: rdev's macOS grab stopped blocking input (it rewrote the event's type instead of returning null), and the camera bindings crashed on cameras with several formats at one resolution, fixed in a vendored copy
- Debounce logic: max 1 capture per second, a tested `Shutter` timed with `std::time::Instant`

## Key Technical Highlights

### Linux Device Identification
```rust
trait Identify for Device {
    is_probably_keyboard() → EV_REPEAT + KEY_A + KEY_ENTER + KEY_SPACE
    is_probably_mouse()    → REL_X + REL_Y relative axes
}
```
Capability-based heuristics instead of name matching. Works with any hardware.

### Platform Divergence
| Concern | macOS | Linux |
|---------|-------|-------|
| Grab mechanism | CGEventTap answering Drop (Accessibility API) | evdev device.grab() per device |
| Event loop | CFRunLoop on the tap's own thread | nix::poll across file descriptors |
| Permissions | Accessibility API approval | input group membership |
| Shutdown | stop the run loop | device.ungrab() on all devices |

## What I Learned

- How input devices work at the OS level (evdev on Linux, CGEventTap on macOS)
- poll() for multiplexed I/O: waiting on multiple file descriptors simultaneously
- Why you can't just grab "all devices": Bluetooth controllers, network adapters, and power buttons are also evdev devices
- The difference between rdev's abstraction and raw evdev: sometimes you need to drop down a level
- Conditional compilation as a strategy for cross-platform Rust

## Status

Working on both platforms, tested on two Macs. The secret is set with `--password` or `--pw`. Next: configurable debounce interval.

## Repo

~/Developer/gotcha
