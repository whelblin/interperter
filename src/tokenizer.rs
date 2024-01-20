use std::collections::VecDeque;

use regex::Regex;

use crate::errors::Error;

/// const Patterns that is a trait for the tokenizer struct
trait ConstPattern{
    const PATTERN: [(&'static str, &'static str); 27] =  [
        (r"^\s+", "space"),
        (r"^[0-9]+(\.[0-9]+)?", "number"),   
        (r"^\+", "+"),                        
        (r"^-", "-"),                        
        (r#"^\("#, "("),                      
        (r"^\*", "*"),                      
        (r"^\)", ")"),                       
        (r"^\{", "{"),                         
        (r"^\}", "}"),                         
        (r"^/", "/"),                         
        (r"^;", ";"),                        
        (r"^print", "print"),                 
        (r"^if", "if"),                       
        (r"^else", "else"),                  
        (r"^while", "while"),                
        (r"^==", "=="),                     
        (r"^!=", "!="),                       
        (r"^<=", "<="),                      
        (r"^>=", ">="),                      
        (r"^<", "<"),                        
        (r"^>", ">"),                         
        (r"^=", "="),                         
        (r"^\[", "["),                        
        (r"^\]", "]"),                        
        (r"^,", ","),                         
        (r#"^"([^"]|"")*""#, "string"),        
        (r"^[a-zA-Z_][a-zA-Z0-9_]*", "identifier")
        ];
}
///Structure for the tokenizer that stores the tokens as a vector
/// of strings
pub struct Tokenizer{
     v_: VecDeque<String>
}
/// adds the const PATTERN to the tokenizer struct
impl <'a> ConstPattern for Tokenizer{}
impl Tokenizer{
    /// constructor
    pub fn new()-> Self{
        return Tokenizer{v_ : VecDeque::new()};
    }
    /// debug print function
    pub fn print(&self){
        print!("{:#?}", self.v_);
        
    }
    /// main tokenize function
    /// returns a result of the tokens or an error
    pub fn tokenize(&mut self,source_code: String) -> Result<VecDeque< String>, Error>{
        let mut i =0;
        while i < source_code.len(){
            let mut matched = false;
            for (pattern, _) in Tokenizer::PATTERN{
                let re = Regex::new(pattern).ok().ok_or(Error::PatternError)?;
                let found = re.is_match(&source_code[i..]);
                if found {
                        let result = re.find(&source_code[i..]).ok_or(Error::MatchError)?;
                        let value = result.as_str().to_string();
                        i += value.len();
                        matched = true;
                        if !value.contains(char::is_whitespace){
                            self.v_.push_back(value);
                        }
                        break; 
                }
            }
            if !matched{
                return Err(Error::WrongToken);
            }
        }
        return Ok(self.v_.clone());
        
        
    }
}

