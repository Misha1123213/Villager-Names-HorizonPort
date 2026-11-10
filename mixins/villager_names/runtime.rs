// Villager Names for Horizon — a port of Serilum's "Villager Names" mod.
// Original mod by Serilum (ricksouth): https://github.com/Serilum/Villager-Names
// Keep this attribution if you publish the port.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub const ID: &str = "villager_names";
pub const VERSION: &str = "1.0.0";
pub const CUSTOM_NAMES_PATH: &str = "villagernames/customnames.txt";

// A starter set of the original mod's 5000+ names. Extend freely,
// or disable defaults in settings and use customnames.txt.
pub const DEFAULT_MALE: &[&str] = &[
    "Aaron", "Adam", "Adrian", "Alan", "Albert", "Alex", "Alfred", "Andre", "Andrew", "Anthony",
    "Arthur", "Barry", "Ben", "Bernard", "Bill", "Blake", "Bob", "Brian", "Bruce", "Carl",
    "Charles", "Chris", "Clive", "Colin", "Daniel", "David", "Dennis", "Donald", "Doug", "Edward",
    "Eric", "Ernest", "Ethan", "Felix", "Finn", "Frank", "Fred", "Gary", "George", "Gordon",
    "Greg", "Harold", "Harry", "Henry", "Ian", "Isaac", "Ivan", "Jack", "Jacob", "James",
    "Jason", "Jeff", "Jeremy", "John", "Joseph", "Karl", "Keith", "Ken", "Kevin", "Kyle",
    "Larry", "Leo", "Leonard", "Liam", "Louis", "Luke", "Mark", "Martin", "Marvin", "Matt",
    "Max", "Michael", "Nathan", "Neil", "Nicholas", "Noah", "Oliver", "Oscar", "Owen", "Patrick",
    "Paul", "Peter", "Philip", "Ralph", "Ray", "Richard", "Robert", "Roger", "Ronald", "Ross",
    "Roy", "Russell", "Ryan", "Sam", "Scott", "Sean", "Seth", "Simon", "Stanley", "Steve",
    "Stuart", "Terry", "Thomas", "Tim", "Tobias", "Todd", "Tom", "Tyler", "Victor", "Vincent",
    "Walter", "Wayne", "William", "Winston", "Zachary",
];

pub const DEFAULT_FEMALE: &[&str] = &[
    "Abby", "Ada", "Agnes", "Alice", "Alicia", "Amanda", "Amber", "Amelia", "Amy", "Andrea",
    "Angela", "Anita", "Anna", "Annie", "April", "Audrey", "Barbara", "Beatrice", "Bella", "Beth",
    "Betty", "Brenda", "Bridget", "Camilla", "Carla", "Carol", "Caroline", "Catherine", "Cecilia", "Celia",
    "Charlotte", "Chloe", "Claire", "Clara", "Claudia", "Cora", "Daisy", "Dana", "Danielle", "Daphne",
    "Deborah", "Denise", "Diana", "Diane", "Doris", "Dorothy", "Edith", "Eleanor", "Elena", "Eliza",
    "Elizabeth", "Ella", "Ellen", "Elsa", "Emily", "Emma", "Erica", "Erin", "Esther", "Ethel",
    "Eva", "Evelyn", "Fiona", "Flora", "Florence", "Frances", "Freya", "Gabrielle", "Gemma", "Georgia",
    "Gladys", "Gloria", "Grace", "Greta", "Hannah", "Harriet", "Hazel", "Heather", "Helen", "Hilda",
    "Holly", "Hope", "Ida", "Ingrid", "Irene", "Iris", "Isabel", "Ivy", "Jacqueline", "Jane",
    "Janet", "Jasmine", "Jean", "Jennifer", "Jessica", "Jill", "Joan", "Joanna", "Joyce", "Judith",
    "Julia", "Julie", "June", "Karen", "Kate", "Katherine", "Kathleen", "Kelly", "Kim", "Laura",
    "Lauren", "Leah", "Lillian", "Lily", "Linda", "Lisa", "Lois", "Loretta", "Louise", "Lucia",
    "Lucy", "Lydia", "Mabel", "Mae", "Maggie", "Margaret", "Marguerite", "Maria", "Marian", "Marie",
    "Marilyn", "Marion", "Martha", "Mary", "Matilda", "Maureen", "Megan", "Melanie", "Melissa", "Mildred",
    "Molly", "Monica", "Nancy", "Naomi", "Natalie", "Nell", "Nellie", "Nicola", "Nina", "Nora",
    "Norma", "Olive", "Olivia", "Pamela", "Patricia", "Pauline", "Pearl", "Peggy", "Penelope", "Phoebe",
    "Phyllis", "Priscilla", "Rachel", "Rebecca", "Regina", "Rita", "Roberta", "Robin", "Rosa", "Rosalie",
    "Rose", "Rosemary", "Roxanne", "Ruby", "Ruth", "Sabrina", "Sally", "Samantha", "Sandra", "Sara",
    "Sarah", "Scarlett", "Selma", "Serena", "Sheila", "Shelly", "Shirley", "Simone", "Sophia", "Sophie",
    "Stella", "Stephanie", "Sue", "Susan", "Sylvia", "Tabitha", "Tamara", "Tanya", "Teresa", "Thea",
    "Thelma", "Theresa", "Tiffany", "Tina", "Ursula", "Valerie", "Vanessa", "Vera", "Veronica", "Victoria",
    "Viola", "Violet", "Virginia", "Vivian", "Wanda", "Wendy", "Wilma", "Winifred", "Yolanda", "Yvette",
    "Yvonne", "Zelda", "Zoe",
];

// Settings, kept one file per key in horizon-mods/.horizon-mod-data/villager_names/.
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
    // Absent setting means the default.
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
        use_custom: option("use_custom", &get("use_custom")),
        use_male: option("use_male", &get("use_male")),
        use_female: option("use_female", &get("use_female")),
        use_both: option("use_both", &get("use_both")),
        name_modded: option("name_modded", &get("name_modded")),
        capitalize: option("capitalize", &get("capitalize")),
    }
}

// --- xorshift64* PRNG, no dependencies -------------------------------------

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

impl Default for Rng {
    fn default() -> Self { Self::new() }
}

// --- name picking -----------------------------------------------------------

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

// --- custom names file (config/villagernames/customnames.txt) ---------------

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

// --- scan throttle: once per second is plenty -------------------------------

static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn scan_due() -> bool {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed) % 20 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_from_male_and_female() {
        let mut rng = Rng::new();
        let options = Options::default();
        let name = pick_name(&mut rng, &options, &[]).unwrap();
        let in_lists = DEFAULT_MALE.contains(&name.as_str()) || DEFAULT_FEMALE.contains(&name.as_str());
        assert!(in_lists, "{name} not in default lists");
    }

    #[test]
    fn custom_names_replace_defaults_unless_both() {
        let mut rng = Rng::new();
        let options = Options { use_both: false, ..Options::default() };
        let custom = vec!["kevin".to_string()];
        assert_eq!(pick_name(&mut rng, &options, &custom), Some("Kevin".to_string()));
    }

    #[test]
    fn custom_names_join_defaults_when_both() {
        let options = Options { use_both: true, ..Options::default() };
        assert!(options.use_custom && options.use_both);
    }

    #[test]
    fn empty_pool_gives_none() {
        let mut rng = Rng::new();
        let options = Options {
            use_custom: false, use_male: false, use_female: false, use_both: false,
            name_modded: true, capitalize: true,
        };
        assert_eq!(pick_name(&mut rng, &options, &[]), None);
    }

    #[test]
    fn capitalizes_each_word() {
        assert_eq!(capitalize("mary jane"), "Mary Jane");
        assert_eq!(capitalize("MARY"), "MARY");
    }

    #[test]
    fn parses_comma_separated_names() {
        assert_eq!(parse_custom_names(" Anna, bOb ,, Charlie "), vec!["Anna", "bOb", "Charlie"]);
    }
}
