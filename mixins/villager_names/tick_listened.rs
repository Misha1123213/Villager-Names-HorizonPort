// Opt in to server-side entity tick notifications. The entity kind is
// supplied by entity_tick_pre, where villager types can be filtered.
fn villager_names_tick_listened(
    game: &mut crate::game::Game<'_>,
    listened: &mut bool,
) {
    use crate::game::villager_names as m;
    if game.native_on(m::ID) {
        *listened = true;
    }
}
