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
    pub const INIT_SPACE: usize = 8      
        + (4 + 66) 
        + (4 + 66)   
        + (4 + 100) 
        + 1          
        + 8         
        + (1 + 8) 
        + 1;        
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Bishop,
    Pope,
    Root,
}
