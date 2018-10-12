
//!
//! item type 
//! 

#[derive(Debug, Clone)]
pub struct ItemType
{
    name : String,
    value : String
}


impl ItemType
{ 
    pub fn new(name : String, value : String) -> Self {
        ItemType{name : name, value : value}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn set_name(&mut self, name : String) {
        self.name = name;
    }
    pub fn set_value(&mut self, value : String) {
        self.value = value;
    }
}
