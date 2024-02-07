use crate::lexer::TokenKind;

use self::analysis::ValueType;
use super::*;
use std::collections::HashMap;

type Directory = HashMap<String, ValueType>;
type GenResult = Result<String, CompilationError>;

pub fn gen(ast: Vec<Declaration>, directory: &Directory) -> Result<String, Vec<CompilationError>> {
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

fn lookup<'a>(identifier: &String, directory: &'a Directory, line: u32) -> Result<&'a ValueType, CompilationError> {
    directory
        .get(identifier)
        .ok_or(error(line, format!("Undefined variable: {}", identifier).as_str()))
}

fn gen_declaration(dec: &Declaration, directory: &Directory) -> GenResult {
    match dec {
        Declaration::Variable { name, size } => Ok(gen_variable(name, size)),
        Declaration::Function { name, arguments, body } => gen_function(name, arguments, body, directory),
    }
}

fn gen_variable(name: &Token, size: &u8) -> String {
    format!("{}:: ds {}\n", name.lexeme, size)
}

fn gen_function(name: &Token, _arguments: &Vec<Token>, body: &Vec<Stmt>, directory: &Directory) -> GenResult {
    let mut output = format!("{}::\n", name.lexeme);

    for stmt in body {
        output.push_str(gen_statement(stmt, directory)?.as_str());
    }
    output.push_str("\tret\n");

    Ok(output)
}

fn gen_statement(stmt: &Stmt, directory: &Directory) -> GenResult {
    match stmt {
        Stmt::While { condition, body } => gen_while_loop(condition, body, directory),
        Stmt::Assign { target, value } => gen_assign(target, value, directory),
        Stmt::Expression { expr } => gen_expression(expr, directory),
        Stmt::Halt => Ok(gen_halt()),
    }
}

fn gen_while_loop(condition: &Expr, body: &Vec<Stmt>, directory: &Directory) -> GenResult {
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

fn gen_assign(target: &Expr, value: &Expr, directory: &Directory) -> GenResult {
    match target {
        Expr::Variable { name } => gen_assign_variable(name, value, directory),
        Expr::Indexed { name, index } => gen_assign_indexed(name, index, value, directory),
        Expr::Literal { token, .. } => Err(error(token.line, "Cannot assign to non-variable.")),
        Expr::Binary { operator, .. } => Err(error(operator.line, "Cannot assign to non-variable.")),
    }
}

fn gen_assign_variable(target: &Token, value: &Expr, directory: &Directory) -> GenResult {
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

fn gen_assign_indexed(name: &Token, index: &Box<Expr>, value: &Expr, directory: &Directory) -> GenResult {
    // Load indexed pointer into hl, evaluate new value into a, then set.
    let mut output = gen_indexed(name, index, directory)?;
    output.push_str(gen_evaluate(value, directory)?.as_str());
    output.push_str("\tld [hl], a\n");

    Ok(output)
}

fn gen_expression(expr: &Expr, directory: &Directory) -> GenResult {
    gen_evaluate(expr, directory)
}

fn gen_halt() -> String {
    String::from("\thalt\n")
}

fn gen_evaluate(expr: &Expr, directory: &Directory) -> GenResult {
    match expr {
        Expr::Literal { value, .. } => Ok(gen_evaluate_literal(value)),
        Expr::Variable { name } => gen_evaluate_variable(name, directory),
        Expr::Indexed { name, index } => gen_evaluate_indexed(name, index, directory),
        Expr::Binary { operator, left, right } => gen_evaluate_binary(operator, left, right, directory),
    }
}

fn gen_evaluate_literal(value: &u8) -> String {
    format!("\tld a, {}\n", value)
}

fn gen_evaluate_variable(name: &Token, directory: &Directory) -> GenResult {
    let _ = lookup(&name.lexeme, directory, name.line)?;

    // Is it allowed to load the value of a function here? Maybe for function pointers...
    // (which will require support for 16-bit loads too)

    Ok(format!("\tld a, [{}]\n", name.lexeme))
}

fn gen_evaluate_indexed(name: &Token, index: &Box<Expr>, directory: &Directory) -> GenResult {
    let mut output = gen_indexed(name, index, directory)?;
    output.push_str("\tld a, [hl]\n");
    Ok(output)
}

fn gen_evaluate_binary(operator: &Token, left: &Box<Expr>, right: &Box<Expr>, directory: &Directory) -> GenResult {
    // Evaluate left into a, store in c.
    let mut output = gen_evaluate(left, directory)?;
    output.push_str("\tld c, a\n");

    // Evaluate right into a, store in b, get left back into a.
    output.push_str(gen_evaluate(right, directory)?.as_str());
    output.push_str("\tld b, a\n");
    output.push_str("\tld a, c\n");

    let op = match operator.kind {
        TokenKind::Plus => "\tadd a, b\n",
        TokenKind::Minus => "\tsub a, b\n",
        _ => return Err(error(operator.line, "Unexpected operator in binary expression."))
    };
    output.push_str(op);

    Ok(output)
}

fn gen_indexed(name: &Token, index: &Box<Expr>, directory: &Directory) -> GenResult {
    let def = lookup(&name.lexeme, directory, name.line)?;

    // Cannot index function pointer
    if *def == ValueType::Function {
        return Err(error(name.line, "Cannot index a function identifier"));
    }

    // This assumes that the expression will evaluate to an 8-bit value
    // meaning we can only access up to elements 255 of an array.
    // This will definitely need rethinking!
    let mut output = gen_evaluate(index, directory)?;
    output.push_str("\tld b, 0\n\tld c, a\n");
    output.push_str(format!("\tld hl, {}\n", name.lexeme).as_str());
    output.push_str("\tadd hl, bc\n");

    Ok(output)
}
