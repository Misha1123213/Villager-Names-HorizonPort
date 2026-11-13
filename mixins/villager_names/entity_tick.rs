fn villager_names_entity_tick(game: &mut crate::game::Game<'_>, entity_id: u32) {
    use crate::game::villager_names as m;
    if !game.native_on(m::ID) {
        return;
    }
    crate::game::villager_names_entities::on_entity_tick(game, entity_id);
}