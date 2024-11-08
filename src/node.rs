use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Node {
    pub char: Option<char>,
    pub freq: usize,
    pub left: Option<Rc<RefCell<Node>>>,
    // shared_ptr in c++ Rc with inner change ablity
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(char: Option<char>, freq: usize) -> Self {
        Node {
            char,
            freq,
            left: None,
            right: None,
        }
    }
}

