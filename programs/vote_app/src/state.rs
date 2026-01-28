use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TreasuryConfig {
    pub authority: Pubkey,
    pub x_mint: Pubkey,                 // token to be used for voting
    pub treasury_token_account: Pubkey, //to hold account the x mint token
    pub sol_price: u64,                 // sol price for vote and propsale
    pub token_per_purchase: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    pub voter_id: Pubkey,
    pub proposal_voted: u8,}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub proposal_id: u8,
    pub number_of_votes: u64, // sol price for vote and propsale
    pub deadline: i64,
    #[max_len(50)]
    pub proposal_info: String,
    pub authority: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct ProposalCounter {
    pub authority: Pubkey,
    pub proposal_count: u8,
}
