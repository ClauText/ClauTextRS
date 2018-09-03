

mod itemtype;
mod usertype;
mod utility;
mod loaddata;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use std::fs::File;
use std::time;


fn main()
{
    let mut x;
    let a = time::SystemTime::now();
    let mut inFile = File::open("input.eu4").unwrap(); // how check error?
        
    x = ::loaddata::LoadData::_LoadData(&mut inFile);   
         
   
    let b = time::SystemTime::now();
    let c = b.duration_since(a);
    

    let mut outFile = File::create("output.txt").unwrap();
  
    match &x {
        Some(temp) => {
            let bytes = temp.borrow().to_string().into_bytes();
            let buffer = &bytes[..];

            outFile.write(buffer);
        }
        None => { }
    }
    
   // println!("{:?}", x);
    println!("{:?}", c);
}
 
