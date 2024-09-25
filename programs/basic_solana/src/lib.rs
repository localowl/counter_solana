use anchor_lang::prelude::*;

declare_id!("CpiFvsDW7wpvUPGYVGz1Wgq5TgecKd6n6BLUg3XS7ic1");

#[program]
pub mod basic_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous value of counter: {}", counter.count);

        counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous value of counter: {}", counter.count);

        if counter.count > 0 {
            counter.count -= 1;
        }
        Ok(())
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(
            init,
            payer = user,
            space = 8 + CounterAccount::INIT_SPACE,
        )]
        pub counter: Account<'info, CounterAccount>,

        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Update<'info> {
        #[account(mut)]
        pub counter: Account<'info, CounterAccount>,
        pub user: Signer<'info>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct CounterAccount {
        pub count: u64,
    }
}
