// Villager Names for Horizon - a port of Serilum's "Villager Names" mod.
// Original mod by Serilum (ricksouth): https://github.com/Serilum/Villager-Names

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

pub const ID: &str = "villager_names";
pub const VERSION: &str = "1.0.0";
pub const CUSTOM_NAMES_PATH: &str = "villagernames/customnames.txt";

pub const DEFAULT_MALE: &[&str] = &[
    "Aaron", "Adam", "Adrian", "Alan", "Albert", "Alex", "Alfred", "Andrew", "Anthony", "Arthur",
    "Barry", "Ben", "Bill", "Bob", "Brian", "Bruce", "Carl", "Charles", "Chris", "Daniel",
    "David", "Dennis", "Edward", "Eric", "Ethan", "Felix", "Frank", "Fred", "Gary", "George",
    "Harold", "Harry", "Henry", "Jack", "Jacob", "James", "Jason", "John", "Joseph", "Kevin",
    "Leo", "Liam", "Luke", "Mark", "Martin", "Matt", "Max", "Michael", "Nathan", "Nicholas",
    "Noah", "Oliver", "Owen", "Patrick", "Paul", "Peter", "Richard", "Robert", "Ronald", "Ryan",
    "Sam", "Scott", "Sean", "Simon", "Steve", "Stuart", "Terry", "Thomas", "Tim", "Tom",
    "Victor", "Vincent", "Walter", "Wayne", "William", "Winston", "Zachary",
];

pub const DEFAULT_FEMALE: &[&str] = &[
    "Abby", "Alice", "Amanda", "Amber", "Amelia", "Amy", "Anna", "Audrey", "Barbara", "Bella",
    "Beth", "Betty", "Brenda", "Carla", "Carol", "Caroline", "Catherine", "Charlotte", "Chloe", "Claire",
    "Clara", "Daisy", "Diana", "Doris", "Edith", "Eleanor", "Elizabeth", "Ella", "Emily", "Emma",
    "Evelyn", "Fiona", "Flora", "Florence", "Grace", "Hannah", "Hazel", "Helen", "Holly", "Irene",
    "Iris", "Isabel", "Jane", "Jasmine", "Jennifer", "Jessica", "Julia", "Karen", "Kate", "Katherine",
    "Laura", "Leah", "Lily", "Linda", "Lisa", "Lucy", "Lydia", "Maggie", "Margaret", "Maria",
    "Martha", "Mary", "Megan", "Melissa", "Molly", "Nancy", "Natalie", "Nina", "Nora", "Olivia",
    "Pamela", "Patricia", "Rachel", "Rebecca", "Rosa", "Rose", "Ruby", "Ruth", "Sally", "Samantha",
    "Sandra", "Sarah", "Sophia", "Sophie", "Stella", "Susan", "Sylvia", "Teresa", "Valerie", "Vera",
    "Victoria", "Violet", "Virginia", "Wendy", "Zoe",
];

#[derive(Clone, Copy)]
pub struct Options {
    pub use_custom: bool,
    pub use_male: bool,
    pub use_female: bool,
    pub use_both: bool,
    pub name_modded: bool,
    pub capitalize: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            use_custom: true,
            use_male: true,
            use_female: true,
            use_both: false,
            name_modded: true,
            capitalize: true,
        }
    }
}

pub const OPTION_KEYS: [(&str, &str); 6] = [
    ("use_custom", "Use custom names"),
    ("use_male", "Use male names"),
    ("use_female", "Use female names"),
    ("use_both", "Custom + default together"),
    ("name_modded", "Name modded villagers"),
    ("capitalize", "Capitalize names"),
];

pub fn option(key: &str, value: Option<&String>) -> bool {
    let default = Options::default();
    let default = match key {
        "use_custom" => default.use_custom,
        "use_male" => default.use_male,
        "use_female" => default.use_female,
        "use_both" => default.use_both,
        "name_modded" => default.name_modded,
        "capitalize" => default.capitalize,
        _ => true,
    };
    value.map(|v| v == "true").unwrap_or(default)
}

pub fn options_from(get: &dyn Fn(&str) -> Option<String>) -> Options {
    Options {
        use_custom: option("use_custom", get("use_custom").as_ref()),
        use_male: option("use_male", get("use_male").as_ref()),
        use_female: option("use_female", get("use_female").as_ref()),
        use_both: option("use_both", get("use_both").as_ref()),
        name_modded: option("name_modded", get("name_modded").as_ref()),
        capitalize: option("capitalize", get("capitalize").as_ref()),
    }
}

static RNG_STATE: AtomicU64 = AtomicU64::new(0);

pub struct Rng(u64);

impl Rng {
    pub fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        let prev = RNG_STATE.fetch_add(nanos | 1, Ordering::Relaxed);
        Rng((nanos ^ prev.wrapping_mul(0x2545_F491_4F6C_DD1D)) | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 { 0 } else { (self.next() % n as u64) as usize }
    }
}

pub fn capitalize(name: &str) -> String {
    name.split(' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn parse_custom_names(text: &str) -> Vec<String> {
    text.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn pick_name(rng: &mut Rng, options: &Options, custom: &[String]) -> Option<String> {
    let mut pool: Vec<&str> = Vec::new();
    let custom_ok = options.use_custom && !custom.is_empty();
    let custom_only = custom_ok && !options.use_both;
    if custom_ok {
        pool.extend(custom.iter().map(String::as_str));
    }
    if !custom_only {
        if options.use_male {
            pool.extend(DEFAULT_MALE.iter().copied());
        }
        if options.use_female {
            pool.extend(DEFAULT_FEMALE.iter().copied());
        }
    }
    if pool.is_empty() {
        return None;
    }
    let mut name = pool[rng.below(pool.len())].to_string();
    if options.capitalize {
        name = capitalize(&name);
    }
    Some(name)
}

pub fn custom_names_file(config_dir: &Path) -> PathBuf {
    config_dir.join(CUSTOM_NAMES_PATH)
}

pub fn ensure_custom_names_file(config_dir: &Path) {
    let path = custom_names_file(config_dir);
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, "");
}

pub fn load_custom_names(config_dir: &Path) -> Vec<String> {
    std::fs::read_to_string(custom_names_file(config_dir))
        .map(|text| parse_custom_names(&text))
        .unwrap_or_default()
}

#[derive(Default)]
pub struct Tracker {
    by_id: HashMap<u32, u128>,
    processed: HashSet<u128>,
}

impl Tracker {
    pub fn knows_id(&self, id: u32) -> bool {
        self.by_id.contains_key(&id)
    }
    pub fn seen(&mut self, id: u32, uuid: u128) {
        self.by_id.insert(id, uuid);
    }
    pub fn mark_processed(&mut self, uuid: u128) {
        self.processed.insert(uuid);
    }
    pub fn is_processed(&self, uuid: u128) -> bool {
        self.processed.contains(&uuid)
    }
    pub fn villagers(&self) -> Vec<(u32, u128)> {
        self.by_id.iter().map(|(id, uuid)| (*id, *uuid)).collect()
    }
}

static TRACKER: OnceLock<Mutex<Tracker>> = OnceLock::new();

pub fn tracker() -> &'static Mutex<Tracker> {
    TRACKER.get_or_init(|| Mutex::new(Tracker::default()))
}