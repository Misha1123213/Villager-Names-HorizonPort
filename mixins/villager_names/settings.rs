fn villager_names_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::villager_names as m;
    if mod_id != m::ID {
        return;
    }
    for (key, label) in m::OPTION_KEYS {
        let on = m::option(key, game.native_setting(m::ID, key).as_ref());
        buttons.push(format!("{label}: {}", if on { "ON" } else { "OFF" }));
    }
}