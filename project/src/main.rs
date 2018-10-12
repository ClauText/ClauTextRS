

mod itemtype;
mod usertype;
mod utility;
mod loaddata;

use std::io::Write;
use std::fs::File;
use std::time;


fn main()
{
    let mut x;
    let a = time::SystemTime::now();
    let mut in_file = File::open("input.eu4").unwrap(); // how check error?
        
    x = ::loaddata::LoadData::_load_data(&mut in_file);   
         
   
    let b = time::SystemTime::now();
    let c = b.duration_since(a);
    

    let mut out_file = File::create("output.txt").unwrap();
  
    match &x {
        Some(temp) => {
            let bytes = temp.borrow().to_string().into_bytes();
            let buffer = &bytes[..];

            out_file.write(buffer);
        }
        None => { }
    }
    
    //println!("{:?}", x);
    println!("{:?}", c);
}
 
