use super::*;

pub fn gen(ast: Vec<Declaration>) -> Result<String, Vec<CompilationError>> {
    let mut output = String::new();
    // let mut errors: Vec<CompilationError> = Vec::new();

    // Define all variables in memory first
    output.push_str(String::from("SECTION \"Variables\", WRAM\n").as_str());
    for dec in ast.iter().filter(is_variable) {
        output.push_str(gen_declaration(dec).as_str())
    }

    // Now output all functions
    output.push_str(String::from("SECTION \"Functions\", ROM0\n").as_str());
    for dec in ast.iter().filter(is_function) {
        output.push_str(gen_declaration(dec).as_str())
    }

    Ok(output)
}

fn is_variable(dec: &&Declaration) -> bool {
    match dec {
        Declaration::Variable {..} => true,
        _ => false,
    }
}

fn is_function(dec: &&Declaration) -> bool {
    match dec {
        Declaration::Function {..} => true,
        _ => false,
    }
}

fn _error(line: u32, msg: &str) -> CompilationError {
    CompilationError {
        line,
        msg: msg.to_string(),
    }
}

fn gen_declaration(dec: &Declaration) -> String {
    match dec {
        Declaration::Variable { name } => gen_variable(name),
        Declaration::Function { name, arguments, body} => gen_function(name, arguments, body)
    }
}

fn gen_variable(name: &Token) -> String {
    format!("{}:: db\n", name.lexeme)
}

fn gen_function(name: &Token, _arguments: &Vec<Token>, _body: &Vec<Stmt>) -> String {
    let output = format!("{}:", name.lexeme);

    output
}

fn _gen_halt() -> String {
    String::from("halt\n")
}

//fn gen_function
