use colored::Colorize;
use crate::data::{Pokemon, PokemonType};

pub fn lookup(
    db: &rusqlite::Connection,
    name: Option<String>,
    id: Option<i64>
) {
    let query = if let Some(id) = id {
        format!("SELECT * FROM generation_1 WHERE id = {id}")  
    } else if let Some(name) = name {
        format!("SELECT * FROM generation_1 WHERE name = \"{name}\"")
    } else {
        panic!("Woopsie");
    };
    
    let data = db.query_row(&query, (), |row| {
        let type1: i64 = row.get("type1").unwrap();
        let type2: i64 = row.get("type2").unwrap_or(-1);
        
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
            base_special: row.get("base_special").unwrap(),
            base_speed: row.get("base_speed").unwrap(),
            text_red: row.get("text_red").unwrap(),
            text_blue: row.get("text_blue").unwrap(),
            text_yellow: row.get("text_yellow").unwrap()
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
    println!(" - {}: {}", "Special".blue(), data.base_special);
    println!(" - {}: {}", "Speed".blue(), data.base_speed);
    
    println!("{}:", "Text:".red()); // FLAVOR TEXT
    println!(" - {} / {}: {}", "Red".red(), "Blue".blue(), data.text_red);
    println!(" - {}: {}", "Yellow".yellow(), data.text_yellow);
}