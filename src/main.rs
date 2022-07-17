
mod ast;
mod parsing;
mod evaling;

fn main() {
    use std::io::{stdout, stdin, Write};

    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let tokens = parsing::tokenizer::tokenize(&input).unwrap();
        let top = parsing::parser::parse(&tokens).unwrap();
        let mut c = evaling::context::Context::new();
        let data = evaling::evaler::eval(top, &mut c).unwrap();

        println!("{}", evaling::display::print_data(&data, &c));
    } 
}
