// Adapter between the mod and Horizon's entity API: the only file here that
// talks to game internals. Entity handles and entity data reads/writes per
// docs/native-mods.md ("Entity and level handles") and native-hooks.json at
// the pinned commit.
//
// Naming writes CustomName (a JSON text component) and CustomNameVisible
// through native_entity_merge_nbt, exactly what /data merge entity writes;
// the name tag renders as a renamed name tag's, and the name is saved with
// the entity, as in the original Java mod.

use horizon_world::nbt::{Compound, Tag};

pub const VILLAGER_KINDS: &[&str] = &["minecraft:villager", "minecraft:wandering_trader"];

pub fn is_villager_kind(kind: &str) -> bool {
    VILLAGER_KINDS.contains(&kind)
}

pub fn is_villager_id(game: &mut crate::game::Game<'_>, entity_id: u32) -> bool {
    let Some(entity) = game.native_entity(entity_id) else {
        return false;
    };
    game.native_entity_kind(entity)
        .is_some_and(|kind| is_villager_kind(&kind))
}

pub fn on_entity_tick(game: &mut crate::game::Game<'_>, entity_id: u32) {
    use crate::game::villager_names as m;
    if m::tracker().lock().unwrap().knows_id(entity_id) {
        return;
    }
    let Some(entity) = game.native_entity(entity_id) else {
        return;
    };
    let Some(kind) = game.native_entity_kind(entity) else {
        return;
    };
    if !is_villager_kind(&kind) {
        return;
    }
    let Some(uuid) = game.native_entity_uuid(entity) else {
        return;
    };
    m::tracker().lock().unwrap().seen(entity_id, uuid);
    if has_custom_name(game, entity) {
        m::tracker().lock().unwrap().mark_processed(uuid);
        return;
    }
    let _ = name_villager(game, entity, uuid);
}

pub fn has_custom_name(
    game: &mut crate::game::Game<'_>,
    entity: horizon_client::mod_api::EntityRef,
) -> bool {
    let Some(nbt) = game.native_entity_nbt(entity) else {
        return false;
    };
    match nbt.get(&"CustomName".into()) {
        Some(Tag::String(raw)) => !raw.trim().trim_matches('"').trim().is_empty(),
        _ => false,
    }
}

pub fn set_custom_name(
    game: &mut crate::game::Game<'_>,
    entity: horizon_client::mod_api::EntityRef,
    name: &str,
) -> bool {
    let nbt: Compound = [
        ("CustomName".into(), Tag::String(format!("\"{name}\""))),
        ("CustomNameVisible".into(), Tag::Byte(1)),
    ]
    .into();
    game.native_entity_merge_nbt(entity, nbt)
}

pub fn clear_custom_name(
    game: &mut crate::game::Game<'_>,
    entity: horizon_client::mod_api::EntityRef,
) -> bool {
    let nbt: Compound = [
        ("CustomName".into(), Tag::String("\"\"".into())),
        ("CustomNameVisible".into(), Tag::Byte(0)),
    ]
    .into();
    game.native_entity_merge_nbt(entity, nbt)
}

pub fn name_villager(
    game: &mut crate::game::Game<'_>,
    entity: horizon_client::mod_api::EntityRef,
    uuid: u128,
) -> bool {
    use crate::game::villager_names as m;
    let options = m::options_from(&|key| game.native_setting(m::ID, key));
    let Ok(config_dir) = crate::game::Game::native_config_dir() else {
        return false;
    };
    m::ensure_custom_names_file(&config_dir);
    let custom = m::load_custom_names(&config_dir);
    let mut rng = m::Rng::new();
    let named = m::pick_name(&mut rng, &options, &custom)
        .is_some_and(|name| set_custom_name(game, entity, &name));
    if named {
        m::tracker().lock().unwrap().mark_processed(uuid);
    }
    named
}

pub fn live_villagers(
    game: &mut crate::game::Game<'_>,
) -> Vec<(horizon_client::mod_api::EntityRef, u128)> {
    use crate::game::villager_names as m;
    let mut out = Vec::new();
    let tracked = m::tracker().lock().unwrap().villagers();
    for (id, uuid) in tracked {
        let Some(entity) = game.native_entity(id) else {
            continue;
        };
        let Some(kind) = game.native_entity_kind(entity) else {
            continue;
        };
        if is_villager_kind(&kind) {
            out.push((entity, uuid));
        }
    }
    out
}