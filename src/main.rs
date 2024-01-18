pub mod tokenizer;
pub mod parser;
pub mod program_types;
pub mod types;
pub mod executor;

use tokenizer::Tokenizer;
use parser::Parser;

use crate::executor::Executor;
fn main() {
    let code = r#"
    x = [1,2,3];
    y = 5;
    print(x, y);
    print(y);
    "#;

    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(code);
    
    let mut parser = Parser::new(tokenizer.get_tokens());
    let ast = parser.parse();
    //println!("{:#?}", ast);

    let mut executor = Executor::new();
    executor.execute(&ast);
    //executor.print_env();
    //parser.print();

}
