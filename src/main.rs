mod database;
mod witchvm;

use database::DatabaseInner;
use witchvm::{Instruction, WitchVM};

fn main() {
    let mut vm = WitchVM::new();
    let mut database = DatabaseInner::new();

    if let Err(e) = database.create_storage("main".to_string()) {
        println!("Error creating storage: {}", e);
        panic!("Failed to create storage");
    }

    let instructions = vec![
        Instruction::UseStorage {
            name: "main".to_string(),
        },
        Instruction::Set {
            key: "person1".to_string(),
            value: "{\"name\": \"John\", \"age\": 30}".to_string(),
        },
        Instruction::Print {
            key: "person1".to_string(),
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
