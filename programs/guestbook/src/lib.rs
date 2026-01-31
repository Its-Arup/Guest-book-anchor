use anchor_lang::prelude::*;

declare_id!("CoPybcQ2XcHysoNEiYdQCqb1jR3Ay6kU2dwaZ6PnxJFV");


#[program]
pub mod guestbook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let guestbook = &mut ctx.accounts.entries;
        guestbook.signer = *ctx.accounts.signer.key;
        guestbook.entries = Vec::new();
        Ok(())
    }

    pub fn add_entry(ctx: Context<AddEntry>, message: String) -> Result<()> {
        
        require!(message.as_bytes().len() <= 64, GuestbookError::MessageTooLong);
        let guestbook = &mut ctx.accounts.guestbook;

        let entry = Entry {
            user: *ctx.accounts.signer.key,
            message,
            ts: Clock::get()?.unix_timestamp,
        };

        guestbook.entries.push(entry);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, 
        payer = signer, 
        space = 8 + Guestbook::MAX_SIZE, 
        seeds = [b"guestbook", signer.key().as_ref()],
        bump
    )]
    entries: Account<'info, Guestbook>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddEntry<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [b"guestbook", signer.key().as_ref()],
        bump
    )]
    guestbook: Account<'info, Guestbook>,
    system_program: Program<'info, System>,
}

impl Guestbook {
    pub const MAX_SIZE: usize = 8 + 32 + (4 + 90 * (32 + 4 + 64 + 8)); // Adjusted for max 90 entries (fits within 10KB limit)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Entry {
    pub user: Pubkey,
    pub message: String,
    pub ts: i64,
}

#[account]
pub struct Guestbook {
    signer: Pubkey,
    entries: Vec<Entry>,
}

#[error_code]
pub enum GuestbookError {
    #[msg("The message is too long.")]
    MessageTooLong,
}
