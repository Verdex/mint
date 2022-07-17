
mod ast;
mod parsing;
mod evaling;

fn main() {
    use std::io::{stdout, stdin, Write};

    let mut c = evaling::context::Context::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        // TODO if input ends with a semi colon (after trim) then it has to parse
        // TODO if it parses with no semi colon then it's some meaningful expr
        stdin().read_line(&mut input).unwrap();

        let tokens = parsing::tokenizer::tokenize(&input).unwrap();
        let top = parsing::parser::parse(&tokens).unwrap();
        let data = evaling::evaler::eval(top, &mut c).unwrap();

        println!("{}", evaling::display::print_data(&data, &c).unwrap());
    } 
}
