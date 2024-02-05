use super::*;
use std::collections::HashMap;

// When I introduce a type system, this might need to live elsewhere...
#[derive(PartialEq, Eq)]
pub enum ValueType {
    UnsignedByte,
    Function,
}

pub fn generate_directory(ast: &Vec<Declaration>) -> HashMap<String, ValueType> {
    let mut directory: HashMap<String, ValueType> = HashMap::new();

    for dec in ast.iter() {
        match dec {
            Declaration::Function { name, .. } => {
                directory.insert(name.lexeme.clone(), ValueType::Function);
            }
            Declaration::Variable { name, .. } => {
                directory.insert(name.lexeme.clone(), ValueType::UnsignedByte);
            }
        }
    }

    directory
}
