use std::collections::{HashMap, LinkedList};

use crate::{program_types::ProgramTypes, types::Types};
// used to store variables in the env

pub struct Executor{
    stack_: LinkedList<HashMap<String, Types>> // stack of envs
}

impl Executor{
    pub fn new()->Self{
        let mut result = Executor{stack_:LinkedList::new() };
        result.stack_.push_back(HashMap::new()); // main env
        return result
    }
    pub fn print_env(&self){
        println!("{:?}", self.stack_);
    }
    pub fn execute(&mut self, ast_: &ProgramTypes)->Types{
        match ast_.clone(){
            ProgramTypes::Program {   body_ } =>{
               for stm in body_{
                    self.execute(&stm);
               }
               return Types::None;
            }
            ProgramTypes::Print {   expressions_ } => {
                for expr in expressions_{
                    print!("{} ", self.execute(&expr));
                }
                println!();
                return Types::None;
            },
            ProgramTypes::Binary {   operator_, left_, right_ } =>{
                let new_left = self.execute(&left_); 
                let new_right = self.execute(&right_);
                if operator_ == "+"{
                        return new_left + new_right;
                }
                else if operator_ == "-"{
                    return new_left - new_right;
                }
                else if operator_ == "*"{
                    return new_left * new_right;
                }
                else if operator_ == "/"{
                    return new_left / new_right;
                }
                else if operator_ == "<"{
                    return Types::Bool(new_left < new_right);
                }
                else if operator_ == "<="{
                    return Types::Bool(new_left <= new_right);
                }
                else if operator_ == ">"{
                    return Types::Bool(new_left > new_right);
                }
                else if operator_ == ">="{
                    return Types::Bool(new_left >= new_right);
                }
                else if operator_ == "=="{
                    return Types::Bool(new_left == new_right);
                }
                else if operator_ == "!="{
                    return Types::Bool(new_left != new_right);
                }
                return Types::None;
            },
            ProgramTypes::Number {   value_ } => {return Types::Number(value_);},
            ProgramTypes::String {   value_ } => {return Types::String(value_);},
            ProgramTypes::Identifier {   name_ } => {
                // goes through each env on the stack to ensure closures
                for itr in self.stack_.iter().rev(){
                    let value =  itr.get(&name_);
                    if value.is_some(){
                        return value.unwrap().clone();
                    }
                }
                return Types::None;
            }
            ProgramTypes::Assignment {   name_, value_ } => {
                let value  = self.execute(&value_);
                self.stack_.back_mut().expect("No stack left").insert(name_, value);
                return Types::None;
            },
            ProgramTypes::ArrayAccess {   name_, index_ } =>{
                // goes through each env on the stack to ensure closures
                let index = self.execute(&index_);
                if let Types::Number(num) = index{
                    for itr in self.stack_.iter().rev(){
                        let value =  itr.get(&name_);
                        if value.is_some(){
                            if let Types::Array(vec) = value.unwrap(){
                                return vec[num as usize].clone();
    
                            }
                        }
                    }

                }
                return Types::None;
            },
            ProgramTypes::IfStatement {   condition_, body_, else_ } =>{
                let condition = self.execute(&condition_);
                if let Types::Bool(bool) = condition{
                    if bool{
                        self.execute(&body_);
                    }
                    else{
                        self.execute(&else_);
                    }
                }
                return Types::None;
                
            },
            ProgramTypes::WhileStatement {condition_, body_ } => {
                let condition_copy = *condition_.clone();
                let mut condition = self.execute(&condition_copy);
                while let Types::Bool(_) = condition{
                    self.execute(&body_);
                    condition = self.execute(&condition_copy);
                }
                return Types::None;
            },
            ProgramTypes::Block {   body_ } => {
                self.stack_.push_back(HashMap::new());
                self.stack_.push_back(HashMap::new());
                self.execute(&body_);
                return Types::None;

            },
            ProgramTypes::None => Types::None,
            ProgramTypes::ArrayAssignment {   values_, name_ } =>{
                
                let mut array = Vec::new();
                    for item in values_{
                        let value = self.execute(&item);
                        array.push(value);
                    }
                
                self.stack_.back_mut().expect("No stack left").insert(name_, Types::Array(array));    
                return Types::None ;
            },
        }
    }
}