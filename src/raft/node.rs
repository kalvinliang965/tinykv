use crate::raft::core::RaftCore;
use crate::raft::types::NodeId;

pub struct RaftNode {
    id: NodeId,
    core: RaftCore,
}
