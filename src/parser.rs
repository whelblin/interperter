use std::collections::VecDeque;
use crate::program_types;
use program_types::ProgramTypes;



pub struct Parser<'a>{
    tokens_: VecDeque<&'a str>,
}
impl <'a> Parser<'a>{
    pub fn new(tokens: VecDeque<&'a str>)->Self{
        return Parser {tokens_: tokens};
    }
    fn lookahead(&self, offset:usize) -> Option<String>{
        if offset < self.tokens_.len(){
            return Some(self.tokens_[offset].to_string());
        }
        return None;
    }
    fn consume(&mut self )->String{
        let token = self.tokens_.pop_front();
        if token == None{
            return "".to_string();
        }
        return token.unwrap().to_string();
    }
    pub fn print(&self){
        for i in &self.tokens_{
            print!("{} ", i);
        }
    }
    fn parse_statement(&mut self) -> ProgramTypes{
        if self.lookahead(0).unwrap() == "print"{
            return self.parse_print();
        }
        else if self.lookahead(0).unwrap() == "if"{
            return self.parse_if_statement();
        }
        else if self.lookahead(0).unwrap() == "while"{
            return self.parse_while_statement();
        }
        else if self.lookahead(0).unwrap() == "{"{
            return self.parse_block();
        }
        else if self.lookahead(1).unwrap() == "="{
            return self.parse_assignment();
        }
        else{
            return self.parse_expression();
        }
    }
    fn parse_print(&mut self)-> ProgramTypes{
        self.consume(); // print
        self.consume(); // (
            let mut expressions = Vec::new();
            while self.lookahead(0).unwrap() != ";" && self.lookahead(0).unwrap() != ")"{
                expressions.push(self.parse_expression());
            if self.lookahead(0).unwrap() == ","{
                self.consume();
            }
        }
        self.consume();
        return ProgramTypes::Print {expressions_: expressions}; 
        
       
    }
    fn parse_assignment(&mut self)-> ProgramTypes{
        let name = self.consume();
        self.consume(); // =
        let mut values:Vec<ProgramTypes> = Vec::new();
        if self.lookahead(0).unwrap() == "["{
            while self.lookahead(0).unwrap() != "]"{
                self.consume(); // [ and ,
                let value = self.parse_expression();
                values.push(value);
            }
            self.consume(); // ]
            return ProgramTypes::ArrayAssignment {name_: name, values_: values};
        }
        let value = self.parse_expression();
        return ProgramTypes::Assignment {name_:name, value_: Box::new(value) }
    }
    fn parse_expression(&mut self)->ProgramTypes{
        let mut left = self.parse_term();
        while self.lookahead(0).unwrap().contains(['+', '-', '<', '>']) ||
           self.lookahead(0).unwrap().contains("==")||
           self.lookahead(0).unwrap().contains("!=")||
           self.lookahead(0).unwrap().contains("<=")||
           self.lookahead(0).unwrap().contains(">="){
            let operator = self.consume().clone();
            let right = self.parse_term();
            left = ProgramTypes::Binary {operator_: operator, left_: Box::new(left), right_: Box::new(right) };
        }
        return left
    }
    fn parse_term(&mut self) ->ProgramTypes{
        let mut left = self.parse_factor();
        while self.lookahead(0).unwrap().contains(['*', '\\']){
            let operator = self.consume();
            let right = self.parse_factor();
            left = ProgramTypes::Binary {operator_:  operator, left_: Box::new(left), right_: Box::new(right) };

        }
        return left;
    }
    fn parse_factor(&mut self)->ProgramTypes{
        if self.lookahead(0).unwrap() == "("{
            self.consume();
            let expr= self.parse_expression();
            self.consume();
            return expr;
        }
        else if self.lookahead(0).is_some() && self.lookahead(0).unwrap()[0..1].as_bytes()[0].is_ascii_digit(){
            return ProgramTypes::Number {value_: self.consume().parse::<f32>().unwrap()};
        }
        else if self.lookahead(0).is_some() && self.lookahead(0).unwrap().starts_with('"'){
            return ProgramTypes::String {value_: self.consume().trim_matches('"').to_string() };
        }
        else if self.lookahead(0).is_some() && self.lookahead(0).unwrap().as_bytes()[0].is_ascii_alphabetic(){
            if self.lookahead(1).unwrap() == "["{
                return self.parse_array_access();
            }
            return ProgramTypes::Identifier {name_: self.consume() };
        }
        println!("Error at: {:?}", self.lookahead(0).unwrap());
        panic!("Unexpected token");
    }
    fn parse_array_access(&mut self) ->ProgramTypes{
        let name = self.consume();
        self.consume(); // [
        let index = self.parse_expression();
        self.consume(); // ]

        return ProgramTypes::ArrayAccess{name_: name, index_: Box::new(index) };
    }
    fn parse_if_statement(&mut self) ->ProgramTypes{
        self.consume(); // if
        self.consume(); // (
        let condition = self.parse_expression();
        self.consume(); // )
        let if_body = self.parse_statement();
        let mut  else_body = ProgramTypes::None;
        if self.lookahead(0).is_some() && self.lookahead(0).unwrap() == "else"{
            self.consume();
            else_body = self.parse_statement();
        }
        return ProgramTypes::IfStatement {condition_: Box::new(condition), body_: Box::new(if_body), else_: Box::new(else_body)};
    }
    fn parse_while_statement(&mut self) ->ProgramTypes{
        self.consume(); // while
        self.consume(); // (
        let condition = self.parse_expression();
        self.consume(); // )
        let while_body = self.parse_statement();       
        return ProgramTypes::WhileStatement {condition_: Box::new(condition), body_: Box::new(while_body) };
    }
    fn parse_block(&mut self)->ProgramTypes{
        self.consume(); // {
        let body = self.parse();
        self.consume(); // }
        return ProgramTypes::Block {body_: Box::new(body) };
    }
    pub fn parse(&mut self) -> ProgramTypes{
        //self.push("testing new system");
        //self.push("hello");
        let mut stmts: VecDeque<ProgramTypes> = VecDeque::new();
        while self.lookahead(0) != None && self.lookahead(0).unwrap() != "}"{
            stmts.push_back(self.parse_statement());
            if self.lookahead(0).is_some() && self.lookahead(0).unwrap() == ";"{
                self.consume();
            }
        }
        return ProgramTypes::Program {body_: stmts }
    }
}
