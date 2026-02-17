use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalState {
    Pending,
    Active,
    Passed,
    Rejected,
    Executed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteOption {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: Vec<u8>,
    pub option: VoteOption,
    pub weight: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub call_data: Vec<u8>, // Action to execute
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub state: ProposalState,
    pub votes: Vec<Vote>,
}

impl Proposal {
    pub fn new(
        id: u64,
        description: String,
        call_data: Vec<u8>,
        start_epoch: u64,
        end_epoch: u64,
    ) -> Self {
        Self {
            id,
            description,
            call_data,
            start_epoch,
            end_epoch,
            state: ProposalState::Pending,
            votes: Vec::new(),
        }
    }
}
