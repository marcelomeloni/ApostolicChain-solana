// programs/apostolic_chain/src/instructions/initialize_genesis.rs
use anchor_lang::prelude::*;
use crate::state::*;

fn bytes_to_hex(bytes: &[u8; 32]) -> String {
    format!("0x{}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

pub fn handler(
    ctx: Context<InitializeGenesis>,
    jesus_hash: [u8; 32],
    peter_hash: [u8; 32],
    peter_name: String,
    peter_start_date: i64,
) -> Result<()> {

    let jesus = &mut ctx.accounts.jesus;
    jesus.hash = bytes_to_hex(&jesus_hash);
    jesus.name = "Jesus Cristo".to_string();
    jesus.role = Role::Root;
    jesus.start_date = 0;
    jesus.parent_hash = "GOD".to_string();
    jesus.papacy_start_date = None;
    jesus.bump = ctx.bumps.jesus;

    let peter = &mut ctx.accounts.peter;
    peter.hash = bytes_to_hex(&peter_hash);
    peter.name = peter_name;
    peter.role = Role::Pope;
    peter.start_date = peter_start_date;
    peter.parent_hash = bytes_to_hex(&jesus_hash);
    peter.papacy_start_date = Some(peter_start_date);
    peter.bump = ctx.bumps.peter;

    msg!("Gênesis Inicializado: Jesus ({}) -> Pedro ({})",
        bytes_to_hex(&jesus_hash),
        bytes_to_hex(&peter_hash)
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(jesus_hash: [u8; 32], peter_hash: [u8; 32])]
pub struct InitializeGenesis<'info> {
    #[account(
        init,
        payer = user,
        space = Clergy::INIT_SPACE,
        seeds = [b"clergy", jesus_hash.as_ref()],
        bump
    )]
    pub jesus: Account<'info, Clergy>,

    #[account(
        init,
        payer = user,
        space = Clergy::INIT_SPACE,
        seeds = [b"clergy", peter_hash.as_ref()],
        bump
    )]
    pub peter: Account<'info, Clergy>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}