use super::super::grammar::SymbolToken;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LR1Item {
    pub production_index: usize,
    pub position: usize,
    pub lookahead: SymbolToken,
}


