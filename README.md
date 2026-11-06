# OptiZoom (Horizon native mod)

OptiFine's zoom for [Horizon](https://github.com/horizon-rust/horizon) (Minecraft: Java Edition 26.3 in Rust).

- **Hold C** (as OptiFine; rebind in Options > Mods > OptiZoom): the field of view drops to a
  quarter at once, no easing.
- The **cinematic camera** is on while the zoom lasts, then back to what it was (OptiFine's
  `zoomSmoothCamera`). Mouse sensitivity is not changed at OptiFine's 4x.
- No zoom while a screen is open; closing it with the key still down zooms again.
- **Hand hidden** while zoomed (with the held item).
- **Mouse wheel** while holding: zoom further in or out (x1.25 a notch, 1x to 10x, Horizon's cap).
  Beyond 4x turning slows in step so it aims like 4x. Each new press starts at 4x. The hotbar keeps
  the wheel otherwise.

## Build and install

```sh
python tools/package.py --install   # target/optizoom-1.0.1.hnmod, copied into horizon-native-mods/
rustc --edition 2024 --test mixins/optizoom/runtime.rs -o optizoom_test && ./optizoom_test
```

Pinned to Horizon `66f3981` (Nightly 2026-11-05): the cinematic camera is Game's private
`smooth_camera` field (`camera.rs`), which no hook point offers. Everything else uses hook points
(`key`, `input_key`, `frame`, `wheel`, `zoom`, `render_hand`, `mods`).

MIT.
