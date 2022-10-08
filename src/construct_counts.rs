use crate::construct_results::ConstructResults;
use anyhow::Result;
use hashbrown::HashMap;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct ConstructCounts {
    map: HashMap<usize, usize>,
    n_mapped: usize,
    n_unmapped: usize,
    total: usize,
}
impl ConstructCounts {
    pub fn new(n_constructs: usize) -> Self {
        let map = (0..n_constructs).fold(HashMap::new(), |mut map, x| {
            map.insert(x, 0);
            map
        });
        Self {
            map,
            n_mapped: 0,
            n_unmapped: 0,
            total: 0,
        }
    }
    pub fn count(&mut self, results: &ConstructResults) {
        self.total += 1;
        match results.cid() {
            Some(cid) => {
                self.n_mapped += 1;
                *self.map.get_mut(&cid).unwrap() += 1;
            }
            None => {
                self.n_unmapped += 1;
            }
        }
    }
    pub fn pprint(&self, output: &str) -> Result<()> {
        eprintln!("Writing Results to: {}", output);
        let mut f = File::create(output)?;
        writeln!(f, "{}\t{}", "ConstructID", "Counts")?;
        for (k, v) in self.map.iter() {
            writeln!(f, "{}\t{}", k, v)?;
        }
        Ok(())
    }
    pub fn statistics(&self) {
        eprintln!("Mapped Reads    : {}", self.n_mapped);
        eprintln!("Total Reads     : {}", self.total);
        eprintln!(
            "Fraction Mapped : {:.4}",
            self.n_mapped as f64 / self.total as f64
        );
    }
}
