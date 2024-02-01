use self::analysis::ValueType;
use super::*;
use std::collections::HashMap;

pub fn gen(
    ast: Vec<Declaration>,
    directory: &HashMap<String, ValueType>,
) -> Result<String, Vec<CompilationError>> {
    let mut output = String::new();
    let mut errors: Vec<CompilationError> = Vec::new();

    // Define all variables in memory first
    output.push_str(String::from("SECTION \"Variables\", WRAM0\n").as_str());
    for dec in ast.iter().filter(is_variable) {
        match gen_declaration(dec, directory) {
            Ok(asm) => output.push_str(asm.as_str()),
            Err(err) => errors.push(err),
        }
    }

    // Now output all functions
    output.push_str(String::from("SECTION \"Functions\", ROM0\n").as_str());
    for dec in ast.iter().filter(is_function) {
        match gen_declaration(dec, directory) {
            Ok(asm) => output.push_str(asm.as_str()),
            Err(err) => errors.push(err),
        }
    }

    if errors.len() == 0 {
        Ok(output)
    } else {
        Err(errors)
    }
}

static mut UID: u32 = 0;

fn get_uid() -> u32 {
    // There's certainly a way to do this without using unsafe, but this seems OK for now.
    unsafe {
        UID = UID + 1;
        UID
    }
}

fn is_variable(dec: &&Declaration) -> bool {
    match dec {
        Declaration::Variable { .. } => true,
        _ => false,
    }
}

fn is_function(dec: &&Declaration) -> bool {
    match dec {
        Declaration::Function { .. } => true,
        _ => false,
    }
}

fn error(line: u32, msg: &str) -> CompilationError {
    CompilationError {
        line,
        msg: msg.to_string(),
    }
}

fn lookup<'a>(
    identifier: &String,
    directory: &'a HashMap<String, ValueType>,
    line: u32,
) -> Result<&'a ValueType, CompilationError> {
    directory.get(identifier).ok_or(error(
        line,
        format!("Undefined variable: {}", identifier).as_str(),
    ))
}

fn gen_declaration(
    dec: &Declaration,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    match dec {
        Declaration::Variable { name } => Ok(gen_variable(name)),
        Declaration::Function {
            name,
            arguments,
            body,
        } => gen_function(name, arguments, body, directory),
    }
}

fn gen_variable(name: &Token) -> String {
    format!("{}:: db\n", name.lexeme)
}

fn gen_function(
    name: &Token,
    _arguments: &Vec<Token>,
    body: &Vec<Stmt>,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    let mut output = format!("{}::\n", name.lexeme);

    for stmt in body {
        output.push_str(gen_statement(stmt, directory)?.as_str());
    }
    output.push_str("\tret\n");

    Ok(output)
}

fn gen_statement(
    stmt: &Stmt,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    match stmt {
        Stmt::While { condition, body } => gen_while_loop(condition, body, directory),
        Stmt::Assign { target, value } => gen_assign(target, value, directory),
        Stmt::Expression { expr } => gen_expression(expr, directory),
        Stmt::Halt => Ok(gen_halt()),
    }
}

fn gen_while_loop(
    condition: &Expr,
    body: &Vec<Stmt>,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    let uid = get_uid();
    let mut output = format!(".startWhile_{}\n", uid);

    // Check the loop condition
    output.push_str(gen_evaluate(condition, directory)?.as_str());
    output.push_str("\tor a\n");
    output.push_str(format!("\tjr z, .endWhile_{}\n", uid).as_str());

    for stmt in body {
        output.push_str(gen_statement(stmt, directory)?.as_str());
    }

    output.push_str(format!("\tjr .startWhile_{}\n", uid).as_str());
    output.push_str(format!(".endWhile_{}\n", uid).as_str());

    Ok(output)
}

fn gen_assign(
    target: &Token,
    value: &Expr,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    let def = lookup(&target.lexeme, directory, target.line)?;

    // Can't assign to functions...
    if *def == ValueType::Function {
        return Err(error(target.line, "Cannot assign to function"));
    }

    // Evaluate expression into a, then store into memory
    let mut output = gen_evaluate(value, directory)?;
    output.push_str(format!("\tld [{}], a\n", target.lexeme).as_str());

    Ok(output)
}

fn gen_expression(
    expr: &Expr,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    gen_evaluate(expr, directory)
}

fn gen_halt() -> String {
    String::from("\thalt\n")
}

fn gen_evaluate(
    expr: &Expr,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    match expr {
        Expr::Literal { value } => Ok(gen_evaluate_literal(value)),
        Expr::Variable { name } => gen_evaluate_variable(name, directory),
    }
}

fn gen_evaluate_literal(value: &u8) -> String {
    format!("\tld a, {}\n", value)
}

fn gen_evaluate_variable(
    name: &Token,
    directory: &HashMap<String, ValueType>,
) -> Result<String, CompilationError> {
    let _ = lookup(&name.lexeme, directory, name.line)?;

    // Is it allowed to load the value of a function here? Maybe for function pointers...
    // (which will require support for 16-bit loads too)

    Ok(format!("\tld a, [{}]\n", name.lexeme))
}
