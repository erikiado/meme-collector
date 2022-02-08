use anchor_lang::prelude::*;

declare_id!("6pSWkdAc67oEMBn4AGHt4oNNHD5K4Pm7h6GKMHoEV4wb");

#[program]
pub mod meme_collector {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_memes.
    base_account.total_memes = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_meme(ctx: Context<AddMeme>, meme_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	  // Build the struct.
    let item = MemeStruct {
      meme_id: base_account.meme_list.len() as u64,
      meme_link: meme_link.to_string(),
      user_address: *user.to_account_info().key,
      upvotes: 0,
      upvoters: Vec::new(),
    };
		
	  // Add it to the gif_list vector.
    base_account.meme_list.push(item);
    base_account.total_memes += 1;
    Ok(())
  }

  pub fn upvote_meme(ctx: Context<UpvoteMeme>, meme_id: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let meme_id_usize: usize = meme_id.parse().unwrap();

    if meme_id_usize >= base_account.meme_list.len() {
      return Err(ErrorCode::MemeNotFound.into());
    }

    let item = &mut base_account.meme_list[meme_id_usize];
    let user_key = *user.to_account_info().key;
    if item.upvoters.iter().any(|&i| i==user_key) {
      // User already upvoted this meme
    } else {
      item.upvotes += 1;
      item.upvoters.push(user_key);
    }

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddMeme<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpvoteMeme<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MemeStruct {
  pub meme_id: u64,
  pub meme_link: String,
  pub upvotes: u64,
  pub upvoters: Vec<Pubkey>,
  pub user_address: Pubkey,
}

#[derive(Accounts)]
pub struct MemeNotFound {}

#[error]
pub enum ErrorCode {
    #[msg("Meme not found")]
    MemeNotFound,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
  pub total_memes: u64,
  pub meme_list: Vec<MemeStruct>,
}
