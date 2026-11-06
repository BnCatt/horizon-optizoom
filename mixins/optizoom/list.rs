// OptiZoom's row in Options > Mods, with its key.
fn optizoom_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::optizoom as zoom;
    mods.push(crate::game::native_hooks::NativeMod {
        id: zoom::ID.into(),
        title: format!("OptiZoom {}", zoom::VERSION),
        description: zoom::DESCRIPTION.into(),
        keys: vec![[zoom::KEY.into(), "Zoom".into(), zoom::DEFAULT_KEY.into()]],
    });
}
