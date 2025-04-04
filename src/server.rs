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

use crate::kv::database::Database;
use crate::kv::query_handler::{explain_query, handle_query};
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
        crate::kv::local_data::fill_database(database.clone()).await;
    }
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { pentagram() }))
        .route("/kv/sql", get(handle_sql_request))
        .route("/kv/create_storage", get(create_storage))
        .route("/kv/delete_storage", delete(delete_storage))
        .route("/kv/add_key_value", post(add_key_value))
        .route("/kv/delete_key_value", delete(delete_key_value))
        .route("/kv/change_value", put(change_value))
        .route("/kv/get_value", get(get_value))
        .route("/kv/create_index", post(create_index))
        .route("/kv/explain", get(explain))
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
) -> Result<String, (StatusCode, String)> {
    match handle_query(database, request.sql).await {
        Ok(result) => Ok(result),
        Err(e) => {
            let err_response = match e.into_response_string() {
                Ok(response) => response,
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.into_string())),
            };
            Err((StatusCode::BAD_REQUEST, err_response))
        }
    }
}

async fn create_storage(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<CreateStorageRequest>,
) -> Result<String, (StatusCode, String)> {
    match database.lock().await.create_storage(request.storage_name) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn delete_storage(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<DeleteStorageRequest>,
) -> Result<String, (StatusCode, String)> {
    match database.lock().await.delete_storage(request.storage_name) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn add_key_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<AddKeyValueRequest>,
) -> Result<String, (StatusCode, String)> {
    match database
        .lock()
        .await
        .insert(request.storage_name, request.key, request.value)
    {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn get_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<GetValueRequest>,
) -> Result<String, (StatusCode, String)> {
    match database.lock().await.get(request.storage_name, request.key) {
        Ok(value) => Ok(value),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn delete_key_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<DeleteKeyValueRequest>,
) -> Result<String, (StatusCode, String)> {
    match database
        .lock()
        .await
        .delete(request.storage_name, request.key)
    {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn change_value(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<ChangeValueRequest>,
) -> Result<String, (StatusCode, String)> {
    match database
        .lock()
        .await
        .update(request.storage_name, request.key, request.new_value)
    {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn create_index(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<CreateIndexRequest>,
) -> Result<String, (StatusCode, String)> {
    match database.lock().await.create_index(
        request.storage_name,
        request.field_name,
        request.field_type,
        request.unique,
    ) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
    }
}

async fn explain(
    State(database): State<Arc<Mutex<Database>>>,
    Json(request): Json<ExplainRequest>,
) -> Result<String, (StatusCode, String)> {
    match explain_query(database, request.sql).await {
        Ok(result) => Ok(result),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.into_string())),
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
