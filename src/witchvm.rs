use crate::database::Database;

pub struct WitchVM {
    instruction_storage_name: Option<String>,
    output: Vec<String>,
}

impl WitchVM {
    pub fn new() -> Self {
        Self {
            instruction_storage_name: None,
            output: Vec::new(),
        }
    }

    pub fn into_output(self) -> Vec<String> {
        self.output
    }

    pub fn execute(
        &mut self,
        // hashmap: &mut HashMap<String, String>,
        database: &mut Database,
        instructions: Vec<Instruction>,
    ) -> Result<(), String> {
        for instruction in instructions {
            match instruction {
                Instruction::Get { key } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err("No storage name provided".to_string());
                    };

                    match database.get(storage_name, key.clone()) {
                        Ok(value) => println!("Value for key '{}': {}", key, value),
                        Err(_) => return Err(format!("Key '{}' not found", key)),
                    }
                }
                Instruction::Set { key, value } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err("No storage name provided".to_string());
                    };
                    if let Err(e) = database.insert(storage_name, key.clone(), value) {
                        return Err(e);
                    }
                    println!("Set value for key '{}'", key);
                }
                Instruction::Delete { key: _ } => {
                    // if hashmap.remove(&key).is_none() {
                    //     return Err(format!("Key '{}' not found for deletion", key));
                    // }
                    // println!("Deleted key '{}'", key);
                    todo!()
                }
                Instruction::Print { key: _ } => {
                    // match hashmap.get(&key) {
                    // Some(value) => println!("{}", value),
                    // None => return Err(format!("Key '{}' not found for printing", key)),
                    // }
                }
                Instruction::GetJsonField { key, field } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err("No storage name provided".to_string());
                    };

                    match database.get(storage_name, key.clone()) {
                        Ok(value) => match serde_json::from_str::<serde_json::Value>(&value) {
                            Ok(json_value) => match json_value.get(&field) {
                                Some(field_value) => println!(
                                    "JSON field '{}' in key '{}': {}",
                                    field, key, field_value
                                ),
                                None => {
                                    return Err(format!(
                                        "JSON field '{}' not found in key '{}'",
                                        field, key
                                    ))
                                }
                            },
                            Err(_) => {
                                return Err(format!("Value for key '{}' is not valid JSON", key))
                            }
                        },
                        Err(_) => return Err(format!("Key '{}' not found", key)),
                    }
                }
                Instruction::Clear => {
                    // hashmap.clear();
                    // println!("Cleared all entries");
                    todo!()
                }
                Instruction::UseStorage { name } => {
                    self.instruction_storage_name = Some(name);
                }
                Instruction::FullScan { maybe_filter } => {
                    let Some(storage_name) = self.instruction_storage_name.clone() else {
                        return Err("No storage name provided".to_string());
                    };

                    let storage = database
                        .storages
                        .iter()
                        .find(|s| {
                            *s.name.lock().expect("Failed to lock storage").as_str() == storage_name
                        })
                        .ok_or(format!("Storage with name '{}' not found", storage_name))?;

                    for (key, value) in storage.data.lock().expect("Failed to lock storage").iter()
                    {
                        if let Some(filter) = &maybe_filter {
                            match filter {
                                Filter::Condition(condition) => {
                                    if condition(key.clone(), value.clone()) {
                                        self.output.push(value.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub enum Instruction {
    UseStorage { name: String },
    ClearOutput,
    FullScan { maybe_filter: Option<Filter> },
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Print { key: String },
    GetJsonField { key: String, field: String },
    Clear,
}

#[allow(dead_code)]
pub enum Filter {
    Condition(Box<dyn Fn(String, String) -> bool>),
}
