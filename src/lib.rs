use std::collections::{HashMap, HashSet};

pub struct PartitionRefinement {
    sets: Vec<(HashSet<u32>, Option<usize>)>,
    val_to_set_index: HashMap<u32, usize>
}

impl PartitionRefinement {

    pub fn new(elements: Vec<u32>) -> Self {
        let mut val_to_set_index = HashMap::new();
        for element in elements.iter() {
            val_to_set_index.insert(*element, 0);
        }

        let sets = vec![(HashSet::from_iter(elements.into_iter()), None)];

        Self {
            sets,
            val_to_set_index,
        }
    }

    pub fn refine(&mut self, split_elements: Vec<u32>) {
        for split_element in split_elements {
            let &index = self.val_to_set_index.get(&split_element).unwrap();
            if self.sets[index].1.is_none() {
                if !self.sets[index].0.remove(&split_element) {
                    // would violate bijection principle
                    unreachable!();
                }
                let mut new_set = HashSet::new();
                new_set.insert(split_element);
                self.sets.push((new_set, None));

                self.sets[index].1 = Some(self.sets.len()-1);
                self.val_to_set_index.insert(split_element, self.sets.len()-1);

            } else {
                if !self.sets[index].0.remove(&split_element) {
                    // would violate bijection principle
                    unreachable!();
                }
                let new_index = self.sets[index].1.unwrap();
                self.sets[new_index].0.insert(split_element);
                self.val_to_set_index.insert(split_element, new_index);
            }
        }
        
    }
}
