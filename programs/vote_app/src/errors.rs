use anchor_lang::prelude::*;
#[error_code]
pub enum VoteError {
    #[msg("Invlaide deadline passed")]
    InvlaideDealiine,

    #[msg("proposal counter already initialized")]
    ProposalCountAlreadyInitialized,

    #[msg("proposal counter overflowed")]
    ProposalCounterOverFlow,
    #[msg("Proposal Ended")]
    ProposalEnded
}
