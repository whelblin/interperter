pub mod tokenizer;
pub mod parser;
pub mod program_types;
pub mod types;
pub mod executor;
pub mod errors;
pub mod runner;

use std::{env, fmt::Error};

use runner::Runner;
use tokenizer::Tokenizer;
use parser::Parser;

use crate::executor::Executor;
fn main(){
    let args: Vec<String> = env::args().collect();
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
    if args.len() > 1{
        let mut runner = Runner::from_file(args[1].as_str());
        let temp = runner.generateCode();
        runner.run();
    }


}
