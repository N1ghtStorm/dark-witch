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

use crate::kv::database::Database;
use crate::kv::error::Error;
use crate::kv::sql;
use crate::kv::witchvm_kv::WitchVMKV;

pub async fn handle_query(database: Arc<Mutex<Database>>, query: String) -> Result<String, Error> {
    let mut database = database.lock().await;
    let mut lexer = sql::Lexer::new(&query);
    let tokens = lexer.tokenize();
    let mut parser = sql::Parser::new(tokens);
    let ast = parser.parse()?;
    #[cfg(feature = "local")]
    {
        println!("{:?}", ast);
    }
    let mut generator = sql::CodeGenerator::new();
    generator.generate(&ast)?;
    let mut vm: WitchVMKV = WitchVMKV::new();
    vm.execute(&mut database, generator.instructions)?;
    Ok(vm.into_output().join(","))
}

pub async fn explain_query(database: Arc<Mutex<Database>>, query: String) -> Result<String, Error> {
    let mut database = database.lock().await;
    let mut lexer = sql::Lexer::new(&query);
    let tokens = lexer.tokenize();
    let mut parser = sql::Parser::new(tokens);
    let ast = parser.parse()?;
    #[cfg(feature = "local")]
    {
        println!("{:?}", ast);
    }
    let mut generator = sql::CodeGenerator::new();
    generator.generate(&ast)?;
    let mut vm: WitchVMKV = WitchVMKV::new();
    let results = vm.execute(&mut database, generator.instructions)?;
    Ok(results
        .into_iter()
        .map(|x| serde_json::to_string(&x).unwrap_or("{}".to_string()))
        .collect::<Vec<String>>()
        .join(","))
}
