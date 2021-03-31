use std::{collections::HashMap};
use bytes::Bytes;
use mini_redis::Command;
use std::sync::{Arc, Mutex};
pub struct MemoryDb {
    db: Arc<Mutex<HashMap<String, Bytes>>>,
}

#[derive(Debug)]
pub enum CmdResult {
    Get(GetResult),
    None,
}

#[derive(Debug)]
pub struct GetResult {
    pub key: String,
    pub value: Option<Bytes>,
}

#[derive(Debug)]
pub enum CmdError {
    Internal,
    Unimplemented,
}

impl MemoryDb {
    pub fn new() -> Self {
        Self {
            db: Default::default(),
        }
    }

    pub fn process(&self, cmd: Command) -> Result<CmdResult, CmdError> {
        let mut db = self.db.lock().unwrap();
        match cmd {
            Command::Get(cmd) => Ok(CmdResult::Get(GetResult {
                key: cmd.key().into(),
                value: db.get(cmd.key()).map(|v| v.clone()),
            })),
            Command::Set(cmd) => {
                db.insert(cmd.key().into(), cmd.value().clone());
                Ok(CmdResult::None)
            }
            _ => Err(CmdError::Unimplemented),
        }
    }
}


impl Clone for MemoryDb {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone()
        }
    }
}