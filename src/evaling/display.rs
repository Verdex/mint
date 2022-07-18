
use crate::ast::Data;
use super::context::Context;
use super::error::RuntimeError;

pub fn print_data(data : &Data, context : &Context) -> Result<String, RuntimeError> {
    match data {
        Data::Number(n) => Ok(n.to_string()),
        Data::String(s) => Ok(format!("\"{}\"", s)),
        Data::Symbol(s) => Ok(s.into()),
        Data::Variable(var) => {
            let data = context.lookup(var)?;
            print_data(&data, context)
        },
        Data::List(datas) => {
            let inner = datas.iter()
                             .map(|data| print_data(data, context))
                             .collect::<Result<Vec<String>, RuntimeError>>()?
                             .join(", ");
            Ok(format!("[{}]", inner))
        },
        Data::Tuple(datas) => {
            let inner = datas.iter()
                             .map(|data| print_data(data, context))
                             .collect::<Result<Vec<String>, RuntimeError>>()?
                             .join(", ");
            Ok(format!("{{{}}}", inner))
        },
        _ => Ok("err".into()),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_err_for_unbound_variable_in_middle_of_list() {
        let input = Data::List(vec![ Data::Number(1.0), Data::Variable("X".into()), Data::Number(1.0) ]);
        let context = Context::new();
        let output = print_data(&input, &context);

        assert!( matches!( output, Err(RuntimeError::VariableNotFound(_) ) ) );
    }
}