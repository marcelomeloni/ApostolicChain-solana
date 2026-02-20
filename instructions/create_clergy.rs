// programs/apostolic_chain/src/instructions/create_clergy.rs
use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn handler(
    ctx: Context<CreateClergy>,
    _seed_bytes: [u8; 32], // <- Adicionamos aqui (com underline se não for usar no corpo)
    hash: String,
    parent_hash: String,
    name: String,
    role: Role,
    start_date: i64,
    papacy_start_date: Option<i64>,
) -> Result<()> {

    if parent_hash == "00x00x00" {
        msg!("Registro com linhagem quebrada (Dados Perdidos). Aceito.");
    } else {
        require!(
            ctx.accounts.parent_account.is_some(),
            ErrorCode::ParentAccountMissing
        );
        let parent = ctx.accounts.parent_account.as_ref().unwrap();
        require!(parent.hash == parent_hash, ErrorCode::InvalidParentHash);

        msg!("Linhagem verificada: {} -> {}", parent.name, name);
    }

    let clergy = &mut ctx.accounts.new_clergy;
    clergy.hash = hash;
    clergy.parent_hash = parent_hash;
    clergy.name = name;
    clergy.role = role;
    clergy.start_date = start_date;
    clergy.papacy_start_date = papacy_start_date;
    clergy.bump = ctx.bumps.new_clergy;

    Ok(())
}

#[derive(Accounts)]
// MUDANÇA: Exigimos 32 bytes reais como argumento para o cálculo seguro do PDA
#[instruction(seed_bytes: [u8; 32], hash: String, parent_hash: String)]
pub struct CreateClergy<'info> {
    #[account(
        init,
        payer = user,
        space = Clergy::INIT_SPACE,
        // SEED: Agora usa exatos 32 bytes aceitos pela Solana
        seeds = [b"clergy", seed_bytes.as_ref()],
        bump
    )]
    pub new_clergy: Account<'info, Clergy>,

    pub parent_account: Option<Account<'info, Clergy>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}