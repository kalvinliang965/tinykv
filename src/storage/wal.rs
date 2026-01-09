use crate::kv::command::Command;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Debug)]
pub enum WALError {
    FileOpenFailed,
    SerializeError,
    DeserializeError,
    WriteError,
    ReadError,
}

pub trait WAL { 
    // goes from history -> data
    fn record(&self, cmd: Command) -> Result<(), WALError>;
    fn replay(&self) -> Result<Vec<Command>, WALError>;
}

pub struct FileSystemWAL { 
    path: String,
}

impl FileSystemWAL { 
    fn new(path: &str) -> Self { 
        Self { 
            path: path.to_string()
        }
    }
}

impl WAL for FileSystemWAL { 
    fn record(&self, cmd: Command) -> Result<(), WALError> {
        let mut file = OpenOptions::new()
                .append(true)
                .open(&self.path)
                .map_err(|_| WALError::FileOpenFailed)?;
        let json = serde_json::to_string(&cmd)
                .map_err(|_| WALError::SerializeError)?;
        writeln!(file, "{}", json).map_err(|_| WALError::WriteError)?;
        Ok(())
    }
    fn replay(&self) -> Result<Vec<Command>, WALError> { 
        let file = OpenOptions::new()
                .read(true)
                .open(&self.path)
                .map_err(|_| WALError::FileOpenFailed)?;
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| {
                let line = line.map_err(|_| WALError::ReadError)?;
                let json = serde_json::from_str(&line).map_err(|_| WALError::DeserializeError)?;
                println!("{:?}", json);
                Ok(json)
            })
            .collect()
    }
}


#[cfg(test)]
mod testse { 
    use super::*;
    use std::fs::{self, File};
    use anyhow::Result;

    #[test]
    fn test_file_system_wal_replay() -> Result<()>  { 
        let path = "src/storage/replay_test";
        let mut file = File::create(&path)?;
        let json = r#"{"Get":"key"}"#;
        writeln!(file, "{}", json)?;
        let wal = FileSystemWAL::new(&path);
        let v = wal.replay();
        assert_eq!(Ok(vec![Command::Get("key".to_string())]), v);
        fs::remove_file(&path)?;
        Ok(())
    }

    #[test]
    fn test_file_system_wal_record() -> Result<()>  { 
        let path = "src/storage/record_test";
        let file = File::create(&path)?;
        let wal = FileSystemWAL::new(&path);
        let exp = vec![
            Command::Get("key".to_string()),
            Command::Get("key1".to_string()),
            Command::Get("key2".to_string()),
            Command::Put("key1".to_string(), "val1".to_string()),
            Command::Put("key2".to_string(), "val2".to_string()),
            Command::Put("key3".to_string(), "val3".to_string()),
        ];
        for e in &exp { 
            wal.record(e.clone());
        }
        let act = wal.replay();
        assert_eq!(Ok(exp), act);
        fs::remove_file(&path)?;
        Ok(())
    }
}
