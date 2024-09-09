use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::system_program;
use anchor_lang::Discriminator;
use anchor_lang::InstructionData;
use bytemuck::Zeroable;
use solana_program_test::tokio;
use solana_program_test::{anchor_processor, ProgramTest};
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_option::COption;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::SysvarId;
use solana_sdk::transaction::Transaction;
use vprog::{CollateralInfos, GlobalConfig, WhirlpoolStrategy};

#[tokio::test]
async fn test_ix_failure() {
    let mut program_test = ProgramTest::new("vprog", vprog::ID, anchor_processor!(vprog::entry));
    // Disable loose cpi instruction requirement (it's not activated on mainnet)
    program_test.deactivate_feature(solana_sdk::pubkey!(
        "GDH5TVdbTPUpRnXaRyQqiKUa7uZAbZ28Q2N9bhbKoMLm"
    ));
    program_test.set_compute_max_units(1_400_000);

    let admin = Keypair::new();
    program_test.add_account(
        admin.pubkey(),
        Account::new(10_000_000_000, 0, &Pubkey::default()),
    );

    let strategy_pk = Pubkey::new_unique();
    program_test.add_account(
        strategy_pk,
        Account::new(
            u32::MAX as u64,
            size_of::<WhirlpoolStrategy>() + 8,
            &vprog::ID,
        ),
    );

    let pool_pk = Pubkey::new_unique();
    program_test.add_account(pool_pk, Account::new(10_000_000_000, 0, &Pubkey::default()));

    let token_infos_pk = Pubkey::new_unique();
    let collateral_infos_data: Vec<u8> = CollateralInfos::DISCRIMINATOR
        .iter()
        .chain(bytemuck::bytes_of(&CollateralInfos::zeroed()))
        .copied()
        .collect();
    program_test.add_account(
        token_infos_pk,
        Account {
            lamports: u32::MAX as u64,
            data: collateral_infos_data,
            owner: vprog::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let global_config_pk = Pubkey::new_unique();
    let global_config = GlobalConfig {
        admin_authority: admin.pubkey(),
        token_infos: token_infos_pk,
        ..Default::default()
    };
    let global_config_data: Vec<u8> = GlobalConfig::DISCRIMINATOR
        .iter()
        .chain(bytemuck::bytes_of(&global_config).iter())
        .copied()
        .collect();
    program_test.add_account(
        global_config_pk,
        Account {
            lamports: u32::MAX as u64,
            data: global_config_data,
            owner: vprog::ID,
            executable: false,
            rent_epoch: 0,
        },
    );

    let token_a_mint_pk = Pubkey::new_unique();
    program_test.add_account(
        token_a_mint_pk,
        mint_account(Some(admin.pubkey()), 1_000_000_000, 6),
    );

    let token_b_mint_pk = Pubkey::new_unique();
    program_test.add_account(
        token_b_mint_pk,
        mint_account(Some(admin.pubkey()), 1_000_000_000, 6),
    );

    let token_a_vault_pk = Pubkey::find_program_address(
        &[b"STRATEGY_VAULT_A_SEED", strategy_pk.as_ref()],
        &vprog::ID,
    )
    .0;
    let token_b_vault_pk = Pubkey::find_program_address(
        &[b"STRATEGY_VAULT_B_SEED", strategy_pk.as_ref()],
        &vprog::ID,
    )
    .0;

    let base_vault_authority = Pubkey::new_unique();
    program_test.add_account(
        base_vault_authority,
        Account::new(10_000_000_000, 0, &Pubkey::default()),
    );

    let shares_mint_pk = Pubkey::find_program_address(
        &[
            b"SHARES_SEEDS",
            strategy_pk.as_ref(),
            token_a_mint_pk.as_ref(),
            token_b_mint_pk.as_ref(),
        ],
        &vprog::ID,
    )
    .0;

    let shares_mint_authority = Pubkey::new_unique();
    program_test.add_account(
        shares_mint_authority,
        Account::new(10_000_000_000, 0, &Pubkey::default()),
    );

    let mut ctx = program_test.start_with_context().await;

    let accounts = vprog::accounts::InitializeStrategy {
        admin_authority: admin.pubkey(),
        global_config: global_config_pk,
        pool: pool_pk,
        token_a_mint: token_a_mint_pk,
        token_b_mint: token_b_mint_pk,
        token_a_vault: token_a_vault_pk,
        token_b_vault: token_b_vault_pk,
        shares_mint: shares_mint_pk,
        token_infos: token_infos_pk,
        system_program: system_program::ID,
        rent: Rent::id(),
        token_program: spl_token::id(),
        token_a_token_program: spl_token::id(),
        token_b_token_program: spl_token::id(),
        strategy: strategy_pk,
        shares_mint_authority,
        base_vault_authority,
    };

    let data = vprog::instruction::InitializeStrategy {
        _a: 0,
        _b: 1,
        _c: 2,
    };

    let ix = Instruction {
        program_id: vprog::ID,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let latest_blockhash = ctx.get_new_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&admin.pubkey()),
        &[&admin],
        latest_blockhash,
    );
    ctx.banks_client
        .process_transaction_with_commitment(tx, CommitmentLevel::Processed)
        .await
        .unwrap();
}

fn mint_account(authority: Option<Pubkey>, supply: u64, decimals: u8) -> Account {
    let mut data_slice = [0_u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint {
        mint_authority: authority.into(),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: COption::None,
    }
    .pack_into_slice(&mut data_slice);
    Account {
        lamports: u32::MAX as u64,
        data: data_slice.to_vec(),
        owner: spl_token::id(),
        executable: false,
        rent_epoch: u64::MAX,
    }
}
