// Opt the mod in to per-entity ticks: entity_tick_pre runs for an entity
// only when a mod set listened = true in this hook (per mod, not per entity).
fn villager_names_tick_listened(game: &mut crate::game::Game<'_>, listened: &mut bool) {
    use crate::game::villager_names as m;
    if game.native_on(m::ID) {
        *listened = true;
    }
}