1.	Исходный код смарт-контракта (⁠programs/profile/src/lib.rs⁠):
use anchor_lang::prelude::*;

declare_id!("4Tr1iB2qsN3xn3Pvc3eMcrzmkABgU8M3mnrSPeMT18tYUGMMo27z6HMaAwBHwrF46NPWHXvGbiQkXve6DVk3QNij");

#[program]
pub mod profile_program {
    use super::*;

    // Инициализация децентрализованного профиля через PDA
    pub fn initialize_profile(ctx: Context<InitializeProfile>, name: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.user.key();
        profile.name = name;
        profile.bio = bio;
        msg!("On-chain профиль успешно создан для пользователя: {}", profile.authority);
        Ok(())
    }
}

// Структура данных профиля на блокчейне
#[account]
pub struct UserProfile {
    pub authority: Pubkey, // 32 байта
    pub name: String,      // Имя пользователя
    pub bio: String,       // Описание / Биография
}

// Контекст инструкций с использованием PDA семян (seeds)
#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + (4 + 50) + (4 + 200), // Дискриминатор + authority + name + bio
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
