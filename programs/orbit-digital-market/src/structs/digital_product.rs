use anchor_lang::prelude::*;
use product::product_struct::OrbitProduct;

#[account]
pub struct DigitalProduct{
    pub metadata: OrbitProduct,
    pub digital_product_type: DigitalProductType,
    pub digital_file_type: DigitalFileTypes
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum DigitalProductType{
    Commission,
    Template
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum DigitalFileTypes{
    Text,
    Video,
    Audio,
    Folder
}