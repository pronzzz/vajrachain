use ledger::block::BlockHeader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid block signature")]
    InvalidSignature,
    #[error("Validator not found")]
    UnknownValidator,
    #[error("Validator is slashed")]
    SlashedValidator,
}

pub trait ConsensusEngine {
    /// Verifies that a block header is valid according to the consensus rules.
    fn verify_block_header(&self, header: &BlockHeader) -> Result<(), ConsensusError>;

    /// Given a list of fork tips/headers, chooses the best one (fork choice rule).
    fn choose_fork<'a>(&self, forks: &'a [BlockHeader]) -> Option<&'a BlockHeader>;
}
