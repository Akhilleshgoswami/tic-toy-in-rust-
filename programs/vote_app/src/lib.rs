use anchor_lang::prelude::*;
mod contexts;
mod state;
use contexts::*;
mod errors;
use anchor_lang::system_program;
use errors::*;
declare_id!("6Xu4otAyDHAKepsZXJHwy1WRD4h7nxqwz6co8Xk4Lvuu");
#[program]
pub mod vote_app {
    use anchor_spl::token::{mint_to, transfer, MintTo, Transfer};

    use super::*;

    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        sol_price: u64,
        tokens_per_purchase: u64,
    ) -> Result<()> {
        let treasury_config_account = &mut ctx.accounts.treasury_config_account;
        treasury_config_account.authority = ctx.accounts.authority.key();
        treasury_config_account.bump = ctx.bumps.sol_vault;
        treasury_config_account.sol_price = sol_price;
        treasury_config_account.x_mint = ctx.accounts.x_mint.key();
        treasury_config_account.token_per_purchase = tokens_per_purchase;
        let proposal_counter_account = &mut ctx.accounts.proposal_counter_account;
        require!(
            proposal_counter_account.proposal_count == 0,
            VoteError::ProposalCountAlreadyInitialized
        );
        proposal_counter_account.authority = ctx.accounts.authority.key();
        proposal_counter_account.proposal_count = 1;
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<Buytokens>) -> Result<()> {
        let treasury_config_account = &mut ctx.accounts.treasury_config_account;
        let sol = treasury_config_account.sol_price;
        let token_amount = treasury_config_account.token_per_purchase;

        let transfer_ix = anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        system_program::transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_ix),
            sol,
        )?;
        let mint_authority_seeds = &[b"mint_authority".as_ref(), &[ctx.bumps.mint_authority]];
        let signer_seeds = &[&mint_authority_seeds[..]];
        let cpi_accounts = MintTo {
            mint: ctx.accounts.x_mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        mint_to(cpi_ctx, token_amount)?;

        Ok(())
    }
    pub fn register_voter(ctx: Context<RegisterVoter>) -> Result<()> {
        let voter_account = &mut ctx.accounts.voter_account;
        voter_account.voter_id = ctx.accounts.authority.key(); // public key for the user

        Ok(())
    }

    pub fn register_proposal(
        ctx: Context<RegisterProposal>,
        proposal_info: String,
        deadline: i64,
        token_amount: u64,
    ) -> Result<()> {
        let proposal_account = &mut ctx.accounts.proposal_account;
        let clock = Clock::get()?;
        require!(deadline > clock.unix_timestamp, VoteError::InvlaideDealiine);
        //transfer token from propsale token account to treasury token account
        let cpi_accounts = Transfer {
            from: ctx.accounts.proposale_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        //transfer of tokens
        transfer(cpi_ctx, token_amount)?;
        proposal_account.proposal_info = proposal_info;
        proposal_account.deadline = deadline;
        let proposal_counter_account = &mut ctx.accounts.proposal_counter_account;
        proposal_account.proposal_id = proposal_counter_account.proposal_count;
        proposal_counter_account
            .proposal_count
            .checked_add(1)
            .ok_or(VoteError::ProposalCounterOverFlow)?;
        proposal_account.authority = ctx.accounts.authority.key();

        Ok(())
    }
    pub fn proposal_to_vote(ctx: Context<Vote>, proposal_id: u8, token_amount: u64) -> Result<()> {
        let proposal_account = &mut ctx.accounts.proposal_account;
        let clock = Clock::get()?;
        require!(
            proposal_account.deadline > clock.unix_timestamp,
            VoteError::ProposalEnded
        );
        //transfer token from propsale token account to treasury token account
        let cpi_accounts = Transfer {
            from: ctx.accounts.voter_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        //transfer of tokens
        transfer(cpi_ctx, token_amount)?;
        let voter_account = &mut ctx.accounts.voter_account;
        voter_account.proposal_voted = proposal_id;
        proposal_account.number_of_votes = proposal_account
            .number_of_votes
            .checked_add(1)
            .ok_or(VoteError::ProposalCounterOverFlow)?;
        Ok(())
    }
}
