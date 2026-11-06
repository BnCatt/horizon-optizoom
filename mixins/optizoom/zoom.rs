// The zoom in effect, unless another mod's came first.
fn optizoom_view(zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    if zoom.is_none()
        && let Some((factor, smooth, turn)) = crate::game::optizoom::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom {
            factor,
            smooth,
            turn,
        });
    }
}
