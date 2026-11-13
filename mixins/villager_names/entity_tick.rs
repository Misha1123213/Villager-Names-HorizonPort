// The entity tick hook: where villagers are found and named.
fn villager_names_entity_tick(
    game: &mut crate::game::Game<'_>,
    _dimension: &str,
    entity: u32,
    kind: &str,
) {
    use crate::game::villager_names as m;
    if !game.native_on(m::ID) {
        return;
    }
    crate::game::villager_names_entities::on_entity_tick(game, entity, kind);
}