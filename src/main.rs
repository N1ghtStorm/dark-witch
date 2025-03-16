mod database;
mod witchvm;

use database::DatabaseInner;
use witchvm::{Instruction, WitchVM};

fn main() {
    let mut vm = WitchVM::new();
    let mut database = DatabaseInner::new();

    fill_database(&mut database);

    let instructions = vec![
        Instruction::UseStorage {
            name: "main".to_string(),
        },
        Instruction::GetJsonField {
            key: "person1".to_string(),
            field: "name".to_string(),
        },
        Instruction::GetJsonField {
            key: "person1".to_string(),
            field: "age".to_string(),
        },
    ];

    match vm.execute(&mut database, instructions) {
        Ok(_) => println!("Execution successful"),
        Err(e) => println!("Execution failed: {}", e),
    }
}

fn fill_database(database: &mut DatabaseInner) {
    if let Err(e) = database.create_storage("main".to_string()) {
        println!("Error creating storage: {}", e);
        panic!("Failed to create storage");
    }

    if let Err(e) = database.insert(
        "main".to_string(),
        "person1".to_string(),
        "{\"name\": \"John\", \"age\": 30}".to_string(),
    ) {
        println!("Error inserting value: {}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.insert(
        "main".to_string(),
        "person2".to_string(),
        "{\"name\": \"Jane\", \"age\": 25}".to_string(),
    ) {
        println!("Error inserting value: {}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.insert(
        "main".to_string(),
        "person3".to_string(),
        "{\"name\": \"Jim\", \"age\": 40}".to_string(),
    ) {
        println!("Error inserting value: {}", e);
        panic!("Failed to insert value");
    }
}
