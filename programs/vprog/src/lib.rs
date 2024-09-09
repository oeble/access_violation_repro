#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use anchor_spl::token_interface::{
    Mint as MintInterface, TokenAccount as TokenAccountInterface, TokenInterface,
};
use bytemuck::Zeroable;

declare_id!("E6qbhrt4pFmCotNUSSEh6E5cRQCEJpMcd79Z56EG9KY");

#[program]
pub mod bug_program {

    use super::*;

    pub fn initialize_strategy(
        _ctx: Context<InitializeStrategy>,
        _a: u64,
        _b: u64,
        _c: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStrategy<'info> {
    #[account(mut)]
    pub admin_authority: Signer<'info>,

    #[account(
        has_one = token_infos
    )]
    pub global_config: AccountLoader<'info, GlobalConfig>,

    pub pool: AccountInfo<'info>,

    #[account(mint::token_program = token_a_token_program)]
    pub token_a_mint: InterfaceAccount<'info, MintInterface>,
    #[account(mint::token_program = token_b_token_program)]
    pub token_b_mint: InterfaceAccount<'info, MintInterface>,

    #[account(init,
        seeds = [b"STRATEGY_VAULT_A_SEED", strategy.key().as_ref()],
        bump,
        payer = admin_authority,
        token::mint = token_a_mint,
        token::authority = base_vault_authority,
        token::token_program = token_a_token_program
    )]
    pub token_a_vault: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(init,
        seeds = [b"STRATEGY_VAULT_B_SEED", strategy.key().as_ref()],
        bump,
        payer = admin_authority,
        token::mint = token_b_mint,
        token::authority = base_vault_authority,
        token::token_program = token_b_token_program
    )]
    pub token_b_vault: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(mut)]
    pub base_vault_authority: AccountInfo<'info>,

    #[account(init,
        seeds=[b"SHARES_SEEDS", strategy.key().as_ref(), token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump,
        payer = admin_authority,
        mint::decimals = 6,
        mint::authority = shares_mint_authority
    )]
    pub shares_mint: Account<'info, Mint>,

    #[account(mut)]
    pub shares_mint_authority: AccountInfo<'info>,

    pub token_infos: AccountLoader<'info, CollateralInfos>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub token_a_token_program: Interface<'info, TokenInterface>,
    pub token_b_token_program: Interface<'info, TokenInterface>,
    #[account(zero)]
    pub strategy: AccountLoader<'info, WhirlpoolStrategy>,
}

#[account(zero_copy)]
pub struct WhirlpoolStrategy {
    pub admin_authority: Pubkey,

    pub global_config: Pubkey,

    pub base_vault_authority: Pubkey,
    pub base_vault_authority_bump: u64,

    pub pool: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,

    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_metadata: Pubkey,
    pub position_token_account: Pubkey,

    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub deprecated_0: [Pubkey; 2],
    pub deprecated_1: [u64; 2],

    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_mint_decimals: u64,
    pub token_b_mint_decimals: u64,

    pub token_a_amounts: u64,
    pub token_b_amounts: u64,

    pub token_a_collateral_id: u64,
    pub token_b_collateral_id: u64,

    pub scope_prices: Pubkey,
    pub deprecated_2: Pubkey,

    pub shares_mint: Pubkey,
    pub shares_mint_decimals: u64,
    pub shares_mint_authority: Pubkey,
    pub shares_mint_authority_bump: u64,
    pub shares_issued: u64,

    pub status: u64,

    pub reward_0_amount: u64,
    pub reward_0_vault: Pubkey,
    pub reward_0_collateral_id: u64,
    pub reward_0_decimals: u64,

    pub reward_1_amount: u64,
    pub reward_1_vault: Pubkey,
    pub reward_1_collateral_id: u64,
    pub reward_1_decimals: u64,

    pub reward_2_amount: u64,
    pub reward_2_vault: Pubkey,
    pub reward_2_collateral_id: u64,
    pub reward_2_decimals: u64,

    pub deposit_cap_usd: u64,

    pub fees_a_cumulative: u64,
    pub fees_b_cumulative: u64,
    pub reward_0_amount_cumulative: u64,
    pub reward_1_amount_cumulative: u64,
    pub reward_2_amount_cumulative: u64,

    pub deposit_cap_usd_per_ixn: u64,

    pub withdrawal_cap_a: WithdrawalCaps,
    pub withdrawal_cap_b: WithdrawalCaps,

    pub max_price_deviation_bps: u64,

    pub swap_vault_max_slippage_bps: u32,

    pub swap_vault_max_slippage_from_reference_bps: u32,

    pub strategy_type: u64,

    pub padding_0: u64,

    pub withdraw_fee: u64,
    pub fees_fee: u64,
    pub reward_0_fee: u64,
    pub reward_1_fee: u64,
    pub reward_2_fee: u64,

    pub position_timestamp: u64,
    pub kamino_rewards: [KaminoRewardInfo; 3],

    pub strategy_dex: u64,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub allow_deposit_without_invest: u64,
    pub raydium_pool_config_or_base_vault_authority: Pubkey,

    pub deposit_blocked: u8,

    pub creation_status: u8,
    pub invest_blocked: u8,
    pub share_calculation_method: u8,
    pub withdraw_blocked: u8,
    pub reserved_flag_2: u8,
    pub local_admin_blocked: u8,
    pub flash_vault_swap_allowed: u8,

    pub reference_swap_price_a: Price,
    pub reference_swap_price_b: Price,

    pub is_community: u8,
    pub rebalance_type: u8,
    pub padding_1: [u8; 6],
    pub rebalance_raw: RebalanceRaw,
    pub padding_2: [u8; 7],

    pub token_a_fees_from_rewards_cumulative: u64,
    pub token_b_fees_from_rewards_cumulative: u64,
    pub strategy_lookup_table: Pubkey,

    pub last_swap_uneven_step_timestamp: u64,

    pub farm: Pubkey,
    pub rebalances_cap: WithdrawalCaps,
    pub swap_uneven_authority: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,

    pub pending_admin: Pubkey,
    pub padding_3: u64,
    pub padding_4: [u128; 13],
    pub padding_5: [u128; 32],
    pub padding_6: [u128; 32],
    pub padding_7: [u128; 32],
}

#[account(zero_copy)]
pub struct GlobalConfig {
    pub emergency_mode: u64,
    pub block_deposit: u64,
    pub block_invest: u64,
    pub block_withdraw: u64,
    pub block_collect_fees: u64,
    pub block_collect_rewards: u64,
    pub block_swap_rewards: u64,
    pub block_swap_uneven_vaults: u32,
    pub block_emergency_swap: u32,
    pub min_withdrawal_fee_bps: u64,
    pub scope_program_id: Pubkey,
    pub scope_price_id: Pubkey,

    pub swap_rewards_discount_bps: [u64; 256],

    pub actions_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub treasury_fee_vaults: [Pubkey; 256],

    pub token_infos: Pubkey,
    pub block_local_admin: u64,
    pub min_performance_fee_bps: u64,
    pub min_swap_uneven_slippage_tolerance_bps: u64,
    pub min_reference_price_slippage_tolerance_bps: u64,

    pub actions_after_rebalance_delay_seconds: u64,

    pub treasury_fee_vault_receiver: Pubkey,
    pub _padding: [u64; 2035],
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig::zeroed()
    }
}

#[account(zero_copy)]
pub struct CollateralInfos {
    pub infos: [CollateralInfo; 256],
}

#[zero_copy]
pub struct CollateralInfo {
    pub mint: Pubkey,
    pub lower_heuristic: u64,
    pub upper_heuristic: u64,
    pub exp_heuristic: u64,
    pub max_twap_divergence_bps: u64,

    pub scope_twap_price_chain: [u16; 4],

    pub scope_price_chain: [u16; 4],
    pub name: [u8; 32],
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub max_ignorable_amount_as_reward: u64,
    pub disabled: u8,
    pub _padding0: [u8; 7],
    pub scope_staking_rate_chain: [u16; 4],
    pub _padding: [u64; 8],
}

#[zero_copy]
pub struct WithdrawalCaps {
    pub config_capacity: i64,
    pub current_total: i64,
    pub last_interval_start_timestamp: u64,
    pub config_interval_length_seconds: u64,
}

#[zero_copy]
pub struct Price {
    pub price: u64,
    pub exp: u64,
}

#[zero_copy]
pub struct KaminoRewardInfo {
    pub decimals: u64,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_collateral_id: u64,

    pub last_issuance_ts: u64,
    pub reward_per_second: u64,
    pub amount_uncollected: u64,
    pub amount_issued_cumulative: u64,
    pub amount_available: u64,
}

#[zero_copy]
pub struct RebalanceRaw {
    pub params: [u8; 128],
    pub state: [u8; 256],
    pub reference_price_type: u8,
}
