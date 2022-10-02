use anchor_lang::prelude::*;
use product::product_struct::OrbitProduct;

// disc b910f3a2 [185, 16, 243, 162, 235, 96, 85, 214]
#[account]
pub struct DigitalProduct{
    pub metadata: OrbitProduct,
    pub digital_file_type: DigitalFileTypes
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum DigitalFileTypes{
    Text,
    Video,
    Audio,
    Image,
    Folder
}