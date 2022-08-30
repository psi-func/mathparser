pub mod parsemath;

use parsemath::{parser::{Parser, ParseError},
    ast};

pub fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("the generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
