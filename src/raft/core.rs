use crate::raft::log::RaftLog;
use crate::raft::types::Role;
use crate::raft::types::NodeId;

// TODO: leader election, consensus rules, log commitment
pub struct RaftCore {
    current_term: u64,
    voted_for: Option<NodeId>,
    log: RaftLog,
    role: Role,
}
