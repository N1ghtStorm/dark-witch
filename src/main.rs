// MIT License
//
// Copyright (c) 2025
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMds+:--------:+sdNMMMMMMMMMMM
// MMMMMMMMms:-+sdNMMMMMMMMNdy+--omMMMMMMMM
// MMMMMMh:` /mMMMMMMMMMMMMMMMMm+ `-yMMMMMM
// MMMMd--hN``--sNMMMMMMMMMMNy:..`md:.hMMMM
// MMM+`yMMMy hd+./hMMMMMMh/.+dd sMMMh`/MMM
// MM:.mMMMMM:.NMMh/.+dd+./hMMM--MMMMMm--NM
// M+`mMMMMMMN`+MMMMm-  .dMMMMo mMMMMMMN.:M
// d yMMMMMMMMy dNy:.omNs--sNm oMMMMMMMMh h
// /`MMMMMMMMMM.`.+dMMMMMMm+.``NMMMMMMMMM-:
// .:MMMMMMMd+./`oMMMMMMMMMMs /.+dMMMMMMM/`
// .:MMMMmo.:yNMs dMMMMMMMMm`oMNy:.omMMMM/`
// /`MNy:.omMMMMM--MMMMMMMM:.MMMMMNs--sNM.:
// d -` :++++++++: /++++++/ :++++++++:  : h
// M+ yddddddddddd+ yddddy /dddddddddddy`/M
// MM/.mMMMMMMMMMMM.-MMMM/.NMMMMMMMMMMm.:NM
// MMMo`sMMMMMMMMMMd sMMy hMMMMMMMMMMy`+MMM
// MMMMd--hMMMMMMMMM+`mN`/MMMMMMMMMh--hMMMM
// MMMMMMh:.omMMMMMMN.:/`NMMMMMMms.:hMMMMMM
// MMMMMMMMNs:./shmMMh  yMMNds/.:smMMMMMMMM
// MMMMMMMMMMMMdy+/---``---:+sdMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

mod database;
mod sql;
mod witchvm;

use database::DatabaseInner;
use witchvm::{Filter, Instruction, WitchVM};

fn main() {
    let mut database = DatabaseInner::new();

    fill_database(&mut database);

    // let instructions = vec![
    //     Instruction::UseStorage {
    //         name: "main".to_string(),
    //     },
    //     Instruction::GetJsonField {
    //         key: "person1".to_string(),
    //         field: "name".to_string(),
    //     },
    //     Instruction::GetJsonField {
    //         key: "person1".to_string(),
    //         field: "age".to_string(),
    //     },
    // ];

    // println!("--------------------------------");

    // let mut vm: WitchVM = WitchVM::new();
    // match vm.execute(&mut database, instructions) {
    //     Ok(_) => println!("Execution successful"),
    //     Err(e) => println!("Execution failed: {}", e),
    // }

    // let mut vm: WitchVM = WitchVM::new();
    // match vm.execute(&mut database, full_scan_instructions_1()) {
    //     Ok(_) => {
    //         println!("{:?}", vm.into_output());
    //     }
    //     Err(e) => println!("Execution failed: {}", e),
    // }

    // println!("--------------------------------");

    // let mut vm: WitchVM = WitchVM::new();
    // match vm.execute(&mut database, full_scan_instructions_2()) {
    //     Ok(_) => {
    //         println!("{:?}", vm.into_output());
    //     }
    //     Err(e) => println!("Execution failed: {}", e),
    // }

    // println!("--------------------------------");

    // let mut vm: WitchVM = WitchVM::new();
    // match vm.execute(&mut database, full_scan_instructions_all()) {
    //     Ok(_) => {
    //         println!("{:?}", vm.into_output());
    //     }
    //     Err(e) => println!("Execution failed: {}", e),
    // }

    // println!("--------------------------------");

    // let sql = "SELECT * FROM main WHERE age >= 30";
    // let mut lexer = sql::Lexer::new(sql);
    // let tokens = lexer.tokenize();
    // let mut parser = sql::Parser::new(tokens);
    // let ast = parser.parse();
    // println!("{:?}", ast);

    // println!("--------------------------------");

    // let sql = "SELECT * FROM main WHERE name = \'John\'";
    // let mut lexer = sql::Lexer::new(sql);
    // let tokens = lexer.tokenize();
    // let mut parser = sql::Parser::new(tokens);
    // let ast = parser.parse();
    // let mut generator = sql::CodeGenerator::new();
    // generator.generate(&ast.unwrap());
    // let mut vm: WitchVM = WitchVM::new();
    // match vm.execute(&mut database, generator.instructions) {
    //     Ok(_) => {
    //         println!("{:?}", vm.into_output());
    //     }
    //     Err(e) => println!("Execution failed: {}", e),
    // }

    println!("--------------------------------");

    let sql = "SELECT * FROM main WHERE name = 'John' AND age >= 10 AND sex = 'male'";
    let mut lexer = sql::Lexer::new(sql);
    let tokens = lexer.tokenize();
    let mut parser = sql::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
    let mut generator = sql::CodeGenerator::new();
    generator.generate(&ast.unwrap());
    let mut vm: WitchVM = WitchVM::new();
    match vm.execute(&mut database, generator.instructions) {
        Ok(_) => {
            println!("{:?}", vm.into_output());
        }
        Err(e) => println!("Execution failed: {}", e),
    }

    // println!("{:?}", generator.instructions);
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
        "{\"name\": \"John\", \"age\": 30, \"sex\": \"male\"}".to_string(),
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

    if let Err(e) = database.insert(
        "main".to_string(),
        "person4".to_string(),
        "LALALALAL".to_string(),
    ) {
        println!("Error inserting value: {}", e);
        panic!("Failed to insert value");
    }
}
