
use motif::{alt, group, pred, seq, cases};
use motif::MatchError;
use crate::ast::{ Token
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
        Some((i, _)) => Err(MatchError::Fatal(i)),
        None => Ok(top),
    }
}

seq!(parse_top<'a>: &'a Token => Top = lets <= * parse_let, expr <= ? parse_expr, {
    Top { lets, expr }
});

group!(parse_let<'a>: &'a Token => Let = |input| {

    pred!(is_let<'a>: &'a Token => () = 
        |_tok| if let Token::LowerSymbol(_, sym) = _tok {
            sym == "let"
        }
        else {
            false
        }
        
        => { () });

    seq!(main<'a>: &'a Token => Let = is_let
                                    , pattern <= ! parse_pattern
                                    , ! Token::Equal(_)
                                    , expr <= ! parse_expr
                                    , ! Token::Semicolon(_)
                                    , { Let { pattern, expr } });

    main(input)
});

group!(parse_pattern<'a>: &'a Token => Pat = |input| {

    /*
    List(Vec<Pat>, Option<Box<Pat>>),
    */

    seq!(at<'a>: &'a Token => Pat = var <= Token::UpperSymbol(_, _), Token::At(_), pat <= ! parse_pattern, {
        if let Token::UpperSymbol(_, sym) = var {
            Pat::At(sym.into(), Box::new(pat))
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(pat_comma<'a>: &'a Token => Pat = pat <= parse_pattern, Token::Comma(_), { pat });

    seq!(or_bar_pat<'a>: &'a Token => Box<Pat> = Token::OrBar(_), pat <= ! parse_pattern, { Box::new(pat) });

    seq!(pat_list<'a>: &'a Token => Pat = Token::LSquare(_)
                                        , ps <= * pat_comma 
                                        , last <= ? parse_pattern
                                        , rest <= ? or_bar_pat
                                        , ! Token::RSquare(_)
                                        , {

        let mut pats = ps;
        match last {
            Some(pat) => pats.push(pat),
            None => { },
        }
        Pat::List(pats, rest)
    });

    seq!(pat_tuple<'a>: &'a Token => Pat = Token::LCurl(_)
                                         , ps <= * pat_comma 
                                         , last <= ? parse_pattern
                                         , ! Token::RCurl(_)
                                         , {
        let mut pats = ps;
        match last {
            Some(pat) => pats.push(pat),
            None => { },
        }
        Pat::Tuple(pats)
    });

    pred!(wild<'a>: &'a Token => Pat = 
        |_tok| if let Token::LowerSymbol(_, sym) = _tok {
            sym == "_"
        }
        else {
            false
        }
        
        => { Pat::Wild });

    seq!(number<'a>: &'a Token => Pat = n <= Token::Number(_, _), { 
        if let Token::Number(_, number) = n {
            Pat::Number(*number) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(string<'a>: &'a Token => Pat = s <= Token::String(_, _), { 
        if let Token::String(_, string) = s {
            Pat::String(string.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(symbol<'a>: &'a Token => Pat = symbol <= Token::LowerSymbol(_, _), { 
        if let Token::LowerSymbol(_, sym) = symbol {
            Pat::Symbol(sym.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(variable<'a>: &'a Token => Pat = variable <= Token::UpperSymbol(_, _), { 
        if let Token::UpperSymbol(_, var) = variable {
            Pat::Variable(var.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });


    alt!(main<'a>: &'a Token => Pat = at
                                    | wild
                                    | variable
                                    | symbol
                                    | string
                                    | number
                                    | pat_tuple
                                    | pat_list
                                    );
    
    main(input)
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

    seq!(data_comma<'a>: &'a Token => Data = data <= parse_data, Token::Comma(_), { data });

    seq!(data_list<'a>: &'a Token => Data = Token::LSquare(_)
                                          , ds <= * data_comma 
                                          , last <= ? parse_data 
                                          , ! Token::RSquare(_)
                                          , {

        let mut datas = ds;
        match last {
            Some(data) => datas.push(data),
            None => { },
        }
        Data::List(datas)
    });

    seq!(data_tuple<'a>: &'a Token => Data = Token::LCurl(_)
                                          , ds <= * data_comma 
                                          , last <= ? parse_data 
                                          , ! Token::RCurl(_)
                                          , {

        let mut datas = ds;
        match last {
            Some(data) => datas.push(data),
            None => { },
        }
        Data::Tuple(datas)
    });

    seq!(number<'a>: &'a Token => Data = n <= Token::Number(_, _), { 
        if let Token::Number(_, number) = n {
            Data::Number(*number) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(string<'a>: &'a Token => Data = s <= Token::String(_, _), { 
        if let Token::String(_, string) = s {
            Data::String(string.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(symbol<'a>: &'a Token => Data = symbol <= Token::LowerSymbol(_, _), { 
        if let Token::LowerSymbol(_, sym) = symbol {
            Data::Symbol(sym.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(variable<'a>: &'a Token => Data = variable <= Token::UpperSymbol(_, _), { 
        if let Token::UpperSymbol(_, var) = variable {
            Data::Variable(var.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });


    pred!(is_fun<'a>: &'a Token => () = 
        |_tok| if let Token::LowerSymbol(_, sym) = _tok {
            sym == "fun"
        }
        else {
            false
        }
        
        => { () } );

    seq!(pat_comma<'a>: &'a Token => Pat = pat <= parse_pattern, Token::Comma(_), { pat });

    seq!(parse_params<'a>: &'a Token => Vec<Pat> = Token::LParen(_)
                                                 , ps <= * pat_comma 
                                                 , last <= ? parse_pattern
                                                 , ! Token::RParen(_)
                                                 , {

        let mut pats = ps;
        match last {
            Some(pat) => pats.push(pat),
            None => { },
        }
        pats
    });

    seq!(lambda<'a>: &'a Token => Data = is_fun
                                       , params <= ! parse_params
                                       , ! Token::LCurl(_)
                                       , top <= ! parse_top
                                       , ! Token::RCurl(_)
                                       , {

        Data::Lambda(params, Box::new(top))
    });

    alt!(main<'a>: &'a Token => Data = number 
                                     | string 
                                     | lambda
                                     | symbol 
                                     | variable
                                     | data_list
                                     | data_tuple
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
        assert!( matches!( expr, Some(Expr::Data(Data::Number(1.0))) ) );
    });
}
