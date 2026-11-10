// Once a second: find unnamed loaded villagers and give them a name.
fn villager_names_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::villager_names as m;
    if !game.native_on(m::ID) || !m::scan_due() {
        return;
    }

    let options = m::options_from(&|key| game.native_setting(m::ID, key));
    let config_dir = game.native_config_dir();
    m::ensure_custom_names_file(&config_dir);
    let custom = m::load_custom_names(&config_dir);

    let mut rng = m::Rng::new();
    for villager in crate::game::villager_names_entities::villagers(game, options.name_modded) {
        if crate::game::villager_names_entities::has_custom_name(game, villager) {
            continue;
        }
        if let Some(name) = m::pick_name(&mut rng, &options, &custom) {
            crate::game::villager_names_entities::set_custom_name(game, villager, &name);
        }
    }
}
