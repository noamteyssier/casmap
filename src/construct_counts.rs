use hashbrown::HashMap;

use crate::construct_results::ConstructResults;

#[derive(Debug)]
pub struct ConstructCounts {
    map: HashMap<usize, usize>,
}
impl ConstructCounts {
    pub fn new(n_constructs: usize) -> Self {
        let map = (0..n_constructs).fold(HashMap::new(), |mut map, x| {
            map.insert(x, 0);
            map
        });
        Self { map }
    }
    pub fn count(&mut self, results: &ConstructResults) {
        match results.cid() {
            Some(cid) => {
                *self.map.get_mut(&cid).unwrap() += 1;
            }
            None => {}
        }
    }
    pub fn pprint(&self) {
        println!("{}\t{}", "ConstructID", "Counts");
        self.map.iter().for_each(|(k, v)| {
            println!("{}\t{}", k, v);
        });
    }
}
