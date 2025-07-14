use crate::*;

#[turbo::serialize]
pub struct Item {
	pub item_type: enums::ItemTypes,
	pub color: u32
}

impl Item {
    pub fn new(item_type: enums::ItemTypes, color: u32) -> Self {
    	Self { 
    		item_type: item_type, 
    		color: color 
    	}
    }
}