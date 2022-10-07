use crate::{spacer::Spacer, constant::Constant};

#[derive(Debug)]
pub struct Construct {
    spacers: Vec<Spacer>,
    constants: Vec<Constant>,
}
impl Construct {
    pub fn new(spacers: &[Spacer], constants: &[Constant]) -> Self {
        Self {
            spacers: spacers.to_vec(),
            constants: constants.to_vec(),
        }
    }
}

