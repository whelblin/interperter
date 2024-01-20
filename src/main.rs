pub mod tokenizer;
pub mod parser;
pub mod program_types;
pub mod types;
pub mod executor;
pub mod errors;

use tokenizer::Tokenizer;
use parser::Parser;

use crate::executor::Executor;
fn main() {
    /*
    x = [1,2,3];
    y = 5;
    print(x);
    print(y); */
    let code = r#"
    x = [1,2,3];
    y = 5;
    print(x);
    print(y);
    {
        x = 5;
    }
    p = [1,"hello",3];
    print(p);
    print(p[1]);
    "#;

    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(code.to_string());
    
    let mut parser = Parser::new(tokens.expect("msg"));
    let ast = parser.parse().expect("Error:");
    //println!("{:#?}", ast);

    let mut executor = Executor::new();
    executor.execute(&ast);
    executor.print_env();
    //parser.print();

}
