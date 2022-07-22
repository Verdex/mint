
use crate::ast::{Pat, Data};
use super::context::Context;
use super::error::RuntimeError;



pub fn pattern_match( pattern : &Pat, data : &Data, context : &mut Context ) -> Result<(), RuntimeError> {

}