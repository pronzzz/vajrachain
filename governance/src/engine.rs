use crate::proposal::{Proposal, ProposalState, Vote, VoteOption};
use std::collections::HashMap;

pub struct GovernanceEngine {
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
    pub quorum_threshold: u64,
}

impl GovernanceEngine {
    pub fn new(quorum_threshold: u64) -> Self {
        Self {
            proposals: HashMap::new(),
            next_proposal_id: 1,
            quorum_threshold,
        }
    }

    pub fn create_proposal(
        &mut self,
        description: String,
        call_data: Vec<u8>,
        start_epoch: u64,
        duration: u64,
    ) -> u64 {
        let id = self.next_proposal_id;
        let proposal = Proposal::new(
            id,
            description,
            call_data,
            start_epoch,
            start_epoch + duration,
        );
        self.proposals.insert(id, proposal);
        self.next_proposal_id += 1;
        id
    }

    pub fn cast_vote(
        &mut self,
        proposal_id: u64,
        voter: Vec<u8>,
        option: VoteOption,
        weight: u64,
    ) -> Result<(), String> {
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .ok_or("Proposal not found")?;

        // Simple check: active state (mocked time/epoch check)
        // In real system, check current_epoch vs start/end

        // Prevent duplicate voting (simple check)
        if proposal.votes.iter().any(|v| v.voter == voter) {
            return Err("Already voted".to_string());
        }

        proposal.votes.push(Vote {
            voter,
            option,
            weight,
        });
        Ok(())
    }

    pub fn tally_votes(&mut self, proposal_id: u64) -> Result<ProposalState, String> {
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .ok_or("Proposal not found")?;

        let mut yes_weight = 0;
        let mut no_weight = 0;
        let mut total_weight = 0;

        for vote in &proposal.votes {
            total_weight += vote.weight;
            match vote.option {
                VoteOption::Yes => yes_weight += vote.weight,
                VoteOption::No => no_weight += vote.weight,
                VoteOption::Abstain => {}
            }
        }

        // Quorum check
        if total_weight < self.quorum_threshold {
            proposal.state = ProposalState::Rejected; // Or remain Active if time left? Simplified: Rejected.
        } else if yes_weight > no_weight {
            proposal.state = ProposalState::Passed;
        } else {
            proposal.state = ProposalState::Rejected;
        }

        Ok(proposal.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_lifecycle() {
        let mut engine = GovernanceEngine::new(100); // Quorum 100
        let pid = engine.create_proposal("Test".to_string(), vec![], 1, 10);

        // Vote 1: Yes 60
        engine.cast_vote(pid, vec![1], VoteOption::Yes, 60).unwrap();
        // Vote 2: No 30
        engine.cast_vote(pid, vec![2], VoteOption::No, 30).unwrap();

        // Total 90 < 100 -> Rejected (Quorum fail)
        // Oops, let's make it pass quorum

        // Vote 3: Yes 20
        engine.cast_vote(pid, vec![3], VoteOption::Yes, 20).unwrap();
        // Total 110. Yes 80, No 30. -> Passed.

        let state = engine.tally_votes(pid).unwrap();
        assert_eq!(state, ProposalState::Passed);
    }
}
