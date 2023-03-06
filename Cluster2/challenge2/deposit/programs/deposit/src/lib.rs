use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    dex::{
        self, cancel_order_v2,
        serum_dex::{
            instruction::SelfTradeBehavior,
            matching::{OrderType, Side},
        },
        CancelOrderV2, Dex, NewOrderV3,
    },
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer},
};
use std::num::NonZeroU64;

declare_id!("GWZPQsbLKGnB8rQptKGDFsZ3PtbsYPFxhXm8NyhoftbY");

// Got the implementation from Richard, rewrote everything to get familiar with the proper way
// of initializing, sending and withdrawing tokens
#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let system_program = &ctx.accounts.system_program;

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.deposit_auth.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let system_program = &ctx.accounts.system_program;
        let deposit_account = &ctx.accounts.deposit_account;
        let pda_auth = &mut ctx.accounts.pda_auth;
        let sol_vault = &mut ctx.accounts.sol_vault;

        let cpi_accounts = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.deposit_auth.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];
        let signer = &[&seeds[..]];

        let cpi =
            CpiContext::new_with_signer(system_program.to_account_info(), cpi_accounts, signer);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.pda_auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn new_order(ctx: Context<NewOrder>, limit_price: NonZeroU64) -> Result<()> {
        let dex_program = ctx.accounts.dex_program.to_account_info();

        let side: Side = Side::Ask;
        let max_coin_qty = NonZeroU64::new(1000000000000000000).unwrap();
        let max_native_pc_qty_including_fees = NonZeroU64::new(1000000000000000000).unwrap();
        let self_trade_behavior = SelfTradeBehavior::DecrementTake;
        let order_type = OrderType::Limit;
        let client_order_id = 0;
        let limit = 100u16;

        // I didn't manage to complete the coding challenge with the following line:
        // let accounts: NewOrderV3<'static> = ctx;
        // because ctx.accounts.into() was trying to consume the "ctx.accounts" (take ownership)
        // and it's impossible since we have a mutable accounts references
        // so I manually created the struct

        let accounts: NewOrderV3 = NewOrderV3 {
            market: ctx.accounts.market.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            request_queue: ctx.accounts.request_queue.to_account_info(),
            event_queue: ctx.accounts.event_queue.to_account_info(),
            market_bids: ctx.accounts.market_bids.to_account_info(),
            market_asks: ctx.accounts.market_asks.to_account_info(),
            order_payer_token_account: ctx.accounts.order_payer_token_account.to_account_info(),
            open_orders_authority: ctx.accounts.open_orders_authority.to_account_info(),
            coin_vault: ctx.accounts.coin_vault.to_account_info(),
            pc_vault: ctx.accounts.pc_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let cpi = CpiContext::new(dex_program, accounts.into());

        dex::new_order_v3(
            cpi,
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            self_trade_behavior,
            order_type,
            client_order_id,
            limit,
        )?;

        Ok(())
    }

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        let side: Side = Side::Ask;
        let order_id: u128 = 1;

        let cpi_accounts = CancelOrderV2 {
            market: ctx.accounts.market.to_account_info(),
            market_bids: ctx.accounts.market_bids.to_account_info(),
            market_asks: ctx.accounts.market_asks.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            open_orders_authority: ctx.accounts.open_orders_authority.to_account_info(),
            event_queue: ctx.accounts.event_queue.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.dex_program.to_account_info(), cpi_accounts);
        cancel_order_v2(cpi, side, order_id)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, payer = deposit_auth, space = Vault::SIZE)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check it
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check it
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check it
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check it
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        payer = deposit_auth,
        associated_token::authority = pda_auth,
    )]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check it
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth,
    )]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLimit<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, Vault>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init, 
        seeds = [b"limit", token_mint.key().as_ref(), deposit_account.key().as_ref()], 
        bump, 
        payer = deposit_auth, 
        space = Limit::LEN
    )]
    pub limit_account: Account<'info, Limit>,
    pub token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(owner = Token::id())]
    pub ask_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewOrder<'info> {
    /// CHECK: no need to check
    pub market: AccountInfo<'info>,
    /// CHECK: no need to check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: no need to check
    pub request_queue: AccountInfo<'info>,
    /// CHECK: no need to check
    pub event_queue: AccountInfo<'info>,
    /// CHECK: no need to check
    pub market_bids: AccountInfo<'info>,
    /// CHECK: no need to check
    pub market_asks: AccountInfo<'info>,
    // Token account where funds are transferred from for the order. If
    // posting a bid market A/B, then this is the SPL token account for B.
    /// CHECK: no need to check
    pub order_payer_token_account: AccountInfo<'info>,
    /// CHECK: no need to check
    pub open_orders_authority: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    /// CHECK: no need to check
    pub coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    /// CHECK: no need to check
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: no need to check
    pub token_program: AccountInfo<'info>,
    pub dex_program: Program<'info, Dex>,
    /// CHECK: no need to check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    /// CHECK: no need to check
    pub market: AccountInfo<'info>,
    /// CHECK: no need to check
    pub market_bids: AccountInfo<'info>,
    /// CHECK: no need to check
    pub market_asks: AccountInfo<'info>,
    /// CHECK: no need to check
    pub open_orders: AccountInfo<'info>,
    /// CHECK: no need to check
    pub open_orders_authority: AccountInfo<'info>,
    /// CHECK: no need to check
    pub event_queue: AccountInfo<'info>,
    pub dex_program: Program<'info, Dex>,
}

#[account]
pub struct Vault {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
}

#[account]
pub struct Limit {
    pub asset_holding_pda: Option<Pubkey>,
    pub asset: Asset,
    pub ask_price_per_asset: u64,
    pub ask_asset: Asset,
    pub ask_asset_pda: Option<Pubkey>,
}

#[account]
pub struct Asset {
    pub asset_type: String,
    pub asset_metadata: Option<Pubkey>,
    pub asset_mint: Option<Pubkey>,
}

impl Vault {
    const SIZE: usize = 43; // descriminator (8 bytes), 1 pubkey (32 bytes), u8 (1 byte) & Option<T> (1 + 1 bytes)
}

impl Limit {
    const LEN: usize = 33 + Asset::LEN + 8 + Asset::LEN + 33; // Option = 32 + 1  (2 times), Asset::LEN (2 times),
                                                              // u64 = 8 bytes
}

impl Asset {
    const LEN: usize = 32 + 33 + 33; // String = 32, 2 Option enums = (1 + 32) * 2
}
