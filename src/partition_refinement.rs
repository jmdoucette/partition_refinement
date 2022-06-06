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

    pub fn same_set(&self, element_1: u32, element_2: u32) -> bool {
        match (self.val_to_set_index.get(&element_1), self.val_to_set_index.get(&element_2)) {
            (Some(index_1), Some(index_2)) => index_1 == index_2,
            _ => panic!("missing elements")
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_refine() {
        let mut pr = PartitionRefinement::new(vec![0,1,2,3,4,5,6,7]);
        pr.refine(vec![0,1,2]);
        assert!(pr.same_set(0,2));
        assert!(pr.same_set(3,7));
        assert!(!pr.same_set(0,5));
        assert!(!pr.same_set(2,4));

        pr.refine(vec![0,5,6,7]);
        assert!(pr.same_set(1,2));
        assert!(pr.same_set(5,7));
        assert!(!pr.same_set(0,2));
        assert!(!pr.same_set(3,7));
        assert!(!pr.same_set(0,5));
        assert!(!pr.same_set(2,4));

        pr.refine(vec![1,2]);

        assert!(pr.same_set(1,2));
        assert!(pr.same_set(5,6));
        assert!(!pr.same_set(1,3));
        assert!(!pr.same_set(1,4));
        assert!(!pr.same_set(1,5));
        assert!(!pr.same_set(1,6));
        assert!(!pr.same_set(7,2));
    }
}