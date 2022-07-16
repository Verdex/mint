
use motif::{alt, group, pred, seq, cases};
use motif::MatchError;
use crate::data::{ TMeta 
                 , Token
                 , Ast
                 };

pub fn parse(tokens : &Vec<Token>) -> Result<Vec<Ast>, MatchError> {
    let mut x = tokens.iter().enumerate();

    alt!( ast<'a>: &'a Token => Ast = parse_cons_def);

    let mut ret = vec![];
    loop {
        match ast(&mut x) {
            Ok(t) => ret.push(t),
            Err(MatchError::ErrorEndOfFile) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(ret)
}


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_first_parse {
        ($name:ident: $input:expr => $expected:pat => $x:block) => {
            #[test]
            fn $name() -> Result<(), MatchError> {
                use super::super::tokenizer::tokenize;
                if let Ok(tokens) = tokenize($input) {
                    let mut output = internal_parse(&tokens)?;

                    assert_eq!( output.len(), 1 );

                    if let Some($expected) = output.pop() {
                        $x
                    }
                    else {
                        panic!("instead of expected pattern found: {:?}\nfrom tokens: {:?}", output, tokens);
                    }
                    Ok(())
                }
                else {
                    panic!( "tokenize failed in test parse" );
                }
            }
        };
    }
}
