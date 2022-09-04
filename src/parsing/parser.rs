
use motif::{alt, group, pred, seq, cases};
use motif::MatchError;
use crate::ast::{ Token
                , Top 
                , Let
                , Expr
                , Lit 
                , Pat
                , Lambda
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
    seq!(lit<'a>: &'a Token => Expr = lit <= parse_literal, { Expr::Literal(lit) });

    seq!(expr_comma<'a>: &'a Token => Expr = expr <= parse_expr, Token::Comma(_), { expr });

    seq!(param_list<'a>: &'a Token => Vec<Expr> = Token::LParen(_)
                                                , es <= * expr_comma 
                                                , last <= ? parse_expr
                                                , ! Token::RParen(_)
                                                , {

        let mut exprs = es;
        match last {
            Some(expr) => exprs.push(expr),
            None => { },
        }
        exprs
    });

    alt!(main<'a>: &'a Token => Expr = lit);

    seq!(call<'a>: &'a Token => Expr = m <= main, calls <= * param_list, {
        if calls.len() == 0 {
            m
        }
        else {
            let mut r = None;
            for call in calls {
                if let Some(prev) = r {
                    r = Some(Expr::Call(Box::new(prev), call));
                }
                else {
                    r = Some(Expr::Call(Box::new(m.clone()), call));
                }
            }
            if let Some(ret) = r {
                ret
            }
            else {
                panic!("return value isn't there");
            }
        }

    });

    call(input)
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

group!(parse_literal<'a>: &'a Token => Lit = |input| {

    seq!(lit_comma<'a>: &'a Token => Lit = lit <= parse_literal, Token::Comma(_), { lit });

    seq!(lit_list<'a>: &'a Token => Lit = Token::LSquare(_)
                                        , ls <= * lit_comma 
                                        , last <= ? parse_literal
                                        , ! Token::RSquare(_)
                                        , {

        let mut lits = ls;
        match last {
            Some(lit) => lits.push(lit),
            None => { },
        }
        Lit::List(lits)
    });

    seq!(lit_tuple<'a>: &'a Token => Lit = Token::LCurl(_)
                                         , ls <= * lit_comma 
                                         , last <= ? parse_literal
                                         , ! Token::RCurl(_)
                                         , {

        let mut lits = ls;
        match last {
            Some(lit) => lits.push(lit),
            None => { },
        }
        Lit::Tuple(lits)
    });

    seq!(number<'a>: &'a Token => Lit = n <= Token::Number(_, _), { 
        if let Token::Number(_, number) = n {
            Lit::Number(*number) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(string<'a>: &'a Token => Lit = s <= Token::String(_, _), { 
        if let Token::String(_, string) = s {
            Lit::String(string.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(symbol<'a>: &'a Token => Lit = symbol <= Token::LowerSymbol(_, _), { 
        if let Token::LowerSymbol(_, sym) = symbol {
            Lit::Symbol(sym.into()) 
        }
        else {
            panic!("reflexive fail");
        }
    });

    seq!(variable<'a>: &'a Token => Lit = variable <= Token::UpperSymbol(_, _), { 
        if let Token::UpperSymbol(_, var) = variable {
            Lit::Variable(var.into()) 
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

    seq!(lambda<'a>: &'a Token => Lit = is_fun
                                       , params <= ! parse_params
                                       , ! Token::LCurl(_)
                                       , body <= ! parse_top
                                       , ! Token::RCurl(_)
                                       , {

        Lit::Lambda(Lambda { params, body: Box::new(body) } )
    });

    alt!(main<'a>: &'a Token => Lit = number 
                                    | string 
                                    | lambda
                                    | symbol 
                                    | variable
                                    | lit_list
                                    | lit_tuple
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
        assert!( matches!( expr, Some(Expr::Literal(Lit::Number(1.0))) ) );
    });
}
