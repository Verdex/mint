
use super::data::Context;
use super::error::RuntimeError;

pub fn print_data(data : &RuntimeData) -> String {
    use RuntimeData;
    match data { 
        Address(x) => format!("Address: {}", x),
        Function(x) => format!("Function: {}", x),
        Number(x) => format!("Number: {}", x),
        String(x) => format!("String: {}", x),
        Symbol(x) => format!("Symbol: {}", x),
        List(x) => format!("List "),
        Tuple(x),
    }
}
/*pub fn print_data(data : &Lit, context : &Context) -> Result<String, RuntimeError> {
    match data {
        Lit::Number(n) => Ok(n.to_string()),
        Lit::String(s) => Ok(format!("\"{}\"", s)),
        Lit::Symbol(s) => Ok(s.into()),
        Lit::Variable(var) => {
            let data = context.lookup(var)?;
            print_data(&data, context)
        },
        Lit::List(datas) => {
            let inner = datas.iter()
                             .map(|data| print_data(data, context))
                             .collect::<Result<Vec<String>, RuntimeError>>()?
                             .join(", ");
            Ok(format!("[{}]", inner))
        },
        Lit::Tuple(datas) => {
            let inner = datas.iter()
                             .map(|data| print_data(data, context))
                             .collect::<Result<Vec<String>, RuntimeError>>()?
                             .join(", ");
            Ok(format!("{{{}}}", inner))
        },
        Lit::Lambda(_) => {
            Ok("FUNCTION".into())
        },
    }
}*/


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_err_for_unbound_variable_in_middle_of_list() {
        let input = Lit::List(vec![ Lit::Number(1.0), Lit::Variable("X".into()), Lit::Number(1.0) ]);
        let context = Context::new();
        let output = print_data(&input, &context);

        assert!( matches!( output, Err(RuntimeError::VariableNotFound(_) ) ) );
    }
}