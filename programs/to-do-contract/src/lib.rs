use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;
use crate::{constant::*, error::*, states::*};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod todo {
    use super::*;
    
}

