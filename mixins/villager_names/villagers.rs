// Adapter between the mod and Horizon's entity API — the ONLY file here that
// touches game internals!
//
// Before the first build, run (from a horizon-mods clone):
//   python3 native/tools/native.py check /path/to/villager-names
// It applies the bundle to Horizon's source at your pinned commit and compiles
// it: any symbol below that moved is reported with file and line, and the
// catalog (native-hooks.json in Horizon's repo) lists the current names.
//
// Needed surface (map each TODO to the entity helpers of your Nightly):
//   1. iterate loaded villagers around the player (loaded chunks);
//   2. tell whether an entity already has a custom name;
//   3. set/clear the entity's CustomName, with the name tag shown as a name
//      tag item shows it in vanilla.

pub struct Villager {
    pub network_id: u32,
}

/// All villagers currently loaded. `include_modded` also takes modded
/// villagers (content-mod types on a minecraft:villager template).
pub fn villagers(game: &mut crate::game::Game<'_>, include_modded: bool) -> Vec<Villager> {
    // TODO(verify): replace with the entity query helper from your Nightly,
    // e.g. the helpers added under "Entities for server mods" (summon, move,
    // display transforms) — filter by type id "minecraft:villager" (plus
    // "minecraft:wandering_trader" and modded villager types if wanted).
    let _ = (game, include_modded);
    Vec::new()
}

pub fn has_custom_name(game: &mut crate::game::Game<'_>, villager: Villager) -> bool {
    // TODO(verify): read the entity's CustomName component.
    let _ = (game, villager);
    false
}

/// Name the villager: sets CustomName so the name tag renders above the head
/// and the name shows on the trade screen, exactly like a renamed name tag.
pub fn set_custom_name(game: &mut crate::game::Game<'_>, villager: Villager, name: &str) {
    // TODO(verify): set the entity's CustomName and mark the tag visible.
    let _ = (game, villager, name);
}

pub fn clear_custom_name(game: &mut crate::game::Game<'_>, villager: Villager) {
    set_custom_name(game, villager, "");
}
