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

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{extract::State, routing::get, Router, http::StatusCode};
use crate::database::Database;
use crate::query_handler::{handle_query, QueryResult};

pub async fn run_witch_server() {
    greet();
    println!("Running 🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙🧙 server on localhost:3000");

    let database = Arc::new(Mutex::new(Database::new()));

    #[cfg(feature = "local")]
    {
        crate::local_data::fill_database(database.clone()).await;
    }
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { pentagram() }))
        .route("/sql", get(handle_sql_request))
        .with_state(database);

    // run our app
    let listener = match tokio::net::TcpListener::bind("localhost:3000")
        .await {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to bind to port 3000: {}", e);
                return;
            }
        };

    match axum::serve(listener, app).await {
        Ok(_) => {},
        Err(e) => println!("Server failed to start: {}", e),
    }
}

async fn handle_sql_request(State(database): State<Arc<Mutex<Database>>>, sql: String) -> Result<String, StatusCode> {
    match handle_query(database, sql).await {
        QueryResult::Ok(result) => Ok(result),
        QueryResult::Err() => Err(StatusCode::BAD_REQUEST),
    }
}

fn greet() {
    println!("{}", pentagram());
}

fn pentagram() -> &'static str {
    "
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMds+:--------:+sdNMMMMMMMMMMM
    MMMMMMMMms:-+sdNMMMMMMMMNdy+--omMMMMMMMM
    MMMMMMh:` /mMMMMMMMMMMMMMMMMm+ `-yMMMMMM
    MMMMd--hN``--sNMMMMMMMMMMNy:..`md:.hMMMM
    MMM+`yMMMy hd+./hMMMMMMh/.+dd sMMMh`/MMM
    MM:.mMMMMM:.NMMh/.+dd+./hMMM--MMMMMm--NM
    M+`mMMMMMMN`+MMMMm-  .dMMMMo mMMMMMMN.:M
    d yMMMMMMMMy dNy:.omNs--sNm oMMMMMMMMh h
    /`MMMMMMMMMM.`.+dMMMMMMm+.``NMMMMMMMMM-:
    .:MMMMMMMd+./`oMMMMMMMMMMs /.+dMMMMMMM/`
    .:MMMMmo.:yNMs dMMMMMMMMm`oMNy:.omMMMM/`
    /`MNy:.omMMMMM--MMMMMMMM:.MMMMMNs--sNM.:
    d -` :++++++++: /++++++/ :++++++++:  : h
    M+ yddddddddddd+ yddddy /dddddddddddy`/M
    MM/.mMMMMMMMMMMM.-MMMM/.NMMMMMMMMMMm.:NM
    MMMo`sMMMMMMMMMMd sMMy hMMMMMMMMMMy`+MMM
    MMMMd--hMMMMMMMMM+`mN`/MMMMMMMMMh--hMMMM
    MMMMMMh:.omMMMMMMN.:/`NMMMMMMms.:hMMMMMM
    MMMMMMMMNs:./shmMMh  yMMNds/.:smMMMMMMMM
    MMMMMMMMMMMMdy+/---``---:+sdMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    "
}
