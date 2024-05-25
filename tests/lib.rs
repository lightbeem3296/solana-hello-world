use borsh::BorshDeserialize;
use helloworld::{process_instruction, GreetingAccount};
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    // some unique pubkey for our program
    let program_id = Pubkey::new_unique();
    // and its storage
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo-test`
    );

    // create storage account
    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,                             // should be enough for storing single u32
            data: vec![0_u8; mem::size_of::<u32>()], // data is vector of zeroed bytes enough to store u32
            owner: program_id,                       // owner is our program_id
            ..Account::default() // everything else is taken from https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");

    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        0
    );

    // Greet once tx
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greeted_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
