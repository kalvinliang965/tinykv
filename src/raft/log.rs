use crate::kv::command::Command;
use crate::raft::types::Term;
use crate::raft::types::LogIndex;

pub struct LogEntry {
    pub term: Term,
    pub command: Command,
}

pub struct RaftLog {
    entries: Vec<LogEntry>,
    commit_index: LogIndex,
    last_applied: LogIndex,
}
