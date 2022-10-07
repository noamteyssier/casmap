use anyhow::{bail, Result};

use crate::{spacer::Spacer, constant::Constant, utils::reverse_complement};

#[derive(Debug)]
pub struct Construct {
    spacers: Vec<Spacer>,
    constants: Vec<Constant>,
    cid: usize,
}
impl Construct {
    pub fn new(spacers: &[Spacer], constants: &[Constant], cid: usize) -> Result<Self> {
        if spacers.iter().all(|x| x.cid() == cid) {
            Ok( Self {
                spacers: spacers.to_vec(),
                constants: constants.to_vec(),
                cid
            })
        } else {
            bail!("Missing number of constructs found")
        }
    }
    pub fn r1(&self) -> String {
        let s_iter = self.spacers.iter().take(3);
        let c_iter = self.constants.iter().take(3);
        s_iter
            .zip(c_iter)
            .fold(String::new(), |mut seq, (s, c)| {
                seq.push_str(c.sequence());
                seq.push_str(s.sequence());
                seq
            })
    }
    pub fn r2(&self) -> String {
        let s_iter = self.spacers.iter().rev().take(3).rev();
        let c_iter = self.constants.iter().rev().take(3).rev();
        let seq = s_iter
            .zip(c_iter)
            .fold(String::new(), |mut seq, (s, c)| {
                seq.push_str(c.sequence());
                seq.push_str(s.sequence());
                seq
            });
        reverse_complement(&seq)
    }
    pub fn cid(&self) -> usize {
        self.cid
    }
}

