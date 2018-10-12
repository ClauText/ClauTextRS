


use itemtype::ItemType;
use std::vec::Vec;
use std::rc::Rc;
use std::collections::VecDeque;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct UserType
{
    pub name : String,
    pub ilist : Vec<i32>,
    pub item_list : Vec<ItemType>,
    pub UserTypeList : VecDeque<Option<Rc<RefCell<UserType>>>>,
    pub parent : Option<Rc<RefCell<UserType>>>,
}

impl UserType
{
   /* pub fn clone(&self) -> Self
    {
        let mut x = UserType{name : self.name.clone(), ilist : self.ilist.clone(), 
                                    item_list : self.item_list.clone(), UserTypeList : VecDeque::new(), parent : Option::None};
        
        if self.UserTypeList.is_empty() { return x; }

        x.UserTypeList = self.UserTypeList.clone();
        

        x
    }
    */
}

impl UserType
{
    pub fn new(name : String) -> Self 
    {
        UserType{
            name : name.clone(), ilist : Vec::new(), item_list : Vec::new(), UserTypeList : VecDeque::new(), parent : Option::None
        }
    }

    pub fn to_string(&self) -> String
    {
        let mut result : String = String::new();

       // result += "name is ";
       // result += &self.name;
      //  result += "item type is ";
        for i in 0..self.item_list.len() {
          //  result += " name is ";
            {
                let temp = self.item_list[i].get_name();
                if temp.is_empty() {
                    //
                }
                else {
                    result += temp;
                    result += " = ";
                }
            }
            //result += " value is ";
            result += self.item_list[i].get_value();
            result += " \n";
        }

        let n = self.UserTypeList.len();
  
        for i in 0..n 
        {
            {
                match &self.UserTypeList[i] {
                    Some(x) => {
                        let name = x.borrow().get_name();
                        if name.is_empty() {
                            //
                        }
                        else {
                            result += " ";
                            result += &name;
                            result += " = ";
                        }
                    }
                    None => { }
                }
            }
            result += " { ";
            match &self.UserTypeList[i] { 
                Some(x) => { 
                    let temp = x.borrow();
                    let temp2 = temp.to_string();
                    result += &temp2[..];
                }
                None => panic!("to_string"),
            }
            result += " } \n";
        }

        result
    }   

    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }

     pub fn set_name(&mut self, name : String) {
        self.name = name.clone();
    }


    pub fn add_item(&mut self, name : String, value : String)
    {
        self.ilist.push(1);
        self.item_list.push(ItemType::new(name, value));
    }
    pub fn add_UserType_item(&mut self, ut : &mut UserType) 
    {
        self.ilist.push(2);
        
        ut.parent = self.parent.clone();
        self.UserTypeList.push_back(Option::from(Rc::new(RefCell::new(ut.clone()))));
    }

         // link?
    pub fn insert_UserType_item(&mut self, other : &mut Self) 
    {
        let idx : usize = self.UserTypeList.len() - 1;
        if idx >= 0 {
            self.add_UserType_item(other);
        }
    }

    pub fn get_UserTypeList_size(&self) -> usize
    {
        return self.UserTypeList.len();
    }

    pub fn get_UserTypeList(&self, idx : usize) -> Option<Rc<RefCell<UserType>>>
    {
        return self.UserTypeList[idx].clone();
    }
}

