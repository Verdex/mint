

use crate::ast::*;

use super::data::Context;

use super::error::RuntimeError;
use super::pattern_matcher;


pub fn eval( input : Top, context : &mut Context ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    Ok(None)
}
