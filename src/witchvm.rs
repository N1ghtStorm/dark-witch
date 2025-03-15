use std::collections::HashMap;

pub fn execute(hashmap: &mut HashMap<String, String>, instructions: Vec<Instruction>) -> Result<(), String> {
    for instruction in instructions {
        match instruction {
            Instruction::Get { key } => {
                match hashmap.get(&key) {
                    Some(value) => println!("Value for key '{}': {}", key, value),
                    None => return Err(format!("Key '{}' not found", key)),
                }
            },
            Instruction::Set { key, value } => {
                hashmap.insert(key.clone(), value);
                println!("Set value for key '{}'", key);
            },
            Instruction::Delete { key } => {
                if hashmap.remove(&key).is_none() {
                    return Err(format!("Key '{}' not found for deletion", key));
                }
                println!("Deleted key '{}'", key);
            },
            Instruction::Print { key } => {
                match hashmap.get(&key) {
                    Some(value) => println!("{}", value),
                    None => return Err(format!("Key '{}' not found for printing", key)),
                }
            },
            Instruction::Clear => {
                hashmap.clear();
                println!("Cleared all entries");
            },
        }
    }
    
    Ok(())
}

pub enum Instruction {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Print { key: String },
    Clear,
}