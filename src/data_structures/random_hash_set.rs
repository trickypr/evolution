use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

use rand::Rng;
#[derive(Clone)]
pub struct RandomHashSet<T: Eq + Hash + Clone> {
    set: HashSet<T>,
    data: Vec<T>,
}

pub fn new<T: Eq + Hash + Clone>() -> RandomHashSet<T> {
    RandomHashSet {
        set: HashSet::new(),
        data: Vec::new(),
    }
}

impl<T: Eq + Hash + Clone> RandomHashSet<T> {
    pub fn contains(&self, object: &T) -> bool {
        self.set.contains(object)
    }

    pub fn random_element(&self) -> Option<&T> {
        let mut rand = rand::thread_rng();

        if self.data.len() > 0 {
            return Some(self.data.get(rand.gen_range(0, self.data.len())).unwrap());
        }

        None
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn add(&mut self, object: &T) {
        if !self.contains(object) {
            self.set.insert(object.clone());
            self.data.push(object.clone());
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.set.clear();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < 0 || index >= self.data.len() {
            return None;
        }

        Some(self.data.get(index).unwrap())
    }

    pub fn remove(&mut self, index: usize) {
        if index < 0 || index >= self.data.len() {
            return;
        }

        self.set.remove(self.data.get(index).unwrap());
        self.data.remove(index);
    }

    pub fn remove_object(&mut self, object: &T) {
        self.set.remove(object);
        self.data
            .remove(self.data.iter().position(|x| x == object).unwrap());
    }

    pub fn get_data(&self) -> &Vec<T> {
        return &self.data;
    }
}

////////////////////////////////
// Tests for RandomHashSet
#[cfg(test)]
mod random_hash_set_tests {
    use crate::data_structures::random_hash_set::new;

    #[test]
    fn contains() {
        let mut set = new();

        assert_eq!(set.contains(&5), false);
        set.add(&5);
        assert_eq!(set.contains(&5), true);
    }

    #[test]
    fn size() {
        let mut set = new();

        assert_eq!(set.size(), 0);
        set.add(&true);
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn add() {
        let mut set = new();

        set.add(&5);
        set.add(&6);
        assert_eq!(set.size(), 2);
        set.add(&5);
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn clear() {
        let mut set = new();

        set.add(&5);
        set.add(&6);
        assert_eq!(set.size(), 2);
        set.clear();
        assert_eq!(set.size(), 0);
    }
}
