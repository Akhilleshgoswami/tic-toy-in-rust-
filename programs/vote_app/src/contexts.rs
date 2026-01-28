use anchor_lang::prelude::*;

use crate::errors::VoteError;
use crate::state::Proposal;
use crate::state::ProposalCounter;
use crate::state::TreasuryConfig;
use crate::state::Voter;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer=authority,
        space = 8 +  TreasuryConfig::INIT_SPACE,
        seeds=[b"treasury_config"],bump
    )]
    pub treasury_config_account: Account<'info, TreasuryConfig>,
    #[account(init,payer=authority,mint::authority=mint_authority,mint::decimals=6,  seeds=[b"x_mint"],bump)]
    pub x_mint: Account<'info, Mint>,

    #[account(init,payer=authority,associated_token::mint=x_mint,associated_token::authority = authority  )]
    pub treasury_token_account: Account<'info, TokenAccount>, // to hold token
    ///CHECK : This is to receive sol token
    #[account(mut,seeds=[b"sol_vault"],bump)]
    pub sol_vault: AccountInfo<'info>,
    ///CHECK: this is going to be the mint authority of x_mint tokens
    #[account(seeds=[b"mint_authority"],bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(
        init,
        payer=authority,
        space = 8 +  ProposalCounter::INIT_SPACE,
        seeds=[b"proposal_counter"],bump
    )]
    pub proposal_counter_account: Account<'info, ProposalCounter>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Buytokens<'info> {
    #[account(
        seeds=[b"treasury_config"],bump,
        constraint=treasury_config_account.x_mint == x_mint.key()
    )]
    pub treasury_config_account: Account<'info, TreasuryConfig>,
    ///CHECK : This is to receive sol token
    #[account(mut,seeds=[b"sol_vault"],bump = treasury_config_account.bump)]
    pub sol_vault: AccountInfo<'info>,
    ///CHECK : This is to receive sol token
    #[account(mut)]
    pub x_mint: Account<'info, Mint>,
    ///CHECK : This is to receive sol token
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>, // to hold token
    ///CHECK : This is to receive sol token
    #[account(mut,
       constraint=buyer_token_account.owner == buyer.key(),
       constraint = buyer_token_account.mint == x_mint.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, // to hold token
    ///CHECK : This is to receive sol token
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    ///CHECK : This is to receive sol token
    #[account(seeds=[b"mint_authority"],bump)]
    pub mint_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer=authority,
        space = 8 + Voter ::INIT_SPACE,
        seeds=[b"voter",authority.key.as_ref()],bump
    )]
    pub voter_account: Account<'info, Voter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer=authority,
        space = 8 +  Proposal::INIT_SPACE,
        seeds=[b"proposal",proposal_counter_account.proposal_count.to_le_bytes().as_ref()],bump
    )]
    pub proposal_account: Account<'info, Proposal>,
    #[account(mut)]
    pub proposal_counter_account: Account<'info, ProposalCounter>,

    pub x_mint: Account<'info, Mint>,

    #[account(mut,constraint=proposale_token_account.mint == x_mint.key(),constraint=proposale_token_account.owner == authority.key())]
    pub proposale_token_account: Account<'info, TokenAccount>, // to hold token
    #[account(mut,constraint=treasury_token_account.mint == x_mint.key())]
    pub treasury_token_account: Account<'info, TokenAccount>, // to hold token
    ///CHECK: this is going to be the mint authority of x_mint tokens
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(proposal_id:u8)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds =[b"voter",authority.key().as_ref()],bump,
        constraint = voter_account.proposal_voted == 0  //One voter One Vote
     )]
    pub voter_account: Account<'info, Voter>,

    pub x_mint: Account<'info, Mint>,

    #[account(
         mut,
         constraint = voter_token_account.mint == x_mint.key()  ,
         constraint = voter_token_account.owner == authority.key())]
    pub voter_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == x_mint.key())]
       pub treasury_token_account: Account<'info, TokenAccount>,
   
    #[account(mut, seeds =[b"proposal",proposal_id.to_be_bytes().as_ref()],bump)]
    pub proposal_account: Account<'info, Proposal>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    
}
