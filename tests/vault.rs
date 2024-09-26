use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use anchor_test::ProgramTest;
use std::str::FromStr;

declare_id!("396xpdASMW1vFsX2CM9SUaTzQpRi1S5wfQupti5nNLqo");

#[tokio::test]
async fn test_vault() {
    let mut payer = Keypair::new();
    let mint_keypair = Keypair::new();
    let user_keypair = Keypair::new();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "assignment", // The name of your program
        Pubkey::from_str("396xpdASMW1vFsX2CM9SUaTzQpRi1S5wfQupti5nNLqo").unwrap(),
        processor!(assignment::process_instruction), 
    )
    .start_with_context()
    .await;

    // Create Mint Account
    let mint = Mint {
        mint_authority: Some(payer.pubkey()),
        supply: 0,
        decimals: 6,
        is_initialized: true,
        freeze_authority: None,
    };

    // Create token accounts
    let user_token_account = Keypair::new();
    let vault_token_account = Keypair::new();
    
    // Create the vault account
    let vault_account = Keypair::new();

    // Initialize mint and token accounts
    let mint_ix = token::initialize_mint(
        &token::Token::id(),
        &mint_keypair.pubkey(),
        Some(&payer.pubkey()),
        None,
        6,
    )?;

    // Create user token account
    let user_token_account_ix = token::create_account(
        &token::Token::id(),
        &user_token_account.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
    )?;

    // Create vault token account
    let vault_token_account_ix = token::create_account(
        &token::Token::id(),
        &vault_token_account.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
    )?;

    // Transaction for mint and token account creation
    let mut transaction = Transaction::new_with_payer(
        &[mint_ix, user_token_account_ix, vault_token_account_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint_keypair, &user_token_account, &vault_token_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Initialize the vault
    let initialize_ix = program::initialize(
        &payer.pubkey(),
        &vault_account.pubkey(),
        &mint_keypair.pubkey(),
        10, // Fee percentage
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[initialize_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &vault_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Deposit tokens into the vault
    let deposit_amount = 100;
    let deposit_ix = program::deposit(
        &payer.pubkey(),
        &vault_account.pubkey(),
        &user_token_account.pubkey(),
        &vault_token_account.pubkey(),
        deposit_amount,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[deposit_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check the vault balance after deposit
    let vault_data = banks_client.get_account(vault_account.pubkey()).await.unwrap();
    let vault: Vault = Vault::try_from_slice(&vault_data.data).unwrap();
    assert_eq!(vault.balance, deposit_amount);

    // Withdraw tokens from the vault
    let withdraw_amount = 50;
    let withdraw_ix = program::withdraw(
        &payer.pubkey(),
        &vault_account.pubkey(),
        &user_token_account.pubkey(),
        &vault_token_account.pubkey(),
        withdraw_amount,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[withdraw_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check the vault balance after withdrawal
    let vault_data = banks_client.get_account(vault_account.pubkey()).await.unwrap();
    let vault: Vault = Vault::try_from_slice(&vault_data.data).unwrap();
    assert_eq!(vault.balance, deposit_amount - withdraw_amount);
}
