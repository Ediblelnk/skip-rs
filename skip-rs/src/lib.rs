use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

const DEFAULT_P: f64 = 0.5;
type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug, Clone)]
pub struct SkipList<T: Clone> {
    head: Link<T>,
    max_level: usize,
    p: f64,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    forwards: Vec<Option<Link<T>>>,
}

impl<T: Clone + PartialOrd + Debug> SkipList<T> {
    pub fn new(max_level: usize) -> Self {
        let head = Rc::new(RefCell::new(Node {
            value: unsafe { std::mem::zeroed() },
            forwards: vec![None; max_level],
        }));

        SkipList {
            head,
            max_level,
            p: DEFAULT_P,
        }
    }

    pub fn insert(&mut self, value: T) {
        let level = self.determine_level();
        let mut new_node = Rc::new(RefCell::new(Node {
            value: value.clone(),
            forwards: vec![None; level],
        }));

        let mut current = self.head.clone();
        let mut update: Vec<Option<Link<T>>> = vec![None; level];

        for i in (0..self.max_level).rev() {
            loop {
                let next = current.borrow().forwards[i].clone();
                if i < level {
                    update[i] = Some(current.clone());
                }
                match next {
                    Some(node) => {
                        if node.borrow().value < value {
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

impl<T: Clone> SkipList<T> {
    fn determine_level(&self) -> usize {
        let mut rng = SmallRng::from_os_rng();
        let mut level = 1usize;
        while rng.random_bool(self.p) && level < self.max_level {
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
            let mut current = self.head.clone();
            loop {
                let next = current.borrow().forwards[i].clone();
                match next {
                    Some(node) => {
                        write!(f, "{} -> ", node.borrow().value)?;
                        current = node;
                    }
                    None => {
                        write!(f, "//\n")?;
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
