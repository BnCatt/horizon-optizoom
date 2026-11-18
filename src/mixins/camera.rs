fn optizoom_camera(game: &mut Game<'_>) {
    if let Some(on) = mod_optizoom::camera(game.smooth_camera) {
        game.smooth_camera = on;
    }
}
