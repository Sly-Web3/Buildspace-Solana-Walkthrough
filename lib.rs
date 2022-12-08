use anchor_lang::prelude::*;

declare_id!("DmES6KPkfFXXoHGsgNw7GuWKGxkvzT7wLgLoTC6Lb3Lh");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account; //mut means we are running a mutable reference to base_account. This gives us the power to actually make changes to base_account
    base_account.total_gifs = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)] //This is telling Solana how we want to initiliase BaseAccount. Init will tell solana to create a new account owned by our current program.
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>, //this is data passed into the program to prove that the user calling this program actually own's their wallet.
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> { //This is a context named AddGif that has access to a mutable reference to base_account. Thats why i do #[account(mut)]. Basically it means I can actually change the total_gifs value stored on BaseAccount.
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)] //This tells Anchor how to serialize / deserialize the struct. We serialize the data in our accounts into binary format before storing it. Then when we want to retired it we will deserialize it
pub struct ItemStruct {
    pub gif_link: String, //this holds a string of Gif Links
    pub user_address: Pubkey, //and a public key with the users user_address. 
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>, //vec = vector
}

pub fn add_gif(ctx: Context<AddGif>) -> Result <()> { //when someone calls addGif, make sure to attatch the AddGif context to it so the user can access the base_account and whatever els is attatched to AddGif.
    // Get a reference to the account and increment total_gifs.
    let base_account = &mut ctx.accounts.base_account; //grab base_account which was passed in to the function via Context<Addgifs>. 
    base_account.total_gifs += 1; // Then, you increment the counter thats it
    Ok(())
}