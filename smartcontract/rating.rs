use anchor_lang::prelude::*;

declare_id!("YourContract11111111111111111111111111111111111");

#[program]
mod rating {
    use super::*;

    #[state]
    pub struct Rating {
        pub store_id: u64,
        pub ratings: Vec<u32>,
    }

    impl Rating {
        pub fn new(ctx: Context<Initialize>, store_id: u64, _bump: u8) -> ProgramResult {
            let rating = &mut ctx.accounts.rating;
            rating.store_id = store_id;
            rating.ratings = Vec::new();
            Ok(())
        }

        #[access_control(is_allowed_to_add_ratings)]
        pub fn add_ratings(ctx: Context<AddRatings>, new_ratings: Vec<u32>) -> ProgramResult {
            let mut rating_state = ctx.accounts.rating.load()?;
            rating_state.ratings.extend(new_ratings);
            rating_state.save(&mut ctx.accounts.rating)?;
            Ok(())
        }

        pub fn get_average_rating(ctx: Context<GetAverageRating>) -> ProgramResult {
            let rating_state = ctx.accounts.rating.load()?;
            let sum: u32 = rating_state.ratings.iter().sum();
            let average = sum as f32 / rating_state.ratings.len() as f32;
            msg!("Average Rating for Store {}: {:?}", rating_state.store_id, average);
            Ok(())
        }
    }

    fn is_allowed_to_add_ratings(ctx: &Context<AddRatings>) -> Result<()> {
        if *ctx.accounts.user.key != ctx.accounts.rating.store_id {
            return Err(ErrorCode::Unauthorized.into());
        }
        Ok(())
    }

    #[error]
    pub enum ErrorCode {
        #[msg("Unauthorized")]
        Unauthorized,
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + 4)]
        pub rating: Account<'info, Rating>,
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct AddRatings<'info> {
        #[account(mut)]
        pub rating: Loader<'info, Rating>,
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct GetAverageRating<'info> {
        #[account(mut)]
        pub rating: Loader<'info, Rating>,
    }
}
