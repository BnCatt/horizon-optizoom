// SPDX-License-Identifier: MIT
// OptiZoom: OptiFine's zoom for Horizon. Hold the key (C, as OptiFine) and the
// field of view drops to a quarter at once; the cinematic camera is on while
// the zoom lasts and goes back to what it was on release; no zoom while a
// screen is open. Additions: the first-person hand is hidden while zoomed, and
// the mouse wheel zooms further in or out while the key is held (the hotbar
// keeps the wheel otherwise).
//
// Everything but the cinematic camera goes through mod_api hooks; the camera is
// one native mixin (src/mixins/camera.rs) that calls `camera` below.
use mod_api::{
    self, ClientHost, Flow, Hooks, KeyConflictContext, KeyRegistration,
    MouseScrollingEvent, Registrar, RegistrationContext, RenderFramePreEvent, RenderHandEvent,
    Zoom, ZoomEvent,
};

mod state;

/// The mod's id, and its key's id (`key.optizoom.zoom` in `horizon-mods.txt`).
pub const ID: &str = "optizoom";
pub const KEY: &str = "zoom";
/// OptiFine's key.
pub const DEFAULT_KEY: &str = "key.keyboard.c";

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The zoom key on Options > Controls, C by default.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        // Universal: the key's release is seen over a screen too.
        r.register_key_mapping(
            KeyRegistration::new(KEY, DEFAULT_KEY)
                .label("key.optizoom.zoom")
                .context(KeyConflictContext::Universal),
        );
    }

    /// Once a frame (OptiFine checks in `getFOVModifier`): the key and whether
    /// the player is playing (no screen over the world).
    fn render_frame_pre(&self, host: &mut dyn ClientHost, _: &mut RenderFramePreEvent) {
        state::frame(host.is_down(ID, KEY), host.playing());
    }

    /// The zoom in effect, unless another mod's came first.
    fn zoom(&self, _: &dyn ClientHost, _: &mut ZoomEvent) -> Option<Zoom> {
        state::view().map(|(factor, smooth, turn)| Zoom { factor, smooth, turn })
    }

    /// No first-person hand (or held item) while zooming.
    fn render_hand(&self, _: &dyn ClientHost, _: &mut RenderHandEvent<'_, '_>) -> Flow {
        if state::hides_hand() { Flow::Cancel } else { Flow::Continue }
    }

    /// While zooming, the wheel zooms instead of moving the hotbar.
    fn mouse_scrolling(&self, _: &mut dyn ClientHost, event: &mut MouseScrollingEvent) -> Flow {
        let steps = if event.accumulated[1] != 0 {
            event.accumulated[1] as f32
        } else {
            event.scroll[1] as f32
        };
        if state::wheel(steps) { Flow::Cancel } else { Flow::Continue }
    }
}

/// For the camera mixin, once a frame after the hooks: the cinematic camera to
/// set (on when the zoom starts, back to `smooth` as it was when it ends).
pub fn camera(smooth: bool) -> Option<bool> {
    state::camera(smooth)
}
