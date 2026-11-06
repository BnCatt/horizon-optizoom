// Once a frame: start or end the zoom (no screen open, mod on) and turn the
// cinematic camera on for it, or back to what it was.
fn optizoom_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::{optizoom as zoom, optizoom_camera as camera};
    let playing = game.native_playing() && game.native_on(zoom::ID);
    if let Some(on) = zoom::frame(playing, camera::smooth(game)) {
        camera::set_smooth(game, on);
    }
}
