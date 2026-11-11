fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "portage_simple_durability_tooltip".into(),
        title: "portage-simple-durability-tooltip 0.1.1".into(),
        description: "Community Rust port of Infinidoge's mod: durability in item tooltips.".into(),
        keys: Vec::new(),
    });
}
