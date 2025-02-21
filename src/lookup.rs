use colored::Colorize;
use crate::data::{Pokemon, PokemonType};
use crate::LATEST_GENERATION;
use std::collections::HashMap;

pub fn lookup(
    db: &rusqlite::Connection,
    name: Option<String>,
    id: Option<i64>,
    generation: Option<i64>,
) {
    // Handle query for information
    let generation = generation.unwrap_or(LATEST_GENERATION);
    
    let query = if let Some(id) = id {
        format!("SELECT * FROM generation_{generation} WHERE id = {id}")  
    } else if let Some(name) = name {
        format!("SELECT * FROM generation_{generation} WHERE name = \"{name}\"")
    } else {
        panic!("Wooper :)");
    };
    
    let data = db.query_row(&query, (), |row| {
        // Set type numbers
        let mut type1: i64 = row.get("type1").unwrap();
        let type2: i64 = row.get("type2").unwrap_or(-1);
        
        if generation == 1 && type1 == 8 {
            type1 = 9;
        }
        
        // Process flavor text
        let mut flavor_text: HashMap<String, String> = HashMap::new();
        match generation {
            1 => {
                flavor_text.insert(format!("{} / {}", "Red".red(), "Blue".blue()), row.get("text_red").unwrap());
                flavor_text.insert("Yellow".yellow().to_string(), row.get("text_yellow").unwrap());
            },
            2 => {
                flavor_text.insert("Gold".bright_yellow().to_string(), row.get("text_gold").unwrap());
                flavor_text.insert("Silver".to_string(), row.get("text_silver").unwrap());
                flavor_text.insert("Crystal".cyan().to_string(), row.get("text_crystal").unwrap());
            }
            _ => {}
        }
        
        // Return struct
        Ok(Pokemon {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
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
            flavor_text,
        })
    }).unwrap();
    
    // Print information
    println!("{}: {} ({})", "Name".red(), data.name, data.id.to_string()); // NAME (ID)
    
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