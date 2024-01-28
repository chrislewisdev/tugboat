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

fn _error(line: u32, msg: &str) -> CompilationError {
    CompilationError {
        line,
        msg: msg.to_string(),
    }
}

fn gen_declaration(dec: &Declaration) -> String {
    match dec {
        Declaration::Variable { name } => gen_variable(name),
        Declaration::Function {
            name,
            arguments,
            body,
        } => gen_function(name, arguments, body),
    }
}

fn gen_variable(name: &Token) -> String {
    format!("{}:: db\n", name.lexeme)
}

fn gen_function(name: &Token, _arguments: &Vec<Token>, body: &Vec<Stmt>) -> String {
    let mut output = format!("{}:\n", name.lexeme);

    for stmt in body {
        output.push_str(gen_statement(stmt).as_str());
    }

    output.push_str("\tret\n");

    output
}

fn gen_statement(stmt: &Stmt) -> String {
    match stmt {
        Stmt::While { condition, body } => gen_while_loop(condition, body),
        Stmt::Assign { target, value } => gen_assign(target, value),
        Stmt::Expression { expr } => gen_expression(expr),
        Stmt::Halt => gen_halt(),
    }
}

fn gen_while_loop(condition: &Expr, body: &Vec<Stmt>) -> String {
    let uid = get_uid();
    let mut output = format!(".startWhile_{}\n", uid);

    // Check the loop condition
    output.push_str(gen_evaluate(condition).as_str());
    output.push_str("\tor a\n");
    output.push_str(format!("\tjr nz, .endWhile_{}\n", uid).as_str());

    for stmt in body {
        output.push_str(gen_statement(stmt).as_str());
    }

    output.push_str(format!(".endWhile_{}\n", uid).as_str());

    output
}


fn gen_assign(target: &Token, value: &Expr) -> String {
    // Evaluate expression into a, then store into memory
    let mut output = gen_evaluate(value);
    output.push_str(format!("\tld [{}], a\n", target.lexeme).as_str());

    output
}

fn gen_expression(expr: &Expr) -> String {
    gen_evaluate(expr)
}

fn gen_halt() -> String {
    String::from("\thalt\n")
}

fn gen_evaluate(expr: &Expr) -> String {
    match expr {
        Expr::Literal { value } => gen_evaluate_literal(value),
        Expr::Variable { name } => gen_evaluate_variable(name),
    }
}

fn gen_evaluate_literal(value: &u8) -> String {
    format!("\tld a, {}\n", value)
}

fn gen_evaluate_variable(name: &Token) -> String {
    format!("\tld a, [{}]\n", name.lexeme)
}
