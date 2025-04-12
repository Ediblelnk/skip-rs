use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

const DEFAULT_P: f64 = 0.5;
type Link<K, V> = Rc<RefCell<Node<K, V>>>;

#[derive(Debug, Clone)]
pub struct SkipList<K: Clone, V: Sized> {
    head: Link<K, V>,
    max_level: usize,
    p: f64,
}

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    key: K,
    value: V,
    forwards: Vec<Option<Link<K, V>>>,
    distance: Vec<usize>,
}

impl<K: Clone + PartialOrd + Debug, V: Sized> SkipList<K, V> {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node {
            key: unsafe { std::mem::zeroed() },
            value: unsafe { std::mem::zeroed() },
            forwards: vec![None; 2],
            distance: vec![0; 2],
        }));

        SkipList {
            head,
            max_level: 2,
            p: DEFAULT_P,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let level = self.determine_level();
        let new_node = Rc::new(RefCell::new(Node {
            key: key.clone(),
            value,
            forwards: vec![None; level],
            distance: vec![1; level],
        }));

        let mut current = self.head.clone();
        let mut update: Vec<Option<Link<K, V>>> = vec![None; level];

        for i in (0..self.max_level).rev() {
            loop {
                let next = current.borrow().forwards[i].clone();
                if i < level {
                    update[i] = Some(current.clone());
                }
                match next {
                    Some(node) => {
                        if node.borrow().key < key {
                            current = node;
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }

        for i in 0..level {
            match update[i].clone() {
                Some(prev) => {
                    new_node.borrow_mut().forwards[i] = prev.borrow().forwards[i].clone();
                    prev.borrow_mut().forwards[i] = Some(new_node.clone());
                }
                None => {
                    new_node.borrow_mut().forwards[i] = self.head.borrow().forwards[i].clone();
                    self.head.borrow_mut().forwards[i] = Some(new_node.clone());
                }
            }
        }
    }
}

impl<K: Clone, V: Sized> SkipList<K, V> {
    fn determine_level(&mut self) -> usize {
        let mut rng = SmallRng::from_os_rng();
        let mut level = 1usize;
        while rng.random_bool(self.p) {
            level += 1;
            if level > self.max_level {
                self.increase_level();
                break;
            }
        }
        level
    }

    fn increase_level(&mut self) {
        self.max_level += 1;
        self.head.borrow_mut().forwards.push(None);
        self.head.borrow_mut().distance.push(0);
    }
}

impl<K: Clone, V> SkipList<K, V> {
    pub fn max_level(&self) -> usize {
        self.max_level
    }
    pub fn p(&self) -> f64 {
        self.p
    }
    pub fn set_p(&mut self, p: f64) {
        self.p = p;
    }
}

impl<K, V> Display for SkipList<K, V>
where
    K: Display + Clone,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.max_level).rev() {
            write!(f, "L{}: ", i)?;
            let mut current = self.head.clone();
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) => {
                        write!(
                            f,
                            "[{}]:{{{}}} -|{}|-> ",
                            node.borrow().key,
                            node.borrow().value,
                            node.borrow().distance[i]
                        )?;
                        current = node;
                    }
                    None => {
                        write!(f, "||\n")?;
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
