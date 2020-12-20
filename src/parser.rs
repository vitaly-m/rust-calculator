use crate::parser::TokenType::*;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub t: TokenType,
    pub v: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Operand,
    Operator,
    OpenBrace,
    CloseBrace,
    ArgSeparator,
}

pub fn str_to_tokens(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            process_char(&mut tokens, c, Operand, None);
        } else if c == '.' {
            process_char(&mut tokens, c, Operand, Some("0"));
        } else if c == '(' {
            tokens.push(Token {
                t: OpenBrace,
                v: String::from(c),
            })
        } else if c == ')' {
            tokens.push(Token {
                t: CloseBrace,
                v: String::from(c),
            })
        } else if c == ',' {
            tokens.push(Token {
                t: ArgSeparator,
                v: String::from(c),
            })
        } else if c == '*'
            || c == '/'
            || c == '%'
            || c == '+'
            || c == '-'
            || c == '<'
            || c == '>'
            || c == '='
            || c == '!'
            || c == '&'
            || c == '^'
            || c == '|'
        {
            process_char(&mut tokens, c, Operator, None);
        }
    }
    tokens
}

fn process_char(tokens: &mut Vec<Token>, c: char, token_type: TokenType, prefix: Option<&str>) {
    let p = match prefix {
        None => "",
        Some(pp) => pp,
    };
    match tokens.last_mut() {
        None => tokens.push(Token {
            t: token_type,
            v: String::from(p) + &String::from(c),
        }),
        Some(t) => {
            if t.t == token_type {
                t.v.push(c);
            } else {
                tokens.push(Token {
                    t: token_type,
                    v: String::from(p) + &String::from(c),
                });
            }
        }
    };
}

pub fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut rpn = Vec::new();
    let mut ops = Vec::new();
    for token in tokens {
        if token.t == OpenBrace {
            ops.push(token);
        } else if token.t == CloseBrace {
            while let Some(op) = ops.pop() {
                if op.t == OpenBrace {
                    break;
                }
                rpn.push(op);
            }
        } else if token.t != Operator {
            rpn.push(token);
        } else {
            if ops.is_empty()
                || get_operator_precedence(&ops.last().unwrap().v)
                    > get_operator_precedence(&token.v)
            {
                ops.push(token);
            } else {
                while let Some(stack_op) = ops.last() {
                    if get_operator_precedence(&token.v) >= get_operator_precedence(&stack_op.v) {
                        rpn.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(token);
            }
        }
    }
    while let Some(op) = ops.pop() {
        rpn.push(op);
    }
    rpn
}

fn get_operator_precedence(op: &str) -> u8 {
    return if op == "^" {
        29
    } else if op == "*" || op == "/" || op == "%" {
        30
    } else if op == "+" || op == "-" {
        40
    } else if op == "<" || op == "<=" || op == ">" || op == ">=" {
        60
    } else if op == "&&" {
        110
    } else if op == "||" {
        120
    } else {
        u8::max_value()
    };
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn str_to_rpn_test_1() {
        let rpn: String = to_rpn(str_to_tokens("a+b*c+d"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("abc*+d+", rpn);
    }

    #[test]
    fn str_to_rpn_test_2() {
        let rpn: String = to_rpn(str_to_tokens("a+b*(c^d-e)^(f+g*h)-i"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("abcd^e-fgh*+^*+i-", rpn);
    }

    #[test]
    fn str_to_rpn_test_3() {
        let rpn: String = to_rpn(str_to_tokens("A*(B+C)/D"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("ABC+*D/", rpn);
    }

    #[test]
    fn str_to_rpn_test_4() {
        let rpn: String = to_rpn(str_to_tokens("(6+10-4)/(1+1*2)+1"))
            .iter()
            .flat_map(|t| t.v.chars())
            .collect();
        assert_eq!("610+4-112*+/1+", rpn);
    }
}
