
use motif::{alt, group, pred, seq, cases};
use motif::MatchError;
use crate::ast::{ TMeta 
                , Token
                , Top 
                , Let
                , Expr
                , Data
                , Pat
                };

pub fn parse(tokens : &Vec<Token>) -> Result<Top, MatchError> {
    let mut x = tokens.iter().enumerate();

    let top = parse_top(&mut x)?;

    match x.next() {
        Some((i, v)) => Err(MatchError::Fatal(i)),
        None => Ok(top),
    }
}

seq!(parse_top<'a>: &'a Token => Top = lets <= * parse_let, expr <= parse_expr, {
    Top { lets, expr }
});

group!(parse_let<'a>: &'a Token => Let = |input| {

    pred!(is_let<'a>: &'a Token => () = 
        |tok| if let Token::LowerSymbol(_, sym) = tok {
            sym == "let"
        }
        else {
            false
        }
        
        => { () });

    seq!(main<'a>: &'a Token => Let = is_let
                                    , pattern <= parse_pattern
                                    , Token::Equal(_)
                                    , expr <= parse_expr
                                    , Token::Semicolon(_)
                                    , { Let { pattern, expr } });

    main(input)
});

group!(parse_pattern<'a>: &'a Token => Pat = |input| {
    Err(MatchError::FatalEndOfFile)
});

group!(parse_expr<'a>: &'a Token => Expr = |input| {
    seq!(data<'a>: &'a Token => Expr = data <= parse_data, { Expr::Data(data) });
    alt!(main<'a>: &'a Token => Expr = data);

    main(input)
});
// match
// let
// function 
// let X = fun(P, P, P) { };
// let { X, Y, Z } = { 1, 2, blah };
/*

let X = fun(A) {
    let Z = Blah(A);
    Z
};



*/

group!(parse_data<'a>: &'a Token => Data = |input| {

    seq!(comma_data<'a>: &'a Token => Data = Token::Comma(_), data <= ! parse_data, { data });

    seq!(data_list<'a>: &'a Token => Data = Token::LSquare(_)
                                          , _1 <= parse_data 
                                          , r <= * comma_data
                                          , ! Token::RSquare(_)
                                          , {
        let mut rest = r;
        rest.insert(0, _1);
        Data::List(rest)
    });

    seq!(number<'a>: &'a Token => Data = n <= Token::Number(_, _), { 
        if let Token::Number(_, number) = n {
            Data::Number(*number) 
        }
        else {
            panic!("reflective failure");
        }
    });

    seq!(string<'a>: &'a Token => Data = s <= Token::String(_, _), { 
        if let Token::String(_, string) = s {
            Data::String(string.into()) 
        }
        else {
            panic!("reflective failure");
        }
    });

    seq!(symbol<'a>: &'a Token => Data = symbol <= Token::LowerSymbol(_, _), { 
        if let Token::LowerSymbol(_, sym) = symbol {
            Data::Symbol(sym.into()) 
        }
        else {
            panic!("reflective failure");
        }
    });

    seq!(variable<'a>: &'a Token => Data = variable <= Token::UpperSymbol(_, _), { 
        if let Token::UpperSymbol(_, var) = variable {
            Data::Variable(var.into()) 
        }
        else {
            panic!("reflective failure");
        }
    });

    alt!(main<'a>: &'a Token => Data = number 
                                     | string 
                                     | symbol 
                                     | variable
                                     | data_list
                                     );

    main(input)
});

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_parse{
        ($name:ident: $input:expr => $expected:pat => $x:block) => {
            #[test]
            fn $name() -> Result<(), MatchError> {
                use super::super::tokenizer::tokenize;
                if let Ok(tokens) = tokenize($input) {
                    let mut output = parse(&tokens)?;

                    if let $expected = output {
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

    test_parse!(should_parse_number: "1.0" => Top { lets, expr } => {
        assert_eq!( lets.len(), 0 );
        assert!( matches!( expr, Expr::Data(Data::Number(1.0)) ) );
    });
}
