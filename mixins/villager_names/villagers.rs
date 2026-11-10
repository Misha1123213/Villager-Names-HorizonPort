// Adapter between the mod and Horizon's entity API — the only file here that
// touches game internals. Entity handles and entity NBT follow the native-mod
// API at the pinned Horizon revision.

use horizon_world::nbt::{Compound, Tag};

#[derive(Clone, Copy)]
pub struct Villager {
    pub network_id: u32,
}

/// Whether a type is a vanilla villager or wandering trader. With
/// `include_modded`, also accept mod namespaces whose type id identifies a
/// villager (for example `example:desert_villager`).
pub fn is_villager_kind(kind: &str, include_modded: bool) -> bool {
    if matches!(kind, "minecraft:villager" | "minecraft:wandering_trader") {
        return true;
    }
    include_modded
        && kind
            .split_once(':')
            .is_some_and(|(namespace, path)| namespace != "minecraft" && path.contains("villager"))
}

/// All loaded living villagers in the current server level.
pub fn villagers(game: &mut crate::game::Game<'_>, include_modded: bool) -> Vec<Villager> {
    let dimension = game.dim.id();
    game.native_mobs(dimension)
        .into_iter()
        .filter(|mob| is_villager_kind(&mob.kind, include_modded))
        .map(|mob| Villager { network_id: mob.id })
        .collect()
}

fn entity_ref(
    game: &crate::game::Game<'_>,
    villager: Villager,
) -> Option<horizon_client::mod_api::EntityRef> {
    game.native_entity(villager.network_id)
}

/// Whether the entity has a non-empty custom-name component.
pub fn has_custom_name(game: &mut crate::game::Game<'_>, villager: Villager) -> bool {
    let Some(entity) = entity_ref(game, villager) else {
        return false;
    };
    let Some(nbt) = game.native_entity_nbt(entity) else {
        return false;
    };
    match nbt.get(&"CustomName".into()) {
        Some(Tag::String(value)) => !value.trim().is_empty() && value.trim() != "\"\"",
        _ => false,
    }
}

/// Set CustomName and show the name tag, as a renamed name tag does.
pub fn set_custom_name(game: &mut crate::game::Game<'_>, villager: Villager, name: &str) {
    let Some(entity) = entity_ref(game, villager) else {
        return;
    };
    let Ok(component) = serde_json::to_string(name) else {
        return;
    };
    let mut nbt = Compound::new();
    nbt.insert("CustomName".into(), Tag::String(component));
    nbt.insert("CustomNameVisible".into(), Tag::Byte(1));
    let _ = game.native_entity_merge_nbt(entity, nbt);
}

/// Clear a custom name and hide its name tag.
pub fn clear_custom_name(game: &mut crate::game::Game<'_>, villager: Villager) {
    let Some(entity) = entity_ref(game, villager) else {
        return;
    };
    let mut nbt = Compound::new();
    nbt.insert("CustomName".into(), Tag::String("\"\"".to_owned()));
    nbt.insert("CustomNameVisible".into(), Tag::Byte(0));
    let _ = game.native_entity_merge_nbt(entity, nbt);
}

/// Called from entity_tick_pre for the villager kinds this mod supports.
pub fn on_entity_tick(
    game: &mut crate::game::Game<'_>,
    entity_id: u32,
    kind: &str,
) {
    use crate::game::villager_names as m;
    if !is_villager_kind(kind, true) {
        return;
    }
    let villager = Villager { network_id: entity_id };
    if has_custom_name(game, villager) {
        return;
    }
    let options = m::options_from(&|key| game.native_setting(m::ID, key));
    let Ok(config_dir) = crate::game::Game::native_config_dir() else {
        return;
    };
    m::ensure_custom_names_file(&config_dir);
    let custom = m::load_custom_names(&config_dir);
    let mut rng = m::Rng::new();
    if let Some(name) = m::pick_name(&mut rng, &options, &custom) {
        set_custom_name(game, villager, &name);
    }
}
