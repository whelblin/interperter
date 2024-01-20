use std::collections::VecDeque;
use crate::{program_types, errors};
use program_types::ProgramTypes;

use errors::Error;

pub struct Parser{
    tokens_: VecDeque<String>,
}
impl Parser{
    pub fn new(tokens: VecDeque<String>)->Self{
        return Parser {tokens_: tokens};
    }
    fn lookahead(&self, offset:usize) -> Option<String>{
        if offset < self.tokens_.len(){
            return Some(self.tokens_[offset].to_string());
        }
        return None;
    }
    fn consume(& mut self )->Option<String>{
        let token = self.tokens_.pop_front()?;
        return Some(token.to_string());
    }
    pub fn print(&self){
        for i in &self.tokens_{
            print!("{} ", i);
        }
    }
    fn parse_statement(&mut self) -> Result<ProgramTypes, Error>{
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
    
    fn parse_print(&mut self)-> Result<ProgramTypes, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // print
        self.consume().ok_or(Error::UnexpectedToken)?; // (
            let mut expressions = Vec::new();
            while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?  != ";"
                && self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != ")"{
                    expressions.push(self.parse_expression()?);
            if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == ","{
                self.consume().ok_or(Error::UnexpectedToken)?;
            }
        }
        self.consume().ok_or(Error::UnexpectedToken)?;
        return Ok(ProgramTypes::Print {expressions_: expressions}); 
        
       
    }
    fn parse_assignment(&mut self)-> Result<ProgramTypes, Error>{
        let name = self.consume().ok_or(Error::PeekOutOfBounds)?;
        self.consume().ok_or(Error::UnexpectedToken)?; // =
        let mut values:Vec<ProgramTypes> = Vec::new();
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "["{
            while self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != "]"{
                self.consume().ok_or(Error::UnexpectedToken)?; // [ and ,
                let value = self.parse_expression()?;
                values.push(value);
            }
            self.consume().ok_or(Error::UnexpectedToken)?; // ]
            return Ok(ProgramTypes::ArrayAssignment {name_: name, values_: values});
        }
        let value = self.parse_expression()?;
        return Ok(ProgramTypes::Assignment {name_:name, value_: Box::new(value) })
    }
    fn parse_expression(&mut self)->Result<ProgramTypes, Error>{
        let mut left = self.parse_term()?;
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(['+', '-', '<', '>']) ||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("==")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("!=")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("<=")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(">="){
            let operator = self.consume().ok_or(Error::UnexpectedToken)?.clone();
            let right = self.parse_term()?;
            left = ProgramTypes::Binary {operator_: operator, left_: Box::new(left), right_: Box::new(right) };
        }
        return Ok(left)
    }
    fn parse_term(&mut self) ->Result<ProgramTypes, Error>{
        let mut left = self.parse_factor()?;
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(['*', '\\']){
            let operator = self.consume().ok_or(Error::UnexpectedToken)?;
            let right = self.parse_factor()?;
            left = ProgramTypes::Binary {operator_:  operator, left_: Box::new(left), right_: Box::new(right) };

        }
        return Ok(left);
    }
    fn parse_factor(&mut self)->Result<ProgramTypes, Error>{
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "("{
            self.consume().ok_or(Error::UnexpectedToken)?;
            let expr= self.parse_expression()?;
            self.consume().ok_or(Error::UnexpectedToken)?;
            return Ok(expr);
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?[0..1].as_bytes()[0].is_ascii_digit(){
            return Ok(ProgramTypes::Number {value_: self.consume().ok_or(Error::UnexpectedToken)?.parse::<f32>().unwrap()});
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.starts_with('"'){
            return Ok(ProgramTypes::String {value_: self.consume().ok_or(Error::UnexpectedToken)?.trim_matches('"').to_string() });
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.as_bytes()[0].is_ascii_alphabetic(){
            if self.lookahead(1).ok_or(Error::PeekOutOfBounds)? == "["{
                return self.parse_array_access();
            }
            return Ok(ProgramTypes::Identifier {name_: self.consume().ok_or(Error::UnexpectedToken)? });
        }
        return Err(Error::UnexpectedToken);
    }
    fn parse_array_access(&mut self) ->Result<ProgramTypes, Error>{
        let name = self.consume().ok_or(Error::UnexpectedToken)?;
        self.consume().ok_or(Error::UnexpectedToken)?; // [
        let index = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // ]

        return Ok(ProgramTypes::ArrayAccess{name_: name, index_: Box::new(index) });
    }
    fn parse_if_statement(&mut self) ->Result<ProgramTypes, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // if
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let condition = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        let if_body = self.parse_statement()?;
        let mut  else_body = ProgramTypes::None;
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "else"{
            self.consume().ok_or(Error::UnexpectedToken)?;
            else_body = self.parse_statement()?;
        }
        return Ok(ProgramTypes::IfStatement {condition_: Box::new(condition), body_: Box::new(if_body), else_: Box::new(else_body)});
    }
    fn parse_while_statement(&mut self) ->Result<ProgramTypes, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // while
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let condition = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        let while_body = self.parse_statement()?;       
        return Ok(ProgramTypes::WhileStatement {condition_: Box::new(condition), body_: Box::new(while_body) });
    }
    fn parse_block(&mut self)->Result<ProgramTypes, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // {
        let body = self.parse()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // }
        return Ok(ProgramTypes::Block {body_: Box::new(body) });
    }
    pub fn parse(&mut self) -> Result<ProgramTypes, Error>{
        let mut stmts: VecDeque<ProgramTypes> = VecDeque::new();
        while self.lookahead(0).is_some_and(|token| {token !="}"}){
            stmts.push_back(self.parse_statement()?);
            if self.lookahead(0).is_some_and(|token| { token == ";"}){
                self.consume().ok_or(Error::UnexpectedToken)?;
            }
        }
        return Ok(ProgramTypes::Program {body_: stmts })
    }
}
