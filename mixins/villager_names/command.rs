// /villagernames name | rename | unname
fn villager_names_command(
    game: &mut crate::game::Game<'_>,
    command: &str,
    ran: &mut bool,
) {
    use crate::game::villager_names as m;

    let Some(rest) = command.strip_prefix("/villagernames") else {
        return;
    };
    *ran = true;

    let Ok(config_dir) = crate::game::Game::native_config_dir() else {
        return;
    };
    m::ensure_custom_names_file(&config_dir);
    let custom = m::load_custom_names(&config_dir);

    let options = m::options_from(&|key| game.native_setting(m::ID, key));
    let mut rng = m::Rng::new();
    let loaded = crate::game::villager_names_entities::villagers(game, options.name_modded);

    match rest.trim() {
        "" | "name" => {
            let mut count = 0usize;
            for villager in loaded {
                if crate::game::villager_names_entities::has_custom_name(game, villager) {
                    continue;
                }
                if let Some(name) = m::pick_name(&mut rng, &options, &custom) {
                    crate::game::villager_names_entities::set_custom_name(game, villager, &name);
                    count += 1;
                }
            }
            game.native_chat(&format!("[Villager Names] Named {count} villagers."));
        }
        "rename" => {
            let mut count = 0usize;
            for villager in loaded {
                if let Some(name) = m::pick_name(&mut rng, &options, &custom) {
                    crate::game::villager_names_entities::set_custom_name(game, villager, &name);
                    count += 1;
                }
            }
            game.native_chat(&format!("[Villager Names] Renamed {count} villagers."));
        }
        "unname" => {
            for villager in loaded {
                crate::game::villager_names_entities::clear_custom_name(game, villager);
            }
            game.native_chat("[Villager Names] Removed all villager names.");
        }
        _ => game.native_chat("[Villager Names] Usage: /villagernames [name|rename|unname]"),
    }
}
