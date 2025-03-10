use colored::Colorize;
use rusqlite::Row;
use crate::data::{Pokemon, PokemonType};
use crate::LATEST_GENERATION;
use indexmap::IndexMap;
use anyhow::{anyhow, Result};

fn retrieve_flavor_text(generation: i64, row: &Row<'_>) -> IndexMap<String, String> {
    let mut flavor_text: IndexMap<String, String> = IndexMap::new();
        
    match generation {
        1 => {
            flavor_text.insert(format!("{} / {}", "Red".red(), "Blue".blue()), row.get("text_red").unwrap());
            flavor_text.insert("Yellow".yellow().to_string(), row.get("text_yellow").unwrap());
        },
        2 => {
            flavor_text.insert("Gold".bright_yellow().to_string(), row.get("text_gold").unwrap());
            flavor_text.insert("Silver".to_string(), row.get("text_silver").unwrap());
            flavor_text.insert("Crystal".cyan().to_string(), row.get("text_crystal").unwrap());
        },
        3 => {
            flavor_text.insert("Ruby".red().to_string(), row.get("text_ruby").unwrap());
            flavor_text.insert("Sapphire".bright_blue().to_string(), row.get("text_sapphire").unwrap());
            flavor_text.insert("Emerald".green().to_string(), row.get("text_emerald").unwrap());
            flavor_text.insert("Firered".bright_red().to_string(), row.get("text_firered").unwrap());
            flavor_text.insert("Leafgreen".bright_green().to_string(), row.get("text_leafgreen").unwrap());
        },
        4 => {
            flavor_text.insert("Diamond".blue().to_string(), row.get("text_diamond").unwrap());
            flavor_text.insert("Pearl".bright_red().to_string(), row.get("text_pearl").unwrap());
            flavor_text.insert("Platinum".white().to_string(), row.get("text_platinum").unwrap());
            flavor_text.insert("HeartGold".yellow().to_string(), row.get("text_heartgold").unwrap());
            flavor_text.insert("SoulSilver".cyan().to_string(), row.get("text_soulsilver").unwrap());
        },
        5 => {
            flavor_text.insert("Black".to_string(), row.get("text_black").unwrap());
            flavor_text.insert("White".to_string(), row.get("text_white").unwrap());
            flavor_text.insert("Black 2".to_string(), row.get("text_black_2").unwrap());
            flavor_text.insert("White 2".to_string(), row.get("text_white_2").unwrap());
        },
        _ => {}
    }
    
    flavor_text
}

fn print_pokedex(data: Pokemon, generation: i64) {
    println!("{}: {} ({})", "Name".red(), data.name, data.id.to_string()); // NAME (ID)
    
    if data.form != String::new() {
        println!("{}: {}", "Form".red(), data.form); // FORM (if the pokemon has one)
    }
    
    if data.type2 == PokemonType::None { // TYPE or TYPES
        println!("{}: {}", "Type".red(), data.type1.to_string());
    } else {
        println!("{}: {} / {}", "Types".red(), data.type1.to_string(), data.type2.to_string());
    }
    
    println!("{}: {}m", "Height".red(), data.height); // HEIGHT
    println!("{}: {}kg", "Weight".red(), data.weight); // WEIGHT
    
    println!("{}:", "Base Stats".red()); //BASE STATS
    println!(" - {}: {}", "HP".blue(), data.base_hp);
    println!(" - {}: {}", "Attack".blue(), data.base_attack);
    println!(" - {}: {}", "Defense".blue(), data.base_defense);
    if generation == 1 {
        println!(" - {}: {}", "Special".blue(), data.base_special);
    } else {
        println!(" - {}: {}", "Special Attack".blue(), data.base_special_attack);
        println!(" - {}: {}", "Special Defense".blue(), data.base_special_defense);
    }
    println!(" - {}: {}", "Speed".blue(), data.base_speed);
    
    println!("{}:", "Text".red()); // FLAVOR TEXT
    for (game, text) in data.flavor_text {
        println!(" - {}: {}", game, text);
    }
}

pub fn lookup(
    db: &rusqlite::Connection,
    name: Option<String>,
    id: Option<i64>,
    generation: Option<i64>,
    form: Option<String>
) -> Result<()> {
    // Build query
    let generation = generation.unwrap_or(LATEST_GENERATION);
    
    if generation > LATEST_GENERATION || generation <= 0 {
        let err = format!("You must use a valid generation number! 1 through {LATEST_GENERATION}");
        return Err(anyhow!(err));
    }
    
    let mut query = if let Some(id) = id {
        format!("SELECT * FROM generation_{generation} WHERE id = {id}")  
    } else if let Some(name) = name {
        format!("SELECT * FROM generation_{generation} WHERE name = \"{name}\"")
    } else {
        return Err(anyhow!("You must include either the pokemon's name or ID!"));
    };
    
    if let Some(form) = form {
        query = format!("{query} AND form LIKE '%{form}%'");
    }
    
    // Execute query
    let data = db.query_row(&query, (), |row| {
        // Set type numbers
        let mut type1: i64 = row.get("type1").unwrap();
        let type2: i64 = row.get("type2").unwrap_or(-1);
        
        if generation == 1 && type1 == 8 { // This is to adjust the type enum due to the type additions in generation 2
            type1 = 9;
        }
        
        // Return struct
        Ok(Pokemon {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
            form: row.get("form").unwrap_or(String::new()),
            type1: PokemonType::from(type1),
            type2: PokemonType::from(type2),
            height: row.get("height").unwrap(),
            weight: row.get("weight").unwrap(),
            base_hp: row.get("base_hp").unwrap(),
            base_attack: row.get("base_attack").unwrap(),
            base_defense: row.get("base_defense").unwrap(),
            base_special: row.get("base_special").unwrap_or(0),
            base_special_attack: row.get("base_special_attack").unwrap_or(0),
            base_special_defense: row.get("base_special_defense").unwrap_or(0),
            base_speed: row.get("base_speed").unwrap(),
            flavor_text: retrieve_flavor_text(generation, row),
        })
    });
    
    let data = match data {
        Ok(poke) => poke,
        Err(_) => return Err(anyhow!("No pokemon found with the given information!"))
    };
    
    print_pokedex(data, generation);
    
    Ok(())
}