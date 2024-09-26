use anchor_lang::prelude::*;
use anchor_spl::token::{Token};

declare_id!("J7ysaPjiecQsUpWeGj8ViQGjvXGJF5zRiC4pbvKWEh57");

#[error_code]
pub enum VaultError {
    InsufficientBalance,
    InvalidTokenMint,
    FeeCalculationOverflow,
}

#[program]
pub mod assignment {
    use super::*;
    use anchor_spl::token::{self, Transfer};

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.token_mint = ctx.accounts.token_mint.key();

        // FeeCalculationOverflow
        if fee > 100 {
            return Err(VaultError::FeeCalculationOverflow.into());
        }

        vault.fee = fee;
        vault.balance = 0; // Initialize balance
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user_vault = &mut ctx.accounts.user_vault; // Access the user's vault account
        let user_token_account: anchor_spl::token::TokenAccount = 
        anchor_spl::token::TokenAccount::try_deserialize(&mut &ctx.accounts.user_token_account.data.borrow()[..])?;

        // Ensure the token mint is valid
        if vault.token_mint != user_token_account.mint {
            return Err(VaultError::InvalidTokenMint.into());
        }
    
        // Create the CPI context for the token transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        // Perform the token transfer
        token::transfer(cpi_ctx, amount)?;

        // Update the user's deposited balance and the vault's total balance
        user_vault.balance += amount;
        vault.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user_vault = &mut ctx.accounts.user_vault; // Access the user's vault account
        let fee_amount = (amount * vault.fee) / 100;
        let net_amount = amount - fee_amount;

        // Check if the user has sufficient balance to withdraw
        if user_vault.balance < net_amount {
            return Err(VaultError::InsufficientBalance.into());
        }

        // Create the CPI context for the token transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),  // Vault owner as authority
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the token transfer
        token::transfer(cpi_ctx, net_amount)?;

        // Update the user's balance and the vault's total balance
        user_vault.balance -= net_amount;
        vault.balance -= net_amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint: AccountInfo<'info>, 
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user_vault: Account<'info, UserVault>,  // New user-specific vault account
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>, 
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>, 
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user_vault: Account<'info, UserVault>,  // New user-specific vault account
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>, 
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>, 
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub balance: u64,
    pub fee: u64,
}

#[account]
pub struct UserVault {
    pub balance: u64,  // Individual user's balance
}
