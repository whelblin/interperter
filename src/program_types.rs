use std::collections::VecDeque;
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum ProgramTypes{
    Program{body_: VecDeque<ProgramTypes>},
    Print{expressions_: Vec<ProgramTypes>},
    Binary{operator_:String, left_: Box<ProgramTypes>, right_:Box<ProgramTypes>},
    Number{value_: f32},
    Assignment{name_: String, value_: Box<ProgramTypes>},
    String{value_:String},
    Identifier{name_: String},
    ArrayAccess{name_: String, index_: Box<ProgramTypes>},
    IfStatement{condition_: Box<ProgramTypes>, body_: Box<ProgramTypes>, else_: Box<ProgramTypes>},
    WhileStatement{condition_: Box<ProgramTypes>, body_: Box<ProgramTypes>},
    Block{body_:Box<ProgramTypes>},
    ArrayAssignment{name_: String, values_: Vec<ProgramTypes>},
    None
}