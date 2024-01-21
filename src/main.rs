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
    func test(x, y){
        print("The addtion of x and y is:", x+y);
        func test1(x){
            print(x+1);
        }

        test1(y);
    
    }
    
    print("hello world";
    z = 10;
    test(5,z);

    "#;

    let mut tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(code.to_string());
    //tokenizer.print();
    let mut parser = Parser::new(tokens.expect("msg"));
    let ast = parser.parse().expect("Error:");
    //println!("{:#?}", ast);

    let mut executor = Executor::new();
    let test  = executor.execute(&ast).expect("Error:");
    println!("{:?}", test);
    //executor.print_env();
    //parser.print();

}
