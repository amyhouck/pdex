// Primary Pokemon Structure
// Generation 1
#[derive(Debug)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub type1: PokemonType,
    pub type2: PokemonType,
    pub height: f64,
    pub weight: f64,
    pub base_hp: i64,
    pub base_attack: i64,
    pub base_defense: i64,
    pub base_special: i64,
    pub base_speed: i64,
    pub text_red: String,
    #[allow(dead_code)] pub text_blue: String, // Allowing dead code temporarily because this field is not currently being used but exists in the database.
    pub text_yellow: String,
}

// Pokemon Types
#[derive(Debug, PartialEq)]
pub enum PokemonType {
    None = -1,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon
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
            8 => Self::Fire,
            9 => Self::Water,
            10 => Self::Grass,
            11 => Self::Electric,
            12 => Self::Psychic,
            13 => Self::Ice,
            14 => Self::Dragon,
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
            Self::Fire => String::from("Fire"),
            Self::Water => String::from("Water"),
            Self::Grass => String::from("Grass"),
            Self::Electric => String::from("Electric"),
            Self::Psychic => String::from("Psychic"),
            Self::Ice => String::from("Ice"),
            Self::Dragon => String::from("Dragon"),
            _ => String::from("None")
        }
    }
}