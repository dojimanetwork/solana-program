use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;

declare_id!("7LruXpuU5sUkRovmSbQsFGcgwVCarSa4GisRJodP3juM");

#[program]
mod solana_lock {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        return Ok(());
    }

    pub fn transfer_tokens(ctx: Context<TransferNonNative>, amount: u64) -> ProgramResult {
        let sender = &ctx.accounts.from;
        let sender_tokens = &ctx.accounts.from_token_account;
        let recipient_tokens = &ctx.accounts.to_token_account;
        let token_program = &ctx.accounts.token_program;
    
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: sender_tokens.to_account_info(),
                    to: recipient_tokens.to_account_info(),
                    authority: sender.to_account_info(),
                    },
                ),
                amount,
            )?;
    
        return Ok(());
    }

    pub fn transfer_nat_tokens(ctx: Context<TransferNative>, amount: u64) -> ProgramResult {
        let ix = system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )?;
    
        return Ok(());
    }

}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferNonNative<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, token::TokenAccount>,
    pub mint: Account<'info, token::Mint>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferNative<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: Account<'info, token::TokenAccount>,
}
