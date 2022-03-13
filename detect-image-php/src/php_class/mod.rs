use std::fmt::Display;

use phper::{
    classes::{DynamicClass, ClassEntry},
    Throwable,
};

#[macro_use]
pub mod macros;
pub mod result;
pub mod scanner;
pub mod rqrr;



#[derive(Debug)]
pub struct DetectResultError(String);

impl Display for DetectResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"detect error: {}",self.0)
    }
}
impl std::error::Error for DetectResultError {}

const EXCEPTION_CLASS_NAME: &'static str = class_name!("Exception");
#[derive(Debug, thiserror::Error, Throwable)]
#[throwable_class(EXCEPTION_CLASS_NAME)]
pub enum DetectError {
    #[error(transparent)]
    #[throwable(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    Detect(DetectResultError)
}




pub fn make_exception_class() -> DynamicClass<()> {
    let mut exception_class = DynamicClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}