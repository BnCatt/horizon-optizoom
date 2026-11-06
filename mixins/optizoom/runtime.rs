// SPDX-License-Identifier: MIT
// OptiZoom: OptiFine's zoom for Horizon. Hold the key (C, as OptiFine) and the
// field of view drops to a quarter at once; the cinematic camera is on while
// the zoom lasts and goes back to what it was on release; no zoom while a
// screen is open. Additions: the first-person hand is hidden while zoomed, and
// the mouse wheel zooms further in or out while the key is held (the hotbar
// keeps the wheel otherwise). This file is the state only: the module the
// bundle adds to the game, and testable on its own (`rustc --test`).
use std::cell::RefCell;

/// The mod's id: its key's line (`key.optizoom.zoom` in `horizon-mods.txt`).
pub const ID: &str = "optizoom";
pub const VERSION: &str = "1.0.1";
pub const DESCRIPTION: &str = "OptiFine's zoom: hold C for a quarter of the field of view with \
the cinematic camera, the hand hidden. Scroll while holding to zoom further in or out.";
/// The key's name, and its default (OptiFine's).
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.c";
/// OptiFine divides the field of view by 4.
pub const START: f32 = 4.0;
/// The wheel's limits: no zoom, and Horizon's maximum.
const MIN: f32 = 1.0;
const MAX: f32 = 10.0;
/// Each wheel step multiplies (up) or divides (down) the zoom by this.
const STEP: f32 = 1.25;

#[derive(Clone, Copy)]
struct State {
    /// The key is down (pressed while playing, not yet released).
    held: bool,
    /// OptiFine's `Config.zoomMode`: zooming in this frame.
    active: bool,
    /// The zoom: START from each press, moved by the wheel.
    factor: f32,
    /// OptiFine's `Config.zoomSmoothCamera`: the cinematic camera before the zoom.
    saved_smooth: bool,
}

const IDLE: State = State {
    held: false,
    active: false,
    factor: START,
    saved_smooth: false,
};

thread_local! {
    static STATE: RefCell<State> = const { RefCell::new(IDLE) };
}

/// The zoom key went down (counts only while playing, as OptiFine's key does
/// nothing over a screen) or up.
pub fn key(pressed: bool, playing: bool) {
    STATE.with_borrow_mut(|s| {
        if !pressed {
            s.held = false;
        } else if playing && !s.held {
            s.held = true;
            s.factor = START;
        }
    });
}

/// The zoom key came up anywhere (over a screen too, where `key` hears nothing).
pub fn release() {
    STATE.with_borrow_mut(|s| s.held = false);
}

/// Once a frame (OptiFine checks in `getFOVModifier`): `playing` is no screen
/// open and the mod on, `smooth` the cinematic camera now. Returns the
/// cinematic camera to set when the zoom starts (on) or ends (as it was).
pub fn frame(playing: bool, smooth: bool) -> Option<bool> {
    STATE.with_borrow_mut(|s| {
        let zoom = s.held && playing;
        if zoom && !s.active {
            s.active = true;
            s.saved_smooth = smooth;
            Some(true)
        } else if !zoom && s.active {
            s.active = false;
            Some(s.saved_smooth)
        } else {
            None
        }
    })
}

/// Wheel steps (positive is up): true when zooming and they zoom.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        if !s.active {
            return false;
        }
        s.factor = (s.factor * STEP.powf(steps)).clamp(MIN, MAX);
        true
    })
}

/// The zoom in effect: (factor, eased, turning scale), none when not zooming.
/// Instant, as OptiFine's. At OptiFine's 4x turning is unchanged (the
/// cinematic camera is what calms it); zoomed further with the wheel it slows
/// in step, so that 10x aims like 4x does.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        s.active
            .then(|| (s.factor, false, if s.factor > START { START / s.factor } else { 1.0 }))
    })
}

/// The first-person hand is hidden while zooming.
pub fn hides_hand() -> bool {
    STATE.with_borrow(|s| s.active)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reset() {
        STATE.with_borrow_mut(|s| *s = IDLE);
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn hold_zooms_a_quarter_with_the_cinematic_camera_and_restores_it() {
        reset();
        key(true, true);
        assert_eq!(frame(true, false), Some(true), "cinematic on");
        assert_eq!(view(), Some((4.0, false, 1.0)));
        assert!(hides_hand());
        assert_eq!(frame(true, true), None, "nothing more while held");
        key(false, true);
        assert_eq!(frame(true, true), Some(false), "back to off");
        assert_eq!(view(), None);
        assert!(!hides_hand());
    }

    #[test]
    fn a_cinematic_camera_that_was_on_stays_on() {
        reset();
        key(true, true);
        assert_eq!(frame(true, true), Some(true));
        release();
        assert_eq!(frame(true, true), Some(true));
    }

    #[test]
    fn a_screen_ends_the_zoom_and_closing_it_brings_it_back() {
        reset();
        key(true, true);
        assert_eq!(frame(true, false), Some(true));
        assert_eq!(frame(false, true), Some(false), "screen open: restored");
        assert_eq!(view(), None);
        assert_eq!(frame(true, false), Some(true), "still held: zooms again");
        release();
        assert_eq!(frame(false, true), Some(false));
    }

    #[test]
    fn a_press_over_a_screen_does_nothing() {
        reset();
        key(true, false);
        assert_eq!(frame(true, false), None);
        assert_eq!(view(), None);
    }

    #[test]
    fn the_wheel_zooms_only_while_zooming_within_one_and_ten() {
        reset();
        assert!(!wheel(1.0), "the hotbar's while not zooming");
        key(true, true);
        frame(true, false);
        assert!(wheel(1.0));
        let (f, _, turn) = view().unwrap();
        assert!(close(f, 5.0));
        assert!(close(turn, 0.8));
        assert!(wheel(-2.0));
        assert!(close(view().unwrap().0, 3.2));
        assert!(close(view().unwrap().2, 1.0));
        assert!(wheel(100.0));
        assert_eq!(view().unwrap().0, 10.0);
        assert!(wheel(-100.0));
        assert_eq!(view().unwrap().0, 1.0);
        // A new press starts over at 4x.
        key(false, true);
        frame(true, false);
        key(true, true);
        frame(true, false);
        assert_eq!(view().unwrap().0, 4.0);
    }
}
