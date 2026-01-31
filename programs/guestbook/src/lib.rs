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


        // realloc magic: calculate new size
        // hard cap: don't let it grow beyond max
        let new_size = Guestbook::BASE_SIZE
            + (guestbook.entries.len() + 1) * (32 + 4 + 64 + 8);
        require!(new_size <= Guestbook::MAX_TOTAL_SIZE, GuestbookError::GuestbookTooBig);
        // This above code is for dynamic size guestbook


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
        // optional: This is for Fixed Size Guestbook
        // space = 8 + Guestbook::MAX_SIZE, 

        // --- realloc magic ---
        space = Guestbook::BASE_SIZE, 
        seeds = [b"guestbook", signer.key().as_ref()],
        bump
    )]
    entries: Account<'info, Guestbook>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct AddEntry<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [b"guestbook", signer.key().as_ref()],
        bump,

        // optional: This is for Dynamic Size Guestbook
          // --- realloc magic ---
        realloc = Guestbook::BASE_SIZE + ((guestbook.entries.len() + 1) * (32 + 4 + 64 + 8)),
        realloc::payer = signer,
        realloc::zero = false    
    )]
    guestbook: Account<'info, Guestbook>,
    system_program: Program<'info, System>,
}


// // Note: This is for fixed size guestbook with max 90 entries
// impl Guestbook {
//     pub const MAX_SIZE: usize = 8 + 32 + (4 + 90 * (32 + 4 + 64 + 8)); // Adjusted for max 90 entries (fits within 10KB limit)
// }


// Note: This is for dynamic size guestbook

impl Guestbook {
    pub const BASE_SIZE: usize = 8 + 32 + 4; // disc + signer + vec len
    pub const MAX_TOTAL_SIZE: usize = 10 * 1024; // cap (10 KB) for playground safety
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
    #[msg("The guestbook has reached its maximum size.")]
    GuestbookTooBig,
}
