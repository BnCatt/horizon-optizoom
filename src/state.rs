// SPDX-License-Identifier: MIT
// The zoom's state: hooks and the camera mixin all run on the window's thread.
use std::cell::RefCell;

/// OptiFine divides the field of view by 4.
pub const START: f32 = 4.0;
/// The wheel's limits: no zoom, and Horizon's maximum.
const MIN: f32 = 1.0;
const MAX: f32 = 10.0;
/// Each wheel step multiplies (up) or divides (down) the zoom by this.
const STEP: f32 = 1.25;

#[derive(Clone, Copy)]
struct State {
    /// The key is down, pressed while playing and not yet released.
    held: bool,
    /// The key was down last frame: a press is the frame it goes down.
    was_down: bool,
    /// OptiFine's `Config.zoomMode`: zooming this frame.
    active: bool,
    /// The cinematic camera is on for the zoom (the mixin applied the start).
    camera_on: bool,
    /// The zoom: START from each press, moved by the wheel.
    factor: f32,
    /// OptiFine's `Config.zoomSmoothCamera`: the cinematic camera before the zoom.
    saved_smooth: bool,
}

const IDLE: State = State {
    held: false,
    was_down: false,
    active: false,
    camera_on: false,
    factor: START,
    saved_smooth: false,
};

thread_local! {
    static STATE: RefCell<State> = const { RefCell::new(IDLE) };
}

/// Once a frame: `down` the zoom key now, `playing` no screen open. A press
/// counts only while playing (OptiFine's key does nothing over a screen); a
/// release counts anywhere.
pub fn frame(down: bool, playing: bool) {
    STATE.with_borrow_mut(|s| {
        if !down {
            s.held = false;
        } else if !s.was_down && playing {
            s.held = true;
            s.factor = START;
        }
        s.was_down = down;
        s.active = s.held && playing;
    });
}

/// The cinematic camera to set this frame: on when the zoom started (keeping
/// `smooth`, what it was), back to that when it ended, else none.
pub fn camera(smooth: bool) -> Option<bool> {
    STATE.with_borrow_mut(|s| {
        if s.active && !s.camera_on {
            s.camera_on = true;
            s.saved_smooth = smooth;
            Some(true)
        } else if !s.active && s.camera_on {
            s.camera_on = false;
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
/// Instant, as OptiFine's. At 4x turning is unchanged (the cinematic camera
/// calms it); zoomed further with the wheel it slows in step.
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
        frame(true, true);
        assert_eq!(camera(false), Some(true), "cinematic on");
        assert_eq!(view(), Some((4.0, false, 1.0)));
        assert!(hides_hand());
        frame(true, true);
        assert_eq!(camera(true), None, "nothing more while held");
        frame(false, true);
        assert_eq!(camera(true), Some(false), "back to off");
        assert_eq!(view(), None);
        assert!(!hides_hand());
    }

    #[test]
    fn a_cinematic_camera_that_was_on_stays_on() {
        reset();
        frame(true, true);
        assert_eq!(camera(true), Some(true));
        frame(false, true);
        assert_eq!(camera(true), Some(true));
    }

    #[test]
    fn a_screen_ends_the_zoom_and_closing_it_brings_it_back() {
        reset();
        frame(true, true);
        assert_eq!(camera(false), Some(true));
        frame(true, false);
        assert_eq!(camera(true), Some(false), "screen open: restored");
        assert_eq!(view(), None);
        frame(true, true);
        assert_eq!(camera(false), Some(true), "still held: zooms again");
        frame(false, false);
        assert_eq!(camera(true), Some(false));
    }

    #[test]
    fn a_press_over_a_screen_does_nothing() {
        reset();
        frame(true, false);
        frame(true, true);
        assert_eq!(camera(false), None);
        assert_eq!(view(), None);
    }

    #[test]
    fn the_wheel_zooms_only_while_zooming_within_one_and_ten() {
        reset();
        assert!(!wheel(1.0), "the hotbar's while not zooming");
        frame(true, true);
        assert!(wheel(1.0));
        let (f, _, turn) = view().unwrap();
        assert!(close(f, 5.0));
        assert!(close(turn, 0.8));
        assert!(wheel(-2.0));
        assert!(close(view().unwrap().0, 3.2));
        assert!(wheel(100.0));
        assert_eq!(view().unwrap().0, 10.0);
        assert!(wheel(-100.0));
        assert_eq!(view().unwrap().0, 1.0);
        frame(false, true);
        frame(true, true);
        assert_eq!(view().unwrap().0, 4.0, "a new press starts over at 4x");
    }
}
