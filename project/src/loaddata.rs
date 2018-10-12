


use usertype::UserType;
use std::collections::VecDeque;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;


pub struct LoadData
{

}

#[inline]
fn is_state0(state_reserve : i64) -> bool 
{
    return 1 == state_reserve;
}

impl LoadData
{
    pub fn _load_data(mut in_file : &mut File) -> Option<Rc<RefCell<UserType>>>
    {
      //  println!("chk-1");

        let global : UserType = UserType::new("".to_string());
        
      //  println!("chk-1");

        let mut str_vec = VecDeque::new();
        let mut state : i32 = 0;
        let mut brace_num : usize = 0;
        let mut state_reserve : i64 = 0;
        let mut nested_ut : Vec<Option<Rc<RefCell<UserType>>>> = Vec::new();
        
        let mut var1 : String = String::new();
        let mut var2 : String = String::new();
        let mut val : String = String::new();

     //   println!("chk0");

        nested_ut.push(Some(Rc::new(RefCell::new(global))));
    
        {
        //    let a = time::SystemTime::now();
            ::utility::Utility::reserve(&mut in_file, &mut str_vec);
      //      let b = time::SystemTime::now();
            // check empty and end of file?
          //  let c = b.duration_since(a);
    
            //println!("{:?}", c);
        }

        //println!("chk1");

        while !str_vec.is_empty() 
        {
            if 0 == state {
                if "{" == ::utility::Utility::top(&str_vec)  {
                    state = 2;
                }
                else {
                    if str_vec.len() >= 2 {
                        if "=" == ::utility::Utility::lookup(&str_vec, 1) {
                            var2 = ::utility::Utility::pop(&mut str_vec);
                            let temp = ::utility::Utility::pop(&mut str_vec);
                            state = 2;
                        }
                        else {
                            let temp = ::utility::Utility::pop(&mut str_vec);
                            
                            match &nested_ut[brace_num] {
                                Some(x) => { x.borrow_mut().add_item("".to_string(), temp); }
                                None => { panic!("test?"); }
                            }

                            state = 0;
                        }
                    }
                    else {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                        match &nested_ut[brace_num] {
                            Some(x) => { x.borrow_mut().add_item("".to_string(), temp); }
                            None => { }
                        }
                    }
                }
            }
            else if 1 == state {
                if !str_vec.is_empty() && "}" == ::utility::Utility::top(&str_vec) {
                    let temp = ::utility::Utility::pop(&mut str_vec);
                    state = 0;
                }
                else {
                    // error
                    panic!("error 1");
                }
            }
            else if 2 == state {
                if !str_vec.is_empty() && "{" == ::utility::Utility::top(&str_vec) {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    let mut ut = UserType::new(var2.clone());
                    
                    match &nested_ut[brace_num] {
                        Some(x) => { x.borrow_mut().add_UserType_item(&mut ut); }
                        None => { }
                    }

                    brace_num += 1;

                    if nested_ut.len() == brace_num {
                        nested_ut.push(Some(Rc::new(RefCell::new(UserType::new("".to_string())))));
                    }
                    {
                        let mut temp = None;
                        {
                            match &nested_ut[brace_num - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nested_ut[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nested_ut[brace_num] = temp;
                    }
                    var2.clear();

                    state = 3;
                }
                else {
                    val = ::utility::Utility::pop(&mut str_vec);

                    match &nested_ut[brace_num] {
                        Some(x) => { x.borrow_mut().add_item(var2.clone(), val.clone()); }
                        None => { }
                    }

                    var2.clear();
                    val.clear();

                    state = 0;
                }
            }
            else if 3 == state {
                if !str_vec.is_empty() && ::utility::Utility::top(&str_vec) == "}" {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    brace_num -= 1;

                    state = 0;
                }
                else {
                    state_reserve += 1;

                    state = 4;
                }
            }
            else if 4 == state {
                if !str_vec.is_empty() && ::utility::Utility::top(&str_vec) == "{" {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }
                    let mut ut = UserType::new("".to_string());
                    match &nested_ut[brace_num] {
                        Some(x) => { x.borrow_mut().add_UserType_item(&mut ut); }
                        None => { }
                    }

                    brace_num += 1;

                    if nested_ut.len() == brace_num {
                        nested_ut.push(Some(Rc::new(RefCell::new(UserType::new("".to_string())))));
                    }
                   
                    {
                        let mut temp = None;
                        {
                            match &nested_ut[brace_num - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nested_ut[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nested_ut[brace_num] = temp;
                    }
                    state = 5;
                }
                else if !str_vec.is_empty() && ::utility::Utility::top(&str_vec) == "}" {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    state = if is_state0(state_reserve) { 0 } else { 4 };
                    state_reserve -= 1;

                    brace_num -= 1;
                }
                else {
                    if str_vec.len() >= 2 {
                       if "=" == ::utility::Utility::lookup(&str_vec, 1) {
                            var2 = ::utility::Utility::pop(&mut str_vec);
                            {
                               let temp = ::utility::Utility::pop(&mut str_vec);
                            }

                            state = 6;
                       }
                       else {
                            var1 = ::utility::Utility::pop(&mut str_vec);
                            match &nested_ut[brace_num] {
                                Some(x) => { x.borrow_mut().add_item("".to_string(), var1.clone()); }
                                None => { }
                            }

                            var1.clear();

                            state = 4;
                       }
                    }
                    else {
                        var1 = ::utility::Utility::pop(&mut str_vec);
                        match &nested_ut[brace_num] {
                            Some(ref x) => { x.borrow_mut().add_item("".to_string(), var1.clone()); }
                            None => { }
                        }

                        var1.clear();

                        state = 4;
                    }
                }
            }
            else if 5 == state {
                if !str_vec.is_empty() && "}" == ::utility::Utility::top(&str_vec) {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    brace_num -= 1;

                    state = 4;
                }
                else {
                    state_reserve += 1;

                    state = 4;
                }
            }
            else if 6 == state {
                if !str_vec.is_empty() && "{" == ::utility::Utility::top(&str_vec) {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    let mut ut = UserType::new(var2.clone());
                    match &nested_ut[brace_num] {
                        Some(x) => { x.borrow_mut().add_UserType_item(&mut ut); }
                        None => { }
                    }

                    var2.clear();

                    brace_num += 1;

                    if nested_ut.len() == brace_num {
                        nested_ut.push(Some(Rc::new(RefCell::new(UserType::new("".to_string())))));
                    }

                    {
                        let mut temp = None;
                        {
                            match &nested_ut[brace_num - 1] {
                                Some(y) => { temp = y.borrow().get_user_type_list(y.borrow().get_user_type_list_size() - 1); } // nested_ut[y.borrow().get_user_type_list_size() - 1]; }
                                None => { }
                            }
                        }
                        nested_ut[brace_num] = temp;
                    }
                    state = 7;
                }
                else {
                    val = ::utility::Utility::pop(&mut str_vec);
                    
                    match &nested_ut[brace_num] {
                        Some(x) => { x.borrow_mut().add_item(var2.clone(), val.clone()); }
                        None => { }
                    }

                    var2.clear();
                    val.clear();

                    if  !str_vec.is_empty() && "}" == ::utility::Utility::top(&str_vec) {
                        {
                            let temp = ::utility::Utility::pop(&mut str_vec);
                        }
                        state = if is_state0(state_reserve) { 0 } else { 4 };
                        state_reserve -= 1;

                        brace_num -= 1;
                    }
                    else {
                        state = 4;
                    }
                }               
            }
            else if 7 == state {
                if !str_vec.is_empty() && "}" == ::utility::Utility::top(&str_vec) {
                    {
                        let temp = ::utility::Utility::pop(&mut str_vec);
                    }

                    brace_num -= 1;

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


            //if str_vec.len() < 10 {
            //    ::utility::Utility::reserve(&mut in_file, &mut str_vec);
            //}
        }
        
        println!("chked {} ", brace_num);
        {
            let global = nested_ut[brace_num].clone();
            return global;
        }
        panic!("error2 in _LoadData");
    }
}

