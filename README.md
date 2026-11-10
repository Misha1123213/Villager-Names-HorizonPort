# Villager Names (Horizon port)

## Install (players)

Copy the mod folder into the game's `horizon-native-mods/` directory, then use the launcher's **Build** / **Native Compilation** action.

## Custom names

Add comma-separated names to `config/villagernames/customnames.txt`. The file is created automatically on first use.

## Commands

- `/villagernames name` — name all unnamed loaded villagers
- `/villagernames rename` — give every loaded villager a new name
- `/villagernames unname` — remove all names

## Not ported (v1)

The original mod's trade-screen profession display and additional configuration options are not included in this port yet.

## Build and test

Requires a clone of `horizon-rust/horizon-mods`:

```bash
git clone https://github.com/horizon-rust/horizon-mods
cd horizon-mods
python3 native/tools/native.py check /path/to/villager-names
```

The first build may take several minutes; later builds should be faster. If an API symbol in `villagers.rs` does not match your Nightly, the compiler will report the file and line. Check Horizon's `native-hooks.json` catalog and update the adapter functions as needed. The game-dependent code is isolated in `villagers.rs`.

Run the logic tests without the game from the mod directory:

```bash
cargo test
```

## License

This port is based on Villager Names by Serilum. See the upstream project and licensing terms before redistributing: https://github.com/Serilum/Villager-Names
