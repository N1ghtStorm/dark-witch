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

use crate::database::Database;
use crate::query_handler::handle_query;
use crate::server_models::*;
use axum::routing::{delete, post, put};
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

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
        .route("/create_storage", get(create_storage))
        .route("/delete_storage", get(delete_storage))
        .route("/add_key_value", post(add_key_value))
        .route("/delete_key_value", delete(delete_key_value))
        .route("/change_value", put(change_value))
        .with_state(database);

    // run our app
    let listener = match tokio::net::TcpListener::bind("localhost:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind to port 3000: {}", e);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => println!("Server failed to start: {}", e),
    }
}

async fn handle_sql_request(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<SQLRequest>,
) -> Result<String, StatusCode> {
    println!("{:?}", request);
    println!("{:?}", request.sql);

    match handle_query(database, request.sql).await {
        Ok(result) => Ok(result),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn create_storage(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<CreateStorageRequest>,
) -> Result<String, StatusCode> {
    match database.lock().await.create_storage(request.storage_name) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn delete_storage(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<DeleteStorageRequest>,
) -> Result<String, StatusCode> {
    match database.lock().await.delete_storage(request.storage_name) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn add_key_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<AddKeyValueRequest>,
) -> Result<String, StatusCode> {
    match database
        .lock()
        .await
        .insert(request.storage_name, request.key, request.value)
    {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn delete_key_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<DeleteKeyValueRequest>,
) -> Result<String, StatusCode> {
    match database
        .lock()
        .await
        .delete(request.storage_name, request.key)
    {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn change_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<ChangeValueRequest>,
) -> Result<String, StatusCode> {
    match database
        .lock()
        .await
        .change(request.storage_name, request.key, request.new_value)
    {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
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
