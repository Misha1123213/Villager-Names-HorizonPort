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

    let villagers = crate::game::villager_names_entities::live_villagers(game);
    match rest.trim() {
        "" | "name" => {
            let mut count = 0usize;
            for (id, uuid) in villagers {
                if m::tracker().lock().unwrap().is_processed(uuid) {
                    continue;
                }
                if crate::game::villager_names_entities::has_custom_name(game, id) {
                    m::tracker().lock().unwrap().mark_processed(uuid);
                    continue;
                }
                if crate::game::villager_names_entities::name_villager(game, id, uuid) {
                    count += 1;
                }
            }
            game.native_chat(&format!("[Villager Names] Named {count} villagers."));
        }
        "rename" => {
            let mut count = 0usize;
            for (id, uuid) in villagers {
                if crate::game::villager_names_entities::name_villager(game, id, uuid) {
                    count += 1;
                }
            }
            game.native_chat(&format!("[Villager Names] Renamed {count} villagers."));
        }
        "unname" => {
            for (id, uuid) in villagers {
                let _ = crate::game::villager_names_entities::clear_custom_name(game, id);
                m::tracker().lock().unwrap().mark_processed(uuid);
            }
            game.native_chat("[Villager Names] Removed all villager names.");
        }
        _ => game.native_chat("[Villager Names] Usage: /villagernames [name|rename|unname]"),
    }
}