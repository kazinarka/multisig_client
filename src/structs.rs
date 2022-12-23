use anchor_client::anchor_lang::prelude::{Pubkey, AnchorSerialize, AnchorDeserialize};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TxInstruction {
    /// Target program to execute this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub keys: Vec<TxAccountMeta>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TxAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}