fn villager_names_tick_listened(
    game: &mut crate::game::Game<'_>,
    entity_id: u32,
    listened: &mut bool,
) {
    use crate::game::villager_names as m;
    if !game.native_on(m::ID) {
        return;
    }
    if crate::game::villager_names_entities::is_villager_id(game, entity_id) {
        *listened = true;
    }
}