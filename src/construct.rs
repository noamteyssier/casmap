use crate::{spacer::Spacer, constant::Constant};

#[derive(Debug)]
pub struct Construct<'a> {
    spacers: Vec<&'a Spacer>,
    constants: Vec<&'a Constant>,
}
