mod witchvm;
mod database;

use std::collections::HashMap;
use witchvm::{WitchVM, Instruction};

fn main() {
    let mut main_hashmap = HashMap::<String, String>::new();

    let instructions = vec![
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
        // Instruction::Clear,
        // Instruction::Print {
        //     key: "name".to_string(),
        // },
    ];

    let mut vm = WitchVM::new();

    match vm.execute(&mut main_hashmap, instructions) {
        Ok(_) => println!("Execution successful"),
        Err(e) => println!("Execution failed: {}", e),
    }
}
