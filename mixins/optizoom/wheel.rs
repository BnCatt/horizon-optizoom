// While zooming, the wheel zooms instead of moving the hotbar.
fn optizoom_wheel(steps: f32, claimed: &mut bool) {
    if !*claimed && crate::game::optizoom::wheel(steps) {
        *claimed = true;
    }
}
