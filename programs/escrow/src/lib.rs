pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BG7WeJAmBHiBEZHdBXFhiooQRSuBwNeRj8vVBKzmYUYY");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, id: u64, token_a_offered_amount: u64, token_b_wanted_amount: u64) -> Result <()> {


        make_offer::make_offer_handler(ctx, id, token_a_offered_amount, token_b_wanted_amount)

    }
}
