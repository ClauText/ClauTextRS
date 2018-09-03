
//!
//! 
//!
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::VecDeque;
use std::time;

pub struct Utility
{
    //
}

#[derive(Debug)]
pub struct Token
{
    pub data : String,
    pub isComment : bool
}

#[inline]
fn isWhitespace(ch : char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

impl Utility 
{
    //! reserve
    pub fn reserve(inFile : &mut File, result : &mut VecDeque<Token>) {
        let mut reader = BufReader::new(inFile);
        let mut data = String::new();
//let a = time::SystemTime::now();
        for line in reader.lines()  {
            let temp = line.unwrap();
            data += &temp;
            data += "\n";
        }
//let b = time::SystemTime::now();
            // check empty and end of file?
        //    let c = b.duration_since(a);println!("{:?}", c);
        let mut state = 0;
        let mut token_first = 0;
        let mut token_last = 0;
        let statement = data.as_str().chars();
        let size = data.len();
        //println!("check");

        let mut i = 0;
        let mut token = String::new();
        token.reserve(1024);
        let mut before_ch = '\0';
        let mut isComment = false;

        for now_ch in statement {
            if state != 0 && '\n' == now_ch {
                state = 0;
                token_last = i;
            }
            else if isComment && !( '\n' == now_ch || '\0' == now_ch ) {
                token.push(now_ch);
            }
            else if isComment && ( '\n' == now_ch || '\0' == now_ch ) {
                let mut _temp = "#".to_string();
                _temp += &token;

                let temp = Token{data : _temp.clone(), isComment : true};

                result.push_back(temp); token.clear(); 
                isComment = false;
            }
            else if 0 == state && '\"' == now_ch  {
                token.push(now_ch);
                state = 1;
                token_last = i;
            }
            else if 1 == state && '\\' == before_ch  && '\"' == now_ch {
                token.push(now_ch);
                token_last = i;
            }
            else if 1 == state && '\"' == now_ch {
                token.push(now_ch);
                state = 0; token_last = i;

                //result.push_back(substr(statement, token_first, token_last - token_first + 1));
                //token_first = i + 1;
            }
            else if 0 == state && '=' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = Token{data : token.clone(), isComment : false};
                    result.push_back(temp); token.clear();
                }
                result.push_back(Token{data : String::from("="), isComment : false});
                token_first = i + 1;
            }
            else if 0 == state && isWhitespace(now_ch) { // isspace ' ' \t \r \n , etc... ?
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = Token{data : token.clone(), isComment : false};
                    result.push_back(temp); token.clear();
                }
                token_first = i + 1;
            }
            else if 0 == state && '{' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = Token{data : token.clone(), isComment : false};
                    result.push_back(temp); token.clear();
                }

                result.push_back(Token{data : String::from("{"), isComment : false});
                token_first = i + 1;
            }
            else if 0 == state && '}' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = Token{data : token.clone(), isComment : false};
                    result.push_back(temp); token.clear();
                }
                result.push_back(Token{data : String::from("}"), isComment : false});
                token_first = i + 1;
            }
            else if 0 == state && '#' == now_ch { // different from load_data_from_file
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = Token{data : token.clone(), isComment : false};
                    result.push_back(temp); token.clear();
                }

                isComment = true;
            }
            else {
                token.push(now_ch);
            }

            i = i + 1;
            before_ch = now_ch;
        }

        if token.len() > 0
        {
            let temp = Token{data : token.clone(), isComment : false};
            result.push_back(temp); token.clear();
        }
    }

    // need to check Comment
    pub fn top(x : &VecDeque<Token>) -> &String
    {
        if(x.len() <= 0) {
            println!("top error");
        }
         & x.front().unwrap().data 
    }
    // need to check Comment
    pub fn pop(x :&mut VecDeque<Token>) -> String
    {
        if(x.len() <= 0) {
            println!("pop error");
        }
        x.pop_front().unwrap().data
    }
    // need to check Comment
    pub fn lookup(x :&VecDeque<Token>, offset : usize) -> &String
    {   
        let mut result : String;
        {
            let y = x.iter().nth(offset);

            match y {
                Item => {
                    //println!("chk");
                    result = "ok".to_string();
                },
                None => {
                    println!("none");
                    result = "none".to_string();
                },
                Error => {
                    println!("error");
                    result = "error".to_string();
                }
            }   
        }
        if result == "ok" {
            & x.iter().nth(offset).unwrap().data
        }
        else {
            panic!("chk");
        }
    }
}

