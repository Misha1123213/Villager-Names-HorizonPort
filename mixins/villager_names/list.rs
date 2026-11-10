// The mod's row in Options > Mods.
fn villager_names_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::villager_names as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Villager Names {}", m::VERSION),
        description: "Gives every villager a name. Port of Serilum's mod.".into(),
        keys: Vec::new(),
    });
}
