// No first-person hand (or held item) while zooming.
fn optizoom_hand(canceled: &mut bool) {
    if crate::game::optizoom::hides_hand() {
        *canceled = true;
    }
}
