use std::collections::{HashMap, LinkedList};

use crate::{program_types::AstNode, types::Types, errors::Error};

pub struct Executor{
    stack_: LinkedList<HashMap<String, Types>> // stack of envs
}

impl Executor{
    pub fn new()->Self{
        Self{stack_: LinkedList::from([HashMap::new()])}
    }
    pub fn print_env(&self){
        println!("ENV: {:#?}", self.stack_);
    }
    pub fn execute(&mut self, ast_: &AstNode)->Result<Types, Error>{
        match ast_.clone(){
            AstNode::Program {   body_ } =>{
               for stm in body_{
                    self.execute(&stm)?;
               }
               return Ok(Types::None);
            }
            AstNode::Print {   expressions_ } => {
                for expr in expressions_{
                    print!("{} ", self.execute(&expr)?);
                }
                println!(); // endline for the expression
                return Ok(Types::None);
            },
            AstNode::Binary {   operator_, left_, right_ } =>{
                let new_left = self.execute(&left_)?; 
                let new_right = self.execute(&right_)?;
                _ = match operator_.as_str(){
                    "+"=>{return Ok(new_left + new_right);},
                    "-"=>{return Ok(new_left - new_right);},
                    "*"=>{return Ok(new_left * new_right);},
                    "/"=>{return Ok(new_left / new_right);},
                    "<"=>{return Ok(Types::Bool(new_left < new_right));},
                    "<="=>{return Ok(Types::Bool(new_left <= new_right));},
                    ">"=>{return Ok(Types::Bool(new_left > new_right));},
                    ">="=>{return Ok(Types::Bool(new_left >= new_right));},
                    "=="=>{return Ok(Types::Bool(new_left == new_right));},
                    "!="=>{return Ok(Types::Bool(new_left != new_right));},
                    _ => {return Ok(Types::None);}
                };
            },
            AstNode::Number {   value_ } => {return Ok(Types::Number(value_));},
            AstNode::String {   value_ } => {return Ok(Types::String(value_));},
            AstNode::Identifier {   name_ } => {
                // goes through each env on the stack to ensure closures
                for itr in self.stack_.iter().rev(){
                    let value =  itr.get(&name_);
                    if value.is_some(){
                        return Ok(value.unwrap().clone());
                    }
                }
                return Err(Error::IdentifierDoesNotExist);
            }
            AstNode::Assignment {   name_, value_ } => {
                let value  = self.execute(&value_)?;
                self.stack_.back_mut().expect("No stack left").insert(name_, value);
                return Ok(Types::None);
            },
            AstNode::ArrayAccess {   name_, index_ } =>{
                // goes through each env on the stack to ensure closures
                let index = self.execute(&index_)?;
                if let Types::Number(num) = index{
                    for itr in self.stack_.iter().rev(){
                        let value =  itr.get(&name_);
                        if value.is_some(){
                            if let Types::Array(vec) = value.unwrap(){
                                return Ok(vec[num as usize].clone());
    
                            }
                        }
                    }

                }
                return Err(Error::IdentifierDoesNotExist);
            },

            AstNode::IfStatement {   condition_, body_, else_ } =>{
                let condition = self.execute(&condition_)?;
                if let Types::Bool(bool) = condition{
                    if bool{
                        self.execute(&body_)?;
                    }
                    else{
                        self.execute(&else_)?;
                    }
                }
                return Ok(Types::None);
                
            },
            AstNode::WhileStatement {condition_, body_ } => {
                let condition_copy = *condition_.clone();
                let mut condition = self.execute(&condition_copy)?;
                while let Types::Bool(_) = condition{
                    self.execute(&body_)?;
                    condition = self.execute(&condition_copy)?;
                }
                return Ok(Types::None);
            },
            AstNode::Block {   body_ } => {
                self.stack_.push_back(HashMap::new());
                self.execute(&body_)?;
                self.stack_.pop_back(); // remove the stack once done
                return Ok(Types::None);

            },
            AstNode::None => Ok(Types::None),
            AstNode::ArrayAssignment {   values_, name_ } =>{
                
                let test = values_.iter() // converts the vector into an iterator 
                        .map(|x| self.execute(x).expect("Error:")) // call execute on each element
                        .collect(); // collects them back into an Type::Array
                self.stack_.back_mut().expect("No stack left").insert(name_, test);    
                return Ok(Types::None) ;
            },
            AstNode::Bool { value_ } => return Ok(Types::Bool(value_)),
            AstNode::FunctionDeclaration { name_, parameters_, body_ } =>{
                self.stack_.back_mut().ok_or(Error::StackOut)?.insert(name_, Types::Function {  paramters: parameters_, body_: *body_});
                return Ok(Types::None);
            }
            AstNode::FunctionCall { name_, parameters_ } => {
                let eval_params:Vec<Types> = parameters_.iter().map(|x| self.execute(x).expect("ERROR:")).collect();
                let mut new_env:HashMap<String, Types> = HashMap::new();
                
                for itr in self.stack_.clone().iter().rev(){
                    let function = itr.get(&name_);
                    if function.is_some(){
                        if let Types::Function { paramters, body_ } = function.unwrap(){
                            if paramters.len() != parameters_.len(){
                                return Err(Error::FunctionParameterUnmatch);
                            }
                            println!("Parms: {:?}",eval_params);
                            for (index, item) in paramters.iter().enumerate(){
                                if let AstNode::Identifier { name_ } = item{
                                    println!("Name: {:?}",name_);
                                    new_env.insert(name_.clone(), eval_params[index].clone());
                                }
                            }
                            
                            
                            self.stack_.push_back(new_env);
                            self.execute(body_)?;
                            self.stack_.pop_back();
                            return Ok(Types::None);
                        }
                    }
                    return Ok(Types::None);

                }
                return Ok(Types::None);
                
            },
        }
    }
}