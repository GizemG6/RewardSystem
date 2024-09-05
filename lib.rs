use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, TokenAccount, Transfer};

declare_id!("6F5GbFBLkHQgkwswFw78VYesccwuesUV3Sibgse7zpzx");

#[program]
mod reward_system {
    use super::*;

    // Initializes a new mint and a reward account
    // Mints the initial supply to the reward account
    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        if amount == 0 {
            return err!(RewardSystemError::InvalidAmount);
        }
        // Mint the initial supply to the reward account
        mint_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.mint,
            &ctx.accounts.reward_account.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            amount,
        )?;

        msg!("Mint initialized with {} tokens.", amount);
        Ok(())
    }

    // Distributes reward tokens to a user
    // Transfers tokens from the reward account to the user's account
    pub fn distribute_reward(ctx: Context<DistributeReward>, amount: u64) -> Result<()> {
        if amount == 0 {
            return err!(RewardSystemError::InvalidAmount);
        }
        // Transfer reward tokens from the reward account to the user account
        transfer_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.reward_account.to_account_info(),
            &ctx.accounts.user_account.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            amount,
        )?;

        msg!("Distributed {} reward tokens to the user.", amount);
        Ok(())
    }

    // Tracks loan actions and calculates rewards
    // Mints reward tokens based on the loan amount
    pub fn loan_action(ctx: Context<LoanAction>, loan_amount: u64) -> Result<()> {
        if loan_amount == 0 {
            return err!(RewardSystemError::InvalidAmount);
        }
        // Calculate reward based on loan amount
        let reward = loan_amount / 10; // 10% of loan amount as reward
        mint_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.mint,
            &ctx.accounts.reward_account.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            reward,
        )?;

        msg!("Loan action recorded, {} tokens rewarded.", reward);
        Ok(())
    }

    // Allows users to stake their tokens
    // Updates the user's account with the staked amount
    pub fn stake_tokens(ctx: Context<StakeTokens>, stake_amount: u64) -> Result<()> {
        if stake_amount == 0 {
            return err!(RewardSystemError::InvalidAmount);
        }

        let user = &mut ctx.accounts.user;
        user.staked_amount = user
            .staked_amount
            .checked_add(stake_amount)
            .ok_or(RewardSystemError::Overflow)?;

        msg!("User staked {} tokens.", stake_amount);
        Ok(())
    }

    // Distributes rewards for staked tokens
    // Mints reward tokens based on the staked amount
    pub fn distribute_staking_rewards(ctx: Context<DistributeStakingRewards>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        if user.staked_amount == 0 {
            return err!(RewardSystemError::NoStakedAmount);
        }
        let reward = user.staked_amount / 10; // 10% reward for staking

        mint_tokens(
            &ctx.accounts.token_program,
            &ctx.accounts.mint,
            &ctx.accounts.user.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            reward,
        )?;
        msg!("Staking rewards distributed: {} tokens.", reward);
        Ok(())
    }

    // Initialize a new user account
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.staked_amount = 0; // Başlangıç değeri
        msg!("User account initialized.");
        Ok(())
    }
}

// Helper function to mint tokens
fn mint_tokens<'info>(
    token_program: &Program<'info, token::Token>,
    mint: &Account<'info, Mint>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: mint.to_account_info(),
        to: to.clone(),
        authority: authority.clone(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_context, amount).map_err(|_| RewardSystemError::MintingFailed.into())
}

// Helper function to transfer tokens
fn transfer_tokens<'info>(
    token_program: &Program<'info, token::Token>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from.clone(),
        to: to.clone(),
        authority: authority.clone(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context, amount).map_err(|_| RewardSystemError::TransferFailed.into())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Creates a new mint account for minting reward tokens
    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority)]
    pub mint: Account<'info, Mint>,
    // Creates a token account for holding the reward tokens
    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub reward_account: Account<'info, TokenAccount>,
    // The authority account that signs the transactions
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = authority, space = 8 + 8)]
    // 8 byte discriminator + 8 byte staked_amount
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeReward<'info> {
    // The token account holding the reward tokens
    #[account(mut)]
    pub reward_account: Account<'info, TokenAccount>,
    // The user's token account
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    // The authority account that signs the transaction
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct LoanAction<'info> {
    // The token account holding the reward tokens
    #[account(mut)]
    pub reward_account: Account<'info, TokenAccount>,
    // The mint account used for minting new tokens
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    // The authority account that signs the transactions
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    // The user account where the staked amount is stored
    #[account(mut)]
    pub user: Account<'info, User>,
    // The authority account that signs the transactions
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeStakingRewards<'info> {
    // The user account where the staked amount and rewards are stored
    #[account(mut)]
    pub user: Account<'info, User>,
    // The mint account used for minting new tokens
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    // The authority account that signs the transactions
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

// User account structure, stores the amount staked by the user
#[account]
pub struct User {
    pub staked_amount: u64,
}

#[error_code]
pub enum RewardSystemError {
    #[msg("Invalid amount provided.")]
    InvalidAmount,
    #[msg("Minting of tokens failed.")]
    MintingFailed,
    #[msg("Token transfer failed.")]
    TransferFailed,
    #[msg("Staked amount cannot be zero.")]
    NoStakedAmount,
    #[msg("Arithmetic overflow occurred.")]
    Overflow,
}
