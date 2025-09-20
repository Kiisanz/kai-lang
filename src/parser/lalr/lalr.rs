use std::collections::{HashMap, HashSet};
use crate::parser::{grammar::Symbol, lalr::LR1Item};

#[derive(Debug, Clone)]
pub struct LALRState {
    pub items: HashSet<LR1Item>,
    pub transitions: HashMap<Symbol, usize>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
    Error,
}