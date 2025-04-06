use std::fmt::Display;

use rand::{rngs::SmallRng, Rng, SeedableRng};

const DEFAULT_P: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct SkipList<T: Clone> {
    head: Vec<Option<Node<T>>>,
    max_level: usize,
    p: f64,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    forward: Vec<Option<Node<T>>>,
}

impl<T: Clone> SkipList<T> {
    pub fn new(max_level: usize) -> Self {
        SkipList {
            head: vec![None; max_level],
            max_level,
            p: DEFAULT_P,
        }
    }

    pub fn insert(&mut self, value: T) {
        let level = Self::determine_level(0.5);
        let node = Node {
            value,
            forward: vec![None; level],
        };

        // Insert logic here
    }
}

impl<T: Clone> SkipList<T> {
    fn determine_level(p: f64) -> usize {
        let mut rng = SmallRng::from_os_rng();

        let mut level = 1usize;
        while rng.random_bool(p) {
            level += 1;
        }

        level
    }
}

impl<T> Display for SkipList<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.max_level).rev() {
            write!(f, "L{}: ", i)?;
            let mut node = &self.head[i];
            while node.as_ref().is_some() {
                write!(f, "{} --> ", node.as_ref().unwrap().value)?;
                node = &node.as_ref().unwrap().forward[i];
            }
            write!(f, "None\n")?;
        }
        Ok(())
    }
}
