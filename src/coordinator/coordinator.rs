use crate::kv::command::Command;
use crate::kv::engine::{Store, InMemoryStore};
use crate::storage::wal::{WAL, WALError, FileSystemWAL};

#[derive(PartialEq, Debug)]
pub enum CoordinatorError { 
    WAL(WALError),

    // the following error msg are unused for now

    // Engine(EngineError)
    // Rejected,
    // Unavailable,
}

pub struct Coordinator<W: WAL, S: Store> { 
    wal: W,
    store: S,
}

impl <W: WAL, S: Store> Coordinator<W, S> { 
    fn new (wal: W, store: S) -> Self {
        Self { 
            wal,
            store,
        }
    }

    pub fn submit(&mut self, cmd: Command) -> Result<(), CoordinatorError> {
        // 1. try to appy to wal
        self.wal.record(cmd.clone()).map_err(|e| CoordinatorError::WAL(e))?;
        // 2. try to appy to store
        let _ = self.store.apply(cmd.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    use std::fs::{self, File};
    use anyhow::Result;

    #[test]
    fn test_coordinator_submit_get() -> Result<()> { 
        let path = "src/coordinator/test";
        let file = File::create(&path)?;
        let mut coordinator = Coordinator::new(FileSystemWAL::new(&path), InMemoryStore::new());
        let cmd = Command::Put("key".to_string(), "value".to_string());
        assert_eq!(Ok(()), coordinator.submit(cmd.clone()));
        assert_eq!(Some("value".to_string()), coordinator.store.apply(Command::Get("key".to_string())));
        assert_eq!(Ok(vec![cmd.clone()]), coordinator.wal.replay());
        fs::remove_file(&path)?;
        Ok(())
    }
}

