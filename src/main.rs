mod database;
mod witchvm;
mod sql;

use database::DatabaseInner;
use witchvm::{Filter, Instruction, WitchVM};

fn main() {
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

    let mut vm: WitchVM = WitchVM::new();
    match vm.execute(&mut database, instructions) {
        Ok(_) => println!("Execution successful"),
        Err(e) => println!("Execution failed: {}", e),
    }

    let mut vm: WitchVM = WitchVM::new();
    match vm.execute(&mut database, full_scan_instructions_1()) {
        Ok(_) => {
            println!("{:?}", vm.into_output());
        }
        Err(e) => println!("Execution failed: {}", e),
    }

    let mut vm: WitchVM = WitchVM::new();
    match vm.execute(&mut database, full_scan_instructions_2()) {
        Ok(_) => {
            println!("{:?}", vm.into_output());
        }
        Err(e) => println!("Execution failed: {}", e),
    }

    let mut vm: WitchVM = WitchVM::new();
    match vm.execute(&mut database, full_scan_instructions_all()) {
        Ok(_) => {
            println!("{:?}", vm.into_output());
        }
        Err(e) => println!("Execution failed: {}", e),
    }


    let sql = "SELECT * FROM main WHERE age >= 30";
    let mut lexer = sql::Lexer::new(sql);
    let tokens = lexer.tokenize();
    let mut parser = sql::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);


    let sql = "SELECT * FROM main WHERE name = \'John\'";
    let mut lexer = sql::Lexer::new(sql);
    let tokens = lexer.tokenize();
    let mut parser = sql::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
}

fn full_scan_instructions_1() -> Vec<Instruction> {
    let filter = Box::new(|_, value: String| {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&value) {
            if let Some(age) = json.get("age").and_then(|v| v.as_i64()) {
                return age >= 30;
            }
        }
        false
    });

    vec![
        Instruction::UseStorage {
            name: "main".to_string(),
        },
        Instruction::FullScan {
            maybe_filter: Some(Filter::Condition(filter)),
        },
    ]
}

fn full_scan_instructions_2() -> Vec<Instruction> {
    let filter = Box::new(|_, value: String| {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&value) {
            if let Some(name) = json.get("name").and_then(|v| v.as_str()) {
                return name.contains('J') && name.contains('e');
            }
        }
        false
    });

    vec![
        Instruction::UseStorage {
            name: "main".to_string(),
        },
        Instruction::FullScan {
            maybe_filter: Some(Filter::Condition(filter)),
        },
    ]
}

fn full_scan_instructions_all() -> Vec<Instruction> {
    let filter = Box::new(|_, _: String| true);

    vec![
        Instruction::UseStorage {
            name: "main".to_string(),
        },
        Instruction::FullScan {
            maybe_filter: Some(Filter::Condition(filter)),
        },
    ]
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

    if let Err(e) = database.insert(
        "main".to_string(),
        "person4".to_string(),
        "{\"name\": \"Jopel\", \"age\": 29}".to_string(),
    ) {
        println!("Error inserting value: {}", e);
        panic!("Failed to insert value");
    }
}
