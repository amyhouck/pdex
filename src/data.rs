use indexmap::IndexMap;

// Primary Pokemon Structure
// Generation 1
#[derive(Debug, Default)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub form: String,
    pub type1: PokemonType,
    pub type2: PokemonType,
    pub height: f64,
    pub weight: f64,
    
    pub base_hp: i64,
    pub base_attack: i64,
    pub base_defense: i64,
    pub base_special: i64, // Generation 1 only
    pub base_special_attack: i64,
    pub base_special_defense: i64,
    pub base_speed: i64,
    
    pub flavor_text: IndexMap<String, String>,
}

// Pokemon Types
#[derive(Debug, Default, PartialEq)]
pub enum PokemonType {
    #[default] None = -1,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}

impl From<i64> for PokemonType {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Normal,
            1 => Self::Fighting,
            2 => Self::Flying,
            3 => Self::Poison,
            4 => Self::Ground,
            5 => Self::Rock,
            6 => Self::Bug,
            7 => Self::Ghost,
            8 => Self::Steel,
            9 => Self::Fire,
            10 => Self::Water,
            11 => Self::Grass,
            12 => Self::Electric,
            13 => Self::Psychic,
            14 => Self::Ice,
            15 => Self::Dragon,
            16 => Self::Dark,
            _ => Self::None
        }
    }
}

impl ToString for PokemonType {
    fn to_string(&self) -> String {
        match self {
            Self::Normal => String::from("Normal"),
            Self::Fighting => String::from("Fighting"),
            Self::Flying => String::from("Flying"),
            Self::Poison => String::from("Poison"),
            Self::Ground => String::from("Ground"),
            Self::Rock => String::from("Rock"),
            Self::Bug => String::from("Bug"),
            Self::Ghost => String::from("Ghost"),
            Self::Steel => String::from("Steel"),
            Self::Fire => String::from("Fire"),
            Self::Water => String::from("Water"),
            Self::Grass => String::from("Grass"),
            Self::Electric => String::from("Electric"),
            Self::Psychic => String::from("Psychic"),
            Self::Ice => String::from("Ice"),
            Self::Dragon => String::from("Dragon"),
            Self::Dark => String::from("Dark"),
            _ => String::from("None")
        }
    }
}