use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
pub enum AstNode{
    Program{body_: VecDeque<AstNode>, start_: Option<Box<AstNode>>},
    Print{expressions_: Vec<AstNode>},
    Binary{operator_:String, left_: Box<AstNode>, right_:Box<AstNode>},
    Number{value_: f32},
    Assignment{name_: String, value_: Box<AstNode>},
    String{value_:String},
    Bool{value_:bool},
    Identifier{name_: String},
    ArrayAccess{name_: String, index_: Box<AstNode>},
    IfStatement{condition_: Box<AstNode>, body_: Box<AstNode>, else_: Box<AstNode>},
    WhileStatement{condition_: Box<AstNode>, body_: Box<AstNode>},
    Block{body_:Box<AstNode>},
    ArrayAssignment{name_: String, values_: Vec<AstNode>},
    Return{value_:Box<AstNode> },
    FunctionDeclaration{name_: String, parameters_: Vec<AstNode>, body_:Box<AstNode>},
    FunctionCall{name_:String,parameters_: Vec<AstNode> },
    None
}