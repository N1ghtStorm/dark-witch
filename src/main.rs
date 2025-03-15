mod witchvm;

use std::collections::HashMap;
use witchvm::{execute, Instruction};

fn main() {
    let mut main_hashmap = HashMap::<String, String>::new();

    let instructions = vec![
        Instruction::Set {
            key: "name".to_string(),
            value: "John".to_string(),
        },
        Instruction::Print {
            key: "name".to_string(),
        },
        Instruction::Clear,
        Instruction::Print {
            key: "name".to_string(),
        },
    ];

    match execute(&mut main_hashmap, instructions) {
        Ok(_) => println!("Execution successful"),
        Err(e) => println!("Execution failed: {}", e),
    }
}
