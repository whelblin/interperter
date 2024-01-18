use std::collections::VecDeque;

use regex::Regex;
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
pub struct Tokenizer<'a>{
     v_: VecDeque<&'a str>
}
impl <'a> ConstPattern for Tokenizer <'a>{}
impl <'a> Tokenizer<'a>{
    pub fn new()-> Self{
        return Tokenizer{v_ : VecDeque::new()};
    }
    pub fn print(&self){
        print!("{:#?}", self.v_);
        
    }
    pub fn tokenize(&mut self,source_code: &'a str){
        let mut i =0;
        while i < source_code.len(){
            let mut matched = false;
            for (pattern, _) in Tokenizer::PATTERN{
                let re = Regex::new(pattern).unwrap();
                let found = re.is_match(&source_code[i..]);
                if found {
                        let result = re.find(&source_code[i..]).unwrap();
                    let value = result.as_str();
                    i += value.len();
                    matched = true;
                    if !value.contains(char::is_whitespace){
                    self.v_.push_back(value);
                    }
                    break; 
                }
            }
            if !matched{
                panic!("not a possible token");
            }
        }
        
    }
    pub fn get_tokens(&self)->VecDeque<&str>{
        return self.v_.clone();
    }
}

