
use crate::runtime::RuntimeData;


pub fn print_data(data : &RuntimeData) -> String {
    use RuntimeData::*;
    match data { 
        Address(x) => format!("Address: {}", x),
        Function(x) => format!("Function: {}", x),
        Number(x) => format!("Number: {}", x),
        String(x) => format!("String: {}", x),
        Symbol(x) => format!("Symbol: {}", x),
        List(x) => format!("List: {}", x.iter().map(print_data).collect().join(", ")),
        Tuple(x) => format!("Tuple: {}", x.iter().map(print_data).collect().join(", ")),
    }
}
