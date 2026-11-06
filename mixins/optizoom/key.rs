// The zoom key went down or up while playing: the player's choice of key
// (`key.optizoom.zoom` in horizon-mods.txt), C by default.
fn optizoom_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::optizoom as zoom;
    if !game.native_on(zoom::ID) {
        return;
    }
    if key != game.native_mod_key(zoom::ID, zoom::KEY, zoom::DEFAULT_KEY) {
        return;
    }
    zoom::key(pressed, game.native_playing());
}
