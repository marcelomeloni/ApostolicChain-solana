use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("HKUdr1NeewdqE3vEzHmAu9waow5p4bHg6V6t4iM5cLhK");

#[program]
pub mod apostolic_chain {
    use super::*;

    pub fn initialize_genesis(
        ctx: Context<InitializeGenesis>,
        jesus_hash: [u8; 32],
        peter_hash: [u8; 32],
        peter_name: String,
        peter_start_date: i64,
    ) -> Result<()> {
        instructions::initialize_genesis::handler(
            ctx,
            jesus_hash,
            peter_hash,
            peter_name,
            peter_start_date,
        )
    }

    pub fn create_clergy(
        ctx: Context<CreateClergy>,
        seed_bytes: [u8; 32],
        hash: String,
        parent_hash: String,
        name: String,
        role: Role,
        start_date: i64,
        papacy_start_date: Option<i64>,
    ) -> Result<()> {
        instructions::create_clergy::handler(
            ctx,
            seed_bytes,
            hash,
            parent_hash,
            name,
            role,
            start_date,
            papacy_start_date,
        )
    }
}