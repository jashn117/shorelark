use std::ops::Index;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub struct Chromosome {
    pub genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes
            .len()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes
            .iter()
    }
    
    pub fn mut_iter(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes
            .iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter
                .into_iter()
                .collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.genes
            .into_iter()
    }
}
