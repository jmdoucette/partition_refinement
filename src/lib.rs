use std::collections::{HashMap, HashSet};

pub struct PartitionRefinement {
    sets: Vec<(HashSet<u32>, Option<usize>)>,
    val_to_set: HashMap<u32, usize>
}

impl PartitionRefinement {

    pub fn new(elements: Vec<u32>) -> Self {
        let mut val_to_set = HashMap::new();
        for element in elements.iter() {
            val_to_set.insert(*element, 0);
        }

        let sets = vec![(HashSet::from_iter(elements.into_iter()), None)];


        Self {
            sets,
            val_to_set,
        }
    }
}
