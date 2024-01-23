use std::collections::VecDeque;
use crate::{program_types, errors};
use program_types::AstNode;
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
    fn parse_statement(&mut self) -> Result<AstNode, Error>{
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
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "func" {
            return self.parse_function();
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "return"{
            return self.parse_return();
        }
        else{
            return self.parse_expression();
        }
    }

    fn parse_return(&mut self)->Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // return
        let value = self.parse_statement()?;
        return Ok(AstNode::Return { value_: Box::new(value) });
        }

    fn parse_function(&mut self)-> Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // func
        let name_ = self.consume().ok_or(Error::UnexpectedToken)?; // name
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let mut params = Vec::new();
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != ")"{
            params.push(self.parse_statement()?);
            if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == ","{
                self.consume().ok_or(Error::UnexpectedToken)?; // ,
            }
        }
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        let body = self.parse_statement()?; // should be the body
        return Ok(AstNode::FunctionDeclaration { name_: name_, parameters_: params, body_: Box::new(body)});
    }
    fn parse_print(&mut self)-> Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // print
        self.consume().ok_or(Error::UnexpectedToken)?; // (
            let mut expressions = Vec::new();
            while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?  != ";"
                && self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != ")"{
                    expressions.push(self.parse_expression()?);
            if self.lookahead(0).is_some() && self.lookahead(0).unwrap() == ","{
                self.consume().ok_or(Error::UnexpectedToken)?;
            }
        }
        self.consume().ok_or(Error::TestError)?;
        return Ok(AstNode::Print {expressions_: expressions}); 
        
       
    }
    fn parse_assignment(&mut self)-> Result<AstNode, Error>{
        let name = self.consume().ok_or(Error::PeekOutOfBounds)?;
        self.consume().ok_or(Error::UnexpectedToken)?; // =
        let mut values:Vec<AstNode> = Vec::new();
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "["{
            while self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != "]"{
                self.consume().ok_or(Error::UnexpectedToken)?; // [ and ,
                let value = self.parse_expression()?;
                values.push(value);
            }
            self.consume().ok_or(Error::UnexpectedToken)?; // ]
            return Ok(AstNode::ArrayAssignment {name_: name, values_: values});
        }
        let value = self.parse_expression()?;
        return Ok(AstNode::Assignment {name_:name, value_: Box::new(value) })
    }
    fn parse_expression(&mut self)->Result<AstNode, Error>{
        let mut left = self.parse_term()?;
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(['+', '-', '<', '>']) ||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("==")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("!=")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("<=")||
           self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(">="){
            let operator = self.consume().ok_or(Error::UnexpectedToken)?.clone();
            let right = self.parse_term()?;
            left = AstNode::Binary {operator_: operator, left_: Box::new(left), right_: Box::new(right) };
        }
        return Ok(left)
    }
    fn parse_term(&mut self) ->Result<AstNode, Error>{
        let mut left = self.parse_factor()?;
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains(['*', '\\']){
            let operator = self.consume().ok_or(Error::UnexpectedToken)?;
            let right = self.parse_factor()?;
            left = AstNode::Binary {operator_:  operator, left_: Box::new(left), right_: Box::new(right) };

        }
        return Ok(left);
    }
    fn parse_factor(&mut self)->Result<AstNode, Error>{
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "("{
            self.consume().ok_or(Error::UnexpectedToken)?;
            let expr= self.parse_expression()?;
            self.consume().ok_or(Error::UnexpectedToken)?;
            return Ok(expr);
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?[0..1].as_bytes()[0].is_ascii_digit(){
            return Ok(AstNode::Number {value_: self.consume().ok_or(Error::UnexpectedToken)?.parse::<f32>().unwrap()});
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.starts_with('"'){
            return Ok(AstNode::String {value_: self.consume().ok_or(Error::UnexpectedToken)?.trim_matches('"').to_string() });
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("True"){
            return Ok(AstNode::Bool { value_:  self.consume().ok_or(Error::UnexpectedToken)?.contains("True")})
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("False"){
            return Ok(AstNode::Bool { value_:  self.consume().ok_or(Error::UnexpectedToken)?.contains("True")})
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.contains("None"){
            return Ok(AstNode::None)
        }
        else if self.lookahead(0).ok_or(Error::PeekOutOfBounds)?.as_bytes()[0].is_ascii_alphabetic(){
            if self.lookahead(1).ok_or(Error::PeekOutOfBounds)? == "["{
                return self.parse_array_access();
            }
            if self.lookahead(1).ok_or(Error::PeekOutOfBounds)? == "("{ // function call
                return self.parse_function_call()

            }
            return Ok(AstNode::Identifier {name_: self.consume().ok_or(Error::UnexpectedToken)? });
        }
        return Err(Error::UnexpectedToken);
    }

    fn parse_function_call(&mut self)->Result<AstNode, Error>{
        let name = self.consume().ok_or(Error::UnexpectedToken)?; // function name
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let mut params = Vec::new();
        while self.lookahead(0).ok_or(Error::PeekOutOfBounds)? != ")"{
            params.push(self.parse_statement()?);
            if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == ","{
                self.consume().ok_or(Error::UnexpectedToken)?; // ,
            }
        }
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        return Ok(AstNode::FunctionCall { name_: name, parameters_: params })
    }
    fn parse_array_access(&mut self) ->Result<AstNode, Error>{
        let name = self.consume().ok_or(Error::UnexpectedToken)?;
        self.consume().ok_or(Error::UnexpectedToken)?; // [
        let index = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // ]

        return Ok(AstNode::ArrayAccess{name_: name, index_: Box::new(index) });
    }
    fn parse_if_statement(&mut self) ->Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // if
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let condition = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        let if_body = self.parse_statement()?;
        let mut  else_body = AstNode::None;
        if self.lookahead(0).ok_or(Error::PeekOutOfBounds)? == "else"{
            self.consume().ok_or(Error::UnexpectedToken)?;
            else_body = self.parse_statement()?;
        }
        return Ok(AstNode::IfStatement {condition_: Box::new(condition), body_: Box::new(if_body), else_: Box::new(else_body)});
    }
    fn parse_while_statement(&mut self) ->Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // while
        self.consume().ok_or(Error::UnexpectedToken)?; // (
        let condition = self.parse_expression()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // )
        let while_body = self.parse_statement()?;       
        return Ok(AstNode::WhileStatement {condition_: Box::new(condition), body_: Box::new(while_body) });
    }
    fn parse_block(&mut self)->Result<AstNode, Error>{
        self.consume().ok_or(Error::UnexpectedToken)?; // {
        let body = self.parse_block_body()?;
        self.consume().ok_or(Error::UnexpectedToken)?; // }
        return Ok(AstNode::Block {body_: Box::new(body) });
    }
     fn parse_block_body(&mut self)-> Result<AstNode, Error>{
        let mut stmts: VecDeque<AstNode> = VecDeque::new();
        while self.lookahead(0).is_some_and(|token| {token !="}"}){
            stmts.push_back(self.parse_statement()?);
            if self.lookahead(0).is_some_and(|token| { token == ";"}){
                self.consume().ok_or(Error::UnexpectedToken)?;
            }
        }
        return Ok(AstNode::Program {body_: stmts, start_:None});
    }
    pub fn parse(&mut self) -> Result<AstNode, Error>{
        let mut stmts: VecDeque<AstNode> = VecDeque::new();
        while self.lookahead(0).is_some_and(|token| {token !="}"}){
            stmts.push_back(self.parse_statement()?);
            if self.lookahead(0).is_some_and(|token| { token == ";"}){
                self.consume().ok_or(Error::UnexpectedToken)?;
            }
        }//Some(Box::new(AstNode::FunctionCall { name_: "main".to_string(), parameters_: Vec::new() }))
        return Ok(AstNode::Program {body_: stmts, start_:Some(Box::new(AstNode::FunctionCall { name_: "main".to_string(), parameters_: Vec::new() }))});
    }
}
