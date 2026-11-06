// Every real key event: the zoom key coming up over a screen ends the hold too
// (the `key` hook hears only play), as OptiFine asks the keyboard each frame.
fn optizoom_release(game: &crate::game::Game<'_>, event: &winit::event::KeyEvent) {
    use crate::game::optizoom as zoom;
    if event.state != winit::event::ElementState::Released {
        return;
    }
    let winit::keyboard::PhysicalKey::Code(code) = event.physical_key else {
        return;
    };
    if keybinds::key_name(code)
        .is_some_and(|name| name == game.native_mod_key(zoom::ID, zoom::KEY, zoom::DEFAULT_KEY))
    {
        zoom::release();
    }
}
