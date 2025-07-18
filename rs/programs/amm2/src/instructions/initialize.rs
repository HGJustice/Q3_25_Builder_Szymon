use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};

use crate::state::Config;



#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initalize<'info> {
    #[account(mut)]
    pub initalizer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init, 
        payer = initalizer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = initalizer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
      #[account(
        init,
        payer = initalizer,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initalizer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initalize<'info> {
    pub fn init(&mut self, seed: u64, fees: u16, authority: Option<Pubkey>, bumps: InitalizeBumps) -> Result<()> {
        self.config.set_inner(Config { 
            seed, 
            authority, 
            mint_x: self.mint_x.key(), 
            mint_y: self.mint_y.key(), 
            fees, 
            locked: false, 
            config_bump: bumps.config, 
            lp_bump: bumps.mint_lp
        });
        Ok(())
    }
}