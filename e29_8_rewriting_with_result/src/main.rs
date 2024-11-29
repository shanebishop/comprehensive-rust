use std::iter::Peekable;
use std::str::Chars;

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Tokenizer<'a> {
    fn collect_number(&mut self, first_char: char) -> Token {
        let mut num = String::from(first_char);
        while let Some(&c @ '0'..='9') = self.0.peek() {
            num.push(c);
            self.0.next();
        }
        Token::Number(num)
    }

    fn collect_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(&c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
            ident.push(c);
            self.0.next();
        }
        Token::Identifier(ident)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.0.next()?;
        Some(match c {
            '0'..='9' => Ok(self.collect_number(c)),
            'a'..='z' => Ok(self.collect_identifier(c)),
            '+' => Ok(Token::Operator(Op::Add)),
            '-' => Ok(Token::Operator(Op::Sub)),
            _ => return Some(Err(format!("Unexpected character {c}"))),
        })
    }
}

fn parse(input: &str) -> Result<Expression, String> {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Result<Expression, String> {
        let Some(tok) = tokens.next() else {
            return Err("Unexpected end of input".to_string());
        };
        let tok = tok?;
        let expr = match tok {
            Token::Number(num) => {
                let v = match num.parse() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("{e}")),
                };
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(format!("Unexpected token {tok:?}")),
        };
        // Look ahead to parse a binary operation if present.
        Ok(match tokens.next() {
            None => expr,
            Some(Ok(Token::Operator(op))) => {
                Expression::Operation(Box::new(expr), op, Box::new(parse_expr(tokens)?))
            }
            Some(Ok(tok)) => return Err(format!("Unexpected token {tok:?}")),
            Some(Err(s)) => return Err(s),
        })
    }

    parse_expr(&mut tokens)
}

fn main() {
    let expr = parse("10+foo+20-30");
    println!("{expr:?}");
}

#[test]
fn happy_path() {
    use Expression::*;
    use Op::*;

    assert_eq!(
        parse("10+foo+20-30"),
        Ok(Operation(
            Box::new(Number(10)),
            Add,
            Box::new(Operation(
                Box::new(Var("foo".to_string())),
                Add,
                Box::new(Operation(Box::new(Number(20)), Sub, Box::new(Number(30))))
            ))
        ))
    );
}
