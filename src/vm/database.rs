use std::collections::HashMap;

use serde::{Serialize, Deserialize, Deserializer};

use vm::contexts::GlobalContext;
use vm::contracts::Contract;
use vm::errors::{Error, InterpreterResult as Result};
use vm::types::{Value, TypeSignature, TupleTypeSignature, AtomTypeIdentifier};

pub trait DataMap {
    fn fetch_entry(&self, key: &Value) -> Result<Value>;
    fn set_entry(&mut self, key: Value, value: Value) -> Result<()>;
    fn insert_entry(&mut self, key: Value, value: Value) -> Result<Value>;
    fn delete_entry(&mut self, key: &Value) -> Result<Value>;
}

pub trait ContractDatabase {
    fn get_data_map(&self, map_name: &str) -> Option<&DataMap>;
    fn get_mut_data_map(&mut self, map_name: &str) -> Option<&mut DataMap>;
    fn create_map(&mut self, map_name: &str, key_type: TupleTypeSignature, value_type: TupleTypeSignature);
}

#[derive(Serialize, Deserialize)]
pub struct MemoryDataMap {
    map: HashMap<Value, Value>,
    key_type: TypeSignature,
    value_type: TypeSignature
}

#[derive(Serialize, Deserialize)]
pub struct MemoryContractDatabase {
    maps: HashMap<String, MemoryDataMap>,
}

impl MemoryDataMap {
    pub fn new(key_type: TupleTypeSignature,
               value_type: TupleTypeSignature) -> MemoryDataMap {
        MemoryDataMap {
            map: HashMap::new(),
            key_type: TypeSignature::new_atom(AtomTypeIdentifier::TupleType(key_type)),
            value_type: TypeSignature::new_atom(AtomTypeIdentifier::TupleType(value_type))
        }
    }
}

impl MemoryContractDatabase {
    pub fn new() -> MemoryContractDatabase {
        MemoryContractDatabase { maps: HashMap::new() }
    }
}

impl ContractDatabase for MemoryContractDatabase {
    fn get_mut_data_map(&mut self, map_name: &str) -> Option<&mut DataMap> {
        if let Some(data_map) = self.maps.get_mut(map_name) {
            Some(data_map)
        } else {
            None
        }
    }

    fn get_data_map(&self, map_name: &str) -> Option<&DataMap> {
        if let Some(data_map) = self.maps.get(map_name) {
            Some(data_map)
        } else {
            None
        }
    }

    fn create_map(&mut self, map_name: &str, key_type: TupleTypeSignature, value_type: TupleTypeSignature) {
        let new_map = MemoryDataMap::new(key_type, value_type);
        self.maps.insert(map_name.to_string(), new_map);
    }
}

impl DataMap for MemoryDataMap {
    // TODO: currently, the return types and behavior of these functions are defined here,
    //   however, they should really be specified in the functions/database.rs file, whereas
    //   this file should really just be speccing out the database connection/requirement.

    fn fetch_entry(&self, key: &Value) -> Result<Value> {
        if !self.key_type.admits(key) {
            return Err(Error::TypeError(format!("{:?}", self.key_type), (*key).clone()))
        }
        if let Some(value) = self.map.get(key) {
            return Ok((*value).clone())
        } else {
            return Ok(Value::Void)
        }
    }

    fn set_entry(&mut self, key: Value, value: Value) -> Result<()> {
        if !self.key_type.admits(&key) {
            return Err(Error::TypeError(format!("{:?}", self.key_type), key))
        }
        if !self.value_type.admits(&value) {
            return Err(Error::TypeError(format!("{:?}", self.value_type), value))
        }
        self.map.insert(key, value);
        Ok(())
    }

    fn insert_entry(&mut self, key: Value, value: Value) -> Result<Value> {
        if !self.key_type.admits(&key) {
            return Err(Error::TypeError(format!("{:?}", self.key_type), key))
        }
        if !self.value_type.admits(&value) {
            return Err(Error::TypeError(format!("{:?}", self.value_type), value))
        }
        if self.map.contains_key(&key) {
            Ok(Value::Bool(false))
        } else {
            self.map.insert(key, value);
            Ok(Value::Bool(true))
        }
    }

    fn delete_entry(&mut self, key: &Value) -> Result<Value> {
        if !self.key_type.admits(key) {
            return Err(Error::TypeError(format!("{:?}", self.key_type), (*key).clone()))
        }
        if let Some(_value) = self.map.remove(key) {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    }
}

