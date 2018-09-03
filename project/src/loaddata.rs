


use usertype::user_type;
use std::collections::VecDeque;
use std::fs::File;
use std::time;
use std::rc::Rc;
use std::cell::RefCell;


pub struct LoadData
{

}

#[inline]
fn isState0(state_reserve : i64) -> bool 
{
    return 1 == state_reserve;
}

impl LoadData
{
    pub fn _LoadData(mut inFile : &mut File) -> Option<Rc<RefCell<user_type>>>
    {
      //  println!("chk-1");

        let global : user_type = user_type::new("".to_string());
        
      //  println!("chk-1");

        let mut strVec = VecDeque::new();
        let mut state : i32 = 0;
        let mut braceNum : usize = 0;
        let mut state_reserve : i64 = 0;
        let mut nestedUT : Vec<Option<Rc<RefCell<user_type>>>> = Vec::new();
        
        let mut var1 : String = String::new();
        let mut var2 : String = String::new();
        let mut val : String = String::new();

     //   println!("chk0");

        nestedUT.push(Some(Rc::new(RefCell::new(global))));
    
        {
        //    let a = time::SystemTime::now();
            ::utility::Utility::reserve(&mut inFile, &mut strVec);
      //      let b = time::SystemTime::now();
            // check empty and end of file?
          //  let c = b.duration_since(a);
    
            //println!("{:?}", c);
        }

        //println!("chk1");

        while !strVec.is_empty() 
        {
            if 0 == state {
                if "{" == ::utility::Utility::top(&strVec)  {
                    state = 2;
                }
                else {
                    if strVec.len() >= 2 {
                        if "=" == ::utility::Utility::lookup(&strVec, 1) {
                            var2 = ::utility::Utility::pop(&mut strVec);
                            let temp = ::utility::Utility::pop(&mut strVec);
                            state = 2;
                        }
                        else {
                            let temp = ::utility::Utility::pop(&mut strVec);
                            
                            match &nestedUT[braceNum] {
                                Some(x) => { x.borrow_mut().add_item("".to_string(), temp); }
                                None => { panic!("test?"); }
                            }

                            state = 0;
                        }
                    }
                    else {
                        let temp = ::utility::Utility::pop(&mut strVec);
                        match &nestedUT[braceNum] {
                            Some(x) => { x.borrow_mut().add_item("".to_string(), temp); }
                            None => { }
                        }
                    }
                }
            }
            else if 1 == state {
                if !strVec.is_empty() && "}" == ::utility::Utility::top(&strVec) {
                    let temp = ::utility::Utility::pop(&mut strVec);
                    state = 0;
                }
                else {
                    // error
                    panic!("error 1");
                }
            }
            else if 2 == state {
                if !strVec.is_empty() && "{" == ::utility::Utility::top(&strVec) {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    let mut ut = user_type::new(var2.clone());
                    
                    match &nestedUT[braceNum] {
                        Some(x) => { x.borrow_mut().add_user_type_item(&mut ut); }
                        None => { }
                    }

                    braceNum += 1;

                    if nestedUT.len() == braceNum {
                        nestedUT.push(Some(Rc::new(RefCell::new(user_type::new("".to_string())))));
                    }
                    {
                        let mut temp = None;
                        {
                            match &nestedUT[braceNum - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nestedUT[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nestedUT[braceNum] = temp;
                    }
                    var2.clear();

                    state = 3;
                }
                else {
                    val = ::utility::Utility::pop(&mut strVec);

                    match &nestedUT[braceNum] {
                        Some(x) => { x.borrow_mut().add_item(var2.clone(), val.clone()); }
                        None => { }
                    }

                    var2.clear();
                    val.clear();

                    state = 0;
                }
            }
            else if 3 == state {
                if !strVec.is_empty() && ::utility::Utility::top(&strVec) == "}" {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    braceNum -= 1;

                    state = 0;
                }
                else {
                    state_reserve += 1;

                    state = 4;
                }
            }
            else if 4 == state {
                if !strVec.is_empty() && ::utility::Utility::top(&strVec) == "{" {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }
                    let mut ut = user_type::new("".to_string());
                    match &nestedUT[braceNum] {
                        Some(x) => { x.borrow_mut().add_user_type_item(&mut ut); }
                        None => { }
                    }

                    braceNum += 1;

                    if nestedUT.len() == braceNum {
                        nestedUT.push(Some(Rc::new(RefCell::new(user_type::new("".to_string())))));
                    }
                   
                    {
                        let mut temp = None;
                        {
                            match &nestedUT[braceNum - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nestedUT[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nestedUT[braceNum] = temp;
                    }
                    state = 5;
                }
                else if !strVec.is_empty() && ::utility::Utility::top(&strVec) == "}" {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    state = if isState0(state_reserve) { 0 } else { 4 };
                    state_reserve -= 1;

                    braceNum -= 1;
                }
                else {
                    if strVec.len() >= 2 {
                       if "=" == ::utility::Utility::lookup(&strVec, 1) {
                            var2 = ::utility::Utility::pop(&mut strVec);
                            {
                               let temp = ::utility::Utility::pop(&mut strVec);
                            }

                            state = 6;
                       }
                       else {
                            var1 = ::utility::Utility::pop(&mut strVec);
                            match &nestedUT[braceNum] {
                                Some(x) => { x.borrow_mut().add_item("".to_string(), var1.clone()); }
                                None => { }
                            }

                            var1.clear();

                            state = 4;
                       }
                    }
                    else {
                        var1 = ::utility::Utility::pop(&mut strVec);
                        match &nestedUT[braceNum] {
                            Some(ref x) => { x.borrow_mut().add_item("".to_string(), var1.clone()); }
                            None => { }
                        }

                        var1.clear();

                        state = 4;
                    }
                }
            }
            else if 5 == state {
                if !strVec.is_empty() && "}" == ::utility::Utility::top(&strVec) {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    braceNum -= 1;

                    state = 4;
                }
                else {
                    state_reserve += 1;

                    state = 4;
                }
            }
            else if 6 == state {
                if !strVec.is_empty() && "{" == ::utility::Utility::top(&strVec) {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    let mut ut = user_type::new(var2.clone());
                    match &nestedUT[braceNum] {
                        Some(x) => { x.borrow_mut().add_user_type_item(&mut ut); }
                        None => { }
                    }

                    var2.clear();

                    braceNum += 1;

                    if nestedUT.len() == braceNum {
                        nestedUT.push(Some(Rc::new(RefCell::new(user_type::new("".to_string())))));
                    }

                    {
                        let mut temp = None;
                        {
                            match &nestedUT[braceNum - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nestedUT[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nestedUT[braceNum] = temp;
                    }
                    state = 7;
                }
                else {
                    val = ::utility::Utility::pop(&mut strVec);
                    
                    match &nestedUT[braceNum] {
                        Some(x) => { x.borrow_mut().add_item(var2.clone(), val.clone()); }
                        None => { }
                    }

                    var2.clear();
                    val.clear();

                    if  !strVec.is_empty() && "}" == ::utility::Utility::top(&strVec) {
                        {
                            let temp = ::utility::Utility::pop(&mut strVec);
                        }
                        state = if isState0(state_reserve) { 0 } else { 4 };
                        state_reserve -= 1;

                        braceNum -= 1;
                    }
                    else {
                        state = 4;
                    }
                }               
            }
            else if 7 == state {
                if !strVec.is_empty() && "}" == ::utility::Utility::top(&strVec) {
                    {
                        let temp = ::utility::Utility::pop(&mut strVec);
                    }

                    braceNum -= 1;

                    state = 4;
                }
                else {
                    state_reserve += 1;

                    state = 4;
                }
            }
            else {
                panic!("error");
            }


            //if strVec.len() < 10 {
            //    ::utility::Utility::reserve(&mut inFile, &mut strVec);
            //}
        }
        
        println!("chked {} ", braceNum);
        {
            let global = nestedUT[braceNum].clone();
            return global;
        }
        panic!("error2 in _LoadData");
    }
}

