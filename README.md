# **Vault Smart Contract**
This repository contains a smart contract written in Rust using the Anchor framework for the Solana blockchain. The contract implements a simple vault system that allows users to deposit and withdraw tokens while applying a fee on withdrawals.
**Overview**
The Vault smart contract allows users to:
*Initialize a vault with a specified token mint and fee.
*Deposit tokens into the vault without incurring any fees.
*Withdraw tokens from the vault, where a fee is deducted from the withdrawal amount.

**Features**
*Token Management: Supports deposits and withdrawals of SPL tokens.
*Fee System: Implements a configurable fee structure for withdrawals.
*Error Handling: Provides clear error codes for common issues such as insufficient balance and invalid token mint.

**Error Codes**
The contract defines several error codes that can be returned during execution:

`#[error_code]
pub enum VaultError {
    InsufficientBalance,
    InvalidTokenMint,
    FeeCalculationOverflow,
}`

**Error Descriptions**
*InsufficientBalance: Returned when a user attempts to withdraw more than their available balance after fees.
*InvalidTokenMint: Returned when the token mint of the user's account does not match the vault's token mint.
*FeeCalculationOverflow: Returned when the specified fee exceeds 100%.

### Smart Contract Functions

**Initialize**

`pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()>`

Initializes a new vault with the specified fee.
Parameters:
*ctx: Context containing accounts required for initialization.
*fee: The percentage fee to apply on withdrawals (must be <= 100).

**Deposit**

`pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>`

Allows users to deposit tokens into the vault.
Parameters:
*ctx: Context containing accounts required for deposit.
*amount: The amount of tokens to deposit.

**Withdraw**

`pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>`

Allows users to withdraw tokens from the vault, deducting a fee.
Parameters:
*ctx: Context containing accounts required for withdrawal.
*amount: The total amount of tokens to withdraw (fee will be deducted).

### Accounts Structure
The contract defines three main account structures:

**Initialize**

`#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_mint: AccountInfo<'info>, 
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}`

**Deposit**

`#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>, 
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>, 
    pub token_program: Program<'info, Token>,
}`

**Withdraw**

`#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>, 
    #[account(mut)]
    pub vault_token_account: AccountInfo<'info>, 
    pub token_program: Program<'info, Token>,
}`

**Vault Struct**

`#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub balance: u64,
    pub fee: u64,
}`

### Deployment and Usage
To deploy this smart contract on the Solana blockchain, follow these steps:

*Install Rust and Anchor framework.
*Clone this repository.
*Navigate to the project directory.
*Run anchor build to compile the smart contract.
*Deploy using anchor deploy.

Refer to the Anchor documentation for detailed instructions on deploying and interacting with Solana smart contracts.