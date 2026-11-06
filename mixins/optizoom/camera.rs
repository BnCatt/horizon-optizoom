// The cinematic camera (`Options.smoothCamera`), which OptiFine's zoom turns on
// while it lasts. Game's private `smooth_camera` field: the mod's one reach
// inside Horizon, kept here so a Nightly that renames it breaks only this file.
pub(super) fn smooth(game: &super::Game<'_>) -> bool {
    game.smooth_camera
}

pub(super) fn set_smooth(game: &mut super::Game<'_>, on: bool) {
    game.smooth_camera = on;
}
