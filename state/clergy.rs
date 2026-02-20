// programs/apostolic_chain/src/state.rs
use anchor_lang::prelude::*;

#[account]
pub struct Clergy {
    pub hash: String,
    pub parent_hash: String,
    pub name: String,
    pub role: Role,
    pub start_date: i64,
    pub papacy_start_date: Option<i64>,
    pub bump: u8,
}

impl Clergy {
    pub const INIT_SPACE: usize = 8       // Discriminator
        + (4 + 66)   // hash: "0x" + 64 hex chars
        + (4 + 66)   // parent_hash
        + (4 + 100)  // name (margem segura)
        + 1          // role enum (1 byte para variante)
        + 8          // start_date i64
        + (1 + 8)    // Option<i64>
        + 1;         // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Bishop,
    Pope,
    Root,
}