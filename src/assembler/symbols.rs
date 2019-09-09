use std::collections::HashMap;

#[derive(Debug)]
pub enum SymbolType {
    Label,
    Integer,
    String,
}

#[derive(Debug)]
pub struct SymbolInfo {
    offset: u32,
    symbol_type: SymbolType,
}

impl SymbolInfo {
    pub fn new(offset: u32, t: SymbolType) -> Self {
        SymbolInfo {
            offset,
            symbol_type: t,
        }
    }
}

pub type SymbolTable = HashMap<String, SymbolInfo>;
