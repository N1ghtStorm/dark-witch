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

use super::database::Database;
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;

// Only for testing purposes, used in "local" feature. Will be removed in future.
pub async fn fill_database(database: Arc<Mutex<Database>>) -> Arc<Mutex<Database>> {
    if let Err(e) = database.lock().await.create_storage("main".to_string()) {
        println!("Error creating storage: {:?}", e);
        panic!("Failed to create storage");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person1".to_string(),
        "{\"name\": \"John\", \"age\": 30, \"gender\": \"male\"}".to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person2".to_string(),
        "{\"name\": \"Jane\", \"age\": 25}".to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person3".to_string(),
        "{\"name\": \"Jim\", \"age\": 40}".to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person4".to_string(),
        "{\"name\": \"Jopel\", \"age\": 29}".to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person5".to_string(),
        "{\"name\": \"Khristina\", \"age\": 22, \"gender\": \"female\"}".to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    if let Err(e) = database.lock().await.insert(
        "main".to_string(),
        "person6".to_string(),
        "{\"name\": \"Veronika\", \"age\": 35, \"gender\": \"female\", \"address\": \"Mashroom\"}"
            .to_string(),
    ) {
        println!("Error inserting value: {:?}", e);
        panic!("Failed to insert value");
    }

    for i in 20..100000 {
        if let Err(e) = database.lock().await.insert(
            "main".to_string(),
            format!("person{}", i),
            format!(
                "{{\"name\": \"Person{}\", \"age\": {}, \"gender\": \"{}\"}}",
                i,
                //make random number between 18 and 90
                rand::thread_rng().gen_range(18..=90),
                if rand::thread_rng().gen_bool(0.05) {
                    "male"
                } else {
                    "female"
                }
            ),
        ) {
            println!("Error inserting value: {:?}", e);
            panic!("Failed to insert value");
        }
    }

    database
}
