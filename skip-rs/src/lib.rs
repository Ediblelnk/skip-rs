use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

const DEFAULT_P: f64 = 0.5;
type Link<K, V> = Rc<RefCell<Node<K, V>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum SkipListError {
    KeyNotFound,
}

impl std::fmt::Display for SkipListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipListError::KeyNotFound => write!(f, "Key not found"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkipList<K: Clone, V: Sized> {
    head: Link<K, V>,
    length: usize,
    level_fixed: bool,
    p: f64,
}

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    key: K,
    value: V,
    forwards: Vec<Option<Link<K, V>>>,
    distance: Vec<usize>,
}

impl<K: Default, V: Default> Default for Node<K, V> {
    fn default() -> Self {
        Node {
            key: K::default(),
            value: V::default(),
            forwards: vec![None],
            distance: vec![1],
        }
    }
}

impl<K: Clone + Ord + Debug + Default, V: Default> SkipList<K, V> {
    pub fn new() -> Self {
        SkipList {
            head: Rc::new(RefCell::new(Node::default())),
            length: 0,
            level_fixed: false,
            p: DEFAULT_P,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.length += 1;
        let level = self.determine_level();
        let mut current = self.head.clone();
        let mut update: Vec<(Link<K, V>, usize)> = vec![(self.head.clone(), 0); level];
        let mut position: usize = 0;

        for i in (0..self.max_level()).rev() {
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) if node.borrow().key < key => {
                        position += current.borrow().distance[i];
                        current = node;
                    }
                    _ => break,
                }
            }

            if i < level {
                update[i].0 = current.clone();
                update[i].1 = position;
            } else {
                current.borrow_mut().distance[i] += 1;
            }
        }

        position += 1;

        let new_node = Rc::new(RefCell::new(Node {
            key: key.clone(),
            value,
            forwards: vec![None; level],
            distance: vec![1; level],
        }));

        for i in 0..level {
            new_node.borrow_mut().forwards[i] = update[i].0.borrow().forwards[i].clone();
            update[i].0.borrow_mut().forwards[i] = Some(new_node.clone());

            let d = position - update[i].1;
            new_node.borrow_mut().distance[i] = update[i].0.borrow().distance[i] - d + 1;
            update[i].0.borrow_mut().distance[i] = d;
        }
    }

    pub fn edit<F>(&mut self, key: K, mut modify: F) -> Result<(), SkipListError>
    where
        F: FnMut(&mut V),
    {
        let mut current = self.head.clone();
        for i in (0..self.max_level()).rev() {
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) if node.borrow().key <= key => {
                        current = node;
                    }
                    _ => break,
                }
            }
        }

        if current.borrow().key == key {
            modify(&mut current.borrow_mut().value);
            Ok(())
        } else {
            Err(SkipListError::KeyNotFound)
        }
    }

    pub fn remove(&mut self, key: K) -> Result<(), SkipListError> {
        if self.length == 0 {
            return Err(SkipListError::KeyNotFound);
        }

        self.length -= 1;
        let mut current = self.head.clone();
        let mut update: Vec<(Link<K, V>, usize)> = vec![(self.head.clone(), 0); self.max_level()];
        let mut position: usize = 0;

        for i in (0..self.max_level()).rev() {
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) if node.borrow().key < key => {
                        position += current.borrow().distance[i];
                        current = node;
                    }
                    _ => break,
                }
            }
            update[i].0 = current.clone();
            update[i].1 = position;
        }

        if current.borrow().key == key {
            // for i in 0..self.max_level() {

            // }
            Ok(())
        } else {
            Err(SkipListError::KeyNotFound)
        }
    }
}

impl<K: Clone, V: Sized> SkipList<K, V> {
    fn determine_level(&mut self) -> usize {
        let mut rng = SmallRng::from_os_rng();
        let mut level = 1;
        while level <= self.max_level() && rng.random_bool(self.p) {
            level += 1;
        }
        if level > self.max_level() && !self.level_fixed {
            self.head.borrow_mut().forwards.push(None);
            self.head.borrow_mut().distance.push(self.length);
        }
        level
    }

    pub fn fix_level(&mut self) {
        self.level_fixed = true;
    }

    pub fn unfix_level(&mut self) {
        self.level_fixed = false;
    }
}

impl<K: Clone, V> SkipList<K, V> {
    pub fn max_level(&self) -> usize {
        self.head.borrow().forwards.len()
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn p(&self) -> f64 {
        self.p
    }
    pub fn set_p(&mut self, p: f64) {
        let p = p.clamp(0., 1.);
        self.p = p;
    }
}

impl<K, V> Display for SkipList<K, V>
where
    K: Display + Clone,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.max_level()).rev() {
            write!(f, "L{}|", i)?;
            let mut current = self.head.clone();
            for _ in 0..(current.borrow().distance[i] - 1) {
                write!(f, "----------")?;
            }
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) => {
                        write!(f, "-|{:>3}:{:>3}|", node.borrow().key, node.borrow().value)?;
                        for _ in 0..(node.borrow().distance[i] - 1) {
                            write!(f, "----------")?;
                        }
                        current = node;
                    }
                    None => break,
                }
            }
            write!(f, "-|\n")?;
        }
        Ok(())
    }
}
