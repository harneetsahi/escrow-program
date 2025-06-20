use anchor_lang::prelude::*;
use crate::{error:: ErrorCode, state::Offer};
use super::shared::{transfer_tokens, close_token_account};
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface:: {Mint, TokenAccount, TokenInterface}
};

// first need to take tokens from the vault and put them in taker's account
// then take tokens from taker's account and send them straight to offer maker's account


#[derive(Accounts)]
pub struct TakeOffer<'info> {
  #[account(mut)]
  pub taker: Signer<'info>,

  #[account(mut)]
  pub maker: SystemAccount<'info>,

  pub token_mint_a: InterfaceAccount<'info, Mint>,

  pub token_mint_b: InterfaceAccount<'info, Mint>,

  #[account(
    init_if_needed, // because if taker never had an account for token a, we need to first initialize it
    payer = taker,
    associated_token::mint = token_mint_a,
    associated_token::authority = taker,
    associated_token::token_program = token_program
  )]
  pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>, // this where we put the tokens we get from the vault

  #[account(
    mut,
    associated_token::mint = token_mint_b,
    associated_token::authority = taker,
    associated_token::token_program = token_program
  )]
  pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>, // this is where we take tokens from to send them to offer maker's account

  #[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = token_mint_b,
    associated_token::authority = maker,
    associated_token::token_program = token_program

  )]
  pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut, // because we will close this account when offer is taken
    close = maker, // this will refund the rent back to the maker because maker paid to create it
    has_one = maker,
    has_one = token_mint_a,
    has_one = token_mint_b,
    seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
    bump = offer.bump
  )]
  offer: Account<'info, Offer>,

  #[account(
    mut,
    associated_token::mint = token_mint_a, // because token a is what was stored in the vault when offer was created
    associated_token::authority = offer,
    associated_token::token_program = token_program

  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,

  pub associated_token_program : Program<'info, AssociatedToken>,
  pub token_program : Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,

}

pub fn take_offer_handler(ctx: Context<TakeOffer>) -> Result <()> {

  let offer_account_seeds = &[
    b"offer",
    &ctx.accounts.offer.id.to_be_bytes()[..],
    &[ctx.accounts.offer.bump],
  ];

  let signer_seeds = Some(&offer_account_seeds[..]);

  // vault to taker
  transfer_tokens(
      &ctx.accounts.vault, 
      &ctx.accounts.taker_token_account_a,
      &ctx.accounts.vault.amount,
      &ctx.accounts.token_mint_a,
      &ctx.accounts.offer.to_account_info(),
      &ctx.accounts.token_program,
      signer_seeds
   ).map_err(|_| ErrorCode::FailedVaultWithdrawal)?;

  
  // close vault
  close_token_account(
      &ctx.accounts.vault, 
      &ctx.accounts.taker.to_account_info(), 
      &ctx.accounts.offer.to_account_info(), 
      &ctx.accounts.token_program, 
      signer_seeds,
    ).map_err(|_| ErrorCode::FailedVaultClosure)?;

  // taker to maker
  transfer_tokens(
      &ctx.accounts.taker_token_account_a,
      &ctx.accounts.maker_token_account_b, 
      &ctx.accounts.offer.token_b_wanted_amount, 
      &ctx.accounts.token_mint_b, 
      &ctx.accounts.taker.to_account_info(),
      &ctx.accounts.token_program,
      None
    ).map_err(|_| ErrorCode::InsufficientTakerBalance)?;


  Ok(())
}