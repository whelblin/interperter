use std::fs;

use crate::{tokenizer::Tokenizer, parser::Parser, errors::Error, executor::Executor};

pub struct Runner<'a>{
    file: Option<&'a str>,
    code: Option<String>,
}

impl<'a> Runner<'a>{
    pub fn new()->Self{
        Self {file: None, code: None}
    }
    pub fn from_file(file_path: &'a str) ->Self{
        Self{file:Some(file_path), code:None}
    }
    pub fn from_code(code_source:String)->Self{
        Self{file:None, code:Some(code_source)}
    }
    pub fn add_file(&mut self, file_path:&'a str){
        self.file = Some(file_path);
    }
    pub fn add_code(&mut self, code_source:String){
        self.code = Some(code_source);
    }
    /// Reads the code from the file and converts it to an internal string
    /// After this, it is ready to be run
    pub fn generate_code(&mut self)->Result<(), std::io::Error>{
        self.code = Some(fs::read_to_string(self.file.unwrap())?);
        Ok(())
    }
    pub fn run(&mut self)-> Result<(), Error>{
        let mut tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(&self.code.clone().unwrap());
        let mut parser = Parser::new(tokens.expect("msg"));
        let ast = parser.parse().expect("Error:");
        let mut executor = Executor::new();
        println!("Parser: {:#?}", ast);
        let _test:_  = executor.execute(&ast).expect("Error:");
        Ok(())
    }
}