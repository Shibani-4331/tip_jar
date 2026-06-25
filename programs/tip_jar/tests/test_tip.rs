use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_tip() {
    let program_id = tip_jar::id();
    let owner = Keypair::new();
    let tipper = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/tip_jar.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&owner.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&tipper.pubkey(), 1_000_000_000).unwrap();

    let (tip_jar_pda, _bump) = anchor_lang::prelude::Pubkey::find_program_address(
        &[b"tip_jar", owner.pubkey().as_ref()],
        &program_id,
    );

    
    let init_ix = Instruction::new_with_bytes(
        program_id,
        &tip_jar::instruction::Initialize {}.data(),
        tip_jar::accounts::Initialize {
            tip_jar: tip_jar_pda,
            owner: owner.pubkey(),
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let init_msg = Message::new_with_blockhash(&[init_ix], Some(&owner.pubkey()), &blockhash);
    let init_tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(init_msg), &[&owner]).unwrap();
    svm.send_transaction(init_tx).unwrap();

    let tip_amount: u64 = 500_000_000; // 0.5 SOL
    let tip_ix = Instruction::new_with_bytes(
        program_id,
        &tip_jar::instruction::Tip { amount: tip_amount }.data(),
        tip_jar::accounts::Tip {
            tip_jar: tip_jar_pda,
            tipper: tipper.pubkey(),
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let tip_msg = Message::new_with_blockhash(&[tip_ix], Some(&tipper.pubkey()), &blockhash);
    let tip_tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(tip_msg), &[&tipper]).unwrap();
    let res = svm.send_transaction(tip_tx);
    assert!(res.is_ok());

    let account = svm.get_account(&tip_jar_pda).unwrap();
    let tip_jar_data: tip_jar::TipJar =
        anchor_lang::AccountDeserialize::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(tip_jar_data.total_tips, tip_amount);
}