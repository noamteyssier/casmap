pub struct KmerIter<'a> {
    sequence: &'a str,
    kmer: usize,
    position: usize,
    len: usize,
}
impl<'a> KmerIter<'a> {
    pub fn new(sequence: &'a str, kmer: usize) -> Self {
        Self {
            sequence,
            kmer,
            position: 0,
            len: sequence.len(),
        }
    }
}
impl<'a> Iterator for KmerIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.kmer + self.position <= self.len {
            let substr = &self.sequence[self.position..self.position + self.kmer];
            self.position += 1;
            Some(substr)
        } else {
            None
        }
    }
}
