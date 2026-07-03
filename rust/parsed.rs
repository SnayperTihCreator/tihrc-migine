use crate::tokenized::{Token, Operators};
use crate::ast::Node;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0 } }

    pub fn parse(&mut self) -> Result<Node, String> {
        let result = self.parse_expression(0)?;
        if !self.is_at_end() && !matches!(self.current(), Token::Operator(Operators::RParen) | Token::Operator(Operators::RBracket)) {
            return Err(format!("Unexpected token after expression: {:?}", self.current()));
        }
        Ok(result)
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Node, String> {
        if precedence > 0 && precedence <= 4 && matches!(self.current(), Token::Operator(Operators::Colon)) {
             return Ok(Node::EmptySlice);
        }

        let mut left = self.parse_prefix()?;
        
        loop {
            let is_implicit = self.is_implicit_multiplication();

            if let Token::Operator(Operators::Prime) = self.current() {
            let mut order = 0;
            while let Token::Operator(Operators::Prime) = self.current() {
                order += 1;
                self.advance();
            }
            let target_var = match &left {
                Node::FunctionCall { args, .. } => {
                    if args.len() == 1 {
                        if let Node::Variable(arg_name) = &args[0] {
                            arg_name.clone()
                        } else {
                            "x".to_string()
                        }
                    } else {
                        "x".to_string()
                    }
                }
                _ => "x".to_string(),
            };

            left = Node::DerivativeExpr {
                var: target_var, 
                order: Box::new(Node::Integer(order)),
                body: Box::new(left),
            };
            continue;
        }

            let (op_precedence, op) = if is_implicit {
                (7, Operators::Star)
            } else if let Token::Operator(op_type) = self.current() {
                if let Some(prec) = op_type.bin_precedence() {
                    (prec, *op_type)
                } else {
                    break;
                }
            } else {
                break;
            };

            if precedence >= op_precedence {
                break;
            }

            if !is_implicit { self.advance(); }

            if op == Operators::RangeDots {
                let end_node = self.parse_expression(op_precedence)?;
                if matches!(self.current(), Token::Operator(Operators::Comma)) {
                    self.advance();
                    let step_node = self.parse_expression(op_precedence)?;
                    
                    left = Node::Range {
                        start: Box::new(left),
                        end: Box::new(end_node),
                        step: Some(Box::new(step_node)),
                    };
                } else {
                    left = Node::Range {
                        start: Box::new(left),
                        end: Box::new(end_node),
                        step: None,
                    };
                }
                continue;
            }

            if op == Operators::LBracket {
                let mut indices = Vec::new();
                if !matches!(self.current(), Token::Operator(Operators::RBracket)) {
                    loop {
                        if matches!(self.current(), Token::Operator(Operators::Colon)) {
                            self.advance();
                            indices.push(Node::EmptySlice);
                        } else {
                            indices.push(self.parse_expression(0)?);
                        }

                        if matches!(self.current(), Token::Operator(Operators::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.consume(Token::Operator(Operators::RBracket), "Expected ']' at the end of matrix index")?;
                left = Node::Index { expr: Box::new(left), indices };
                continue;
            }

            let right = if op.is_right_associative() {
                self.parse_expression(op_precedence - 1)?
            } else {
                self.parse_expression(op_precedence)?
            };

            left = Node::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Node, String> {
        match self.current().clone() {
            Token::Integer(n) => {self.advance(); Ok(Node::Integer(n))}
            Token::Number(n) => { self.advance(); Ok(Node::Number(n)) }
            Token::Complex(re, im) => { self.advance(); Ok(Node::Complex(re, im)) }
            Token::Identifier(name) => {
                self.advance();
                if (name == "sum" || name == "prod" || name == "int" || name == "diff" || name == "lim")
                    && matches!(self.current(), Token::Operator(Operators::LBracket)) {
                    
                    self.advance();
                    
                    let var_name = if let Token::Identifier(v) = self.current().clone() {
                        self.advance();
                        v
                    } else {
                        return Err(format!("Expected variable name inside {}[...]", name));
                    };

                    if name == "sum" || name == "prod" {
                        self.consume(Token::Operator(Operators::Equal), "Expected '=' after variable in sum/prod")?;
                        
                        let range_node = self.parse_expression(0)?;
                        self.consume(Token::Operator(Operators::RBracket), "Expected ']' after bounds")?;
                        
                        self.consume(Token::Operator(Operators::LParen), "Expected '(' before body")?;
                        let body = self.parse_expression(0)?;
                        self.consume(Token::Operator(Operators::RParen), "Expected ')' after body")?;

                        if let Node::Range { start, end, step: _ } = range_node {
                            if name == "sum" {
                                return Ok(Node::Summation { var: var_name, start, end, body: Box::new(body) });
                            } else {
                                return Ok(Node::Product { var: var_name, start, end, body: Box::new(body) });
                            }
                        } else {
                            return Err("Expected a valid range (e.g., 1..10,2) inside sum/prod bounds".to_string());
                        }

                    } else if name == "int" {
                        if matches!(self.current(), Token::Operator(Operators::Comma)) {
                            self.advance();
                            let start_node = self.parse_expression(0)?;

                            self.consume(Token::Operator(Operators::Comma), "Expected ',' between integral bounds")?;
                            let end_node = self.parse_expression(0)?;

                            self.consume(Token::Operator(Operators::RBracket), "Expected ']' after integral bounds")?;
                            self.consume(Token::Operator(Operators::LParen), "Expected '(' before integral body")?;
                            let body_node = self.parse_expression(0)?;
                            self.consume(Token::Operator(Operators::RParen), "Expected ')' after integral body")?;

                            return Ok(Node::DefIntegral { 
                                var: var_name, 
                                start: Box::new(start_node), 
                                end: Box::new(end_node), 
                                body: Box::new(body_node) 
                            });
                        } else {
                            self.consume(Token::Operator(Operators::RBracket), "Expected ']' after integral variable")?;
                            self.consume(Token::Operator(Operators::LParen), "Expected '(' before integral body")?;
                            let body_node = self.parse_expression(0)?;
                            self.consume(Token::Operator(Operators::RParen), "Expected ')' after integral body")?;

                            return Ok(Node::IndefIntegral { 
                                var: var_name, 
                                body: Box::new(body_node) 
                            });
                        }
                    } else if name == "lim" {
                        self.consume(Token::Operator(Operators::Minus), "Expected '->' in limit definition")?;
                        self.consume(Token::Operator(Operators::GreaterThan), "Expected '->' in limit definition")?;

                        let target_node = self.parse_expression(0)?;

                        self.consume(Token::Operator(Operators::RBracket), "Expected ']' after limit definition")?;
                        self.consume(Token::Operator(Operators::LParen), "Expected '(' before limit body")?;
                        let body_node = self.parse_expression(0)?;
                        self.consume(Token::Operator(Operators::RParen), "Expected ')' after limit body")?;

                        return Ok(Node::Limit { 
                            var: var_name, 
                            target: Box::new(target_node), 
                            body: Box::new(body_node) 
                        });
                    } else {
                        let mut order = Box::new(Node::Number(1.0));
                        if matches!(self.current(), Token::Operator(Operators::Comma)) {
                            self.advance();
                            order = Box::new(self.parse_expression(0)?);
                        }
                        self.consume(Token::Operator(Operators::RBracket), "Expected ']' after diff specification")?;
                        
                        self.consume(Token::Operator(Operators::LParen), "Expected '('")?;
                        let body = self.parse_expression(0)?;
                        self.consume(Token::Operator(Operators::RParen), "Expected ')'")?;
                        
                        return Ok(Node::DerivativeExpr { var: var_name, order, body: Box::new(body) });
                    }
                }

                if matches!(self.current(), Token::Operator(Operators::LParen)) {
                    if matches!(self.look_ahead(1), Token::Operator(Operators::Equal)) || self.is_function_def_lookahead() {
                        self.parse_function_def(name)
                    } else {
                        self.parse_function_call(name)
                    }
                } else {
                    if name.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()){
                        Ok(Node::Constant(name))
                    } else {
                        Ok(Node::Variable(name))
                    }
                }
            }
            Token::Operator(op) => {
                if let Some(prec) = op.unary_precedence() {
                    self.advance();
                    let child = self.parse_expression(prec)?;
                    Ok(Node::UnaryOp { op, child: Box::new(child) })
                } else if op == Operators::LParen {
                    self.advance();
                    let expr = self.parse_expression(0)?;
                    self.consume(Token::Operator(Operators::RParen), "Expected ')'")?;
                    Ok(expr)
                } else if op == Operators::LBracket {
                    self.parse_matrix()
                } else if op == Operators::Colon {
                    self.advance();
                    Ok(Node::EmptySlice)
                } else {
                    Err(format!("Unexpected prefix operator: {:?}", op))
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }

    fn parse_matrix(&mut self) -> Result<Node, String> {
        self.consume(Token::Operator(Operators::LBracket), "Expected '['")?;

        if matches!(self.current(), Token::Operator(Operators::RBracket)) {
            self.advance();
            return Ok(Node::Matrix { rows: vec![] });
        }

        let mut rows = Vec::new();

        while !matches!(self.current(), Token::Operator(Operators::RBracket)) {
            let mut row = Vec::new();

            loop {
                if matches!(self.current(), Token::Operator(Operators::RBracket)) {
                    break;
                }
                if matches!(self.current(), Token::Operator(Operators::Semicolon)) {
                    self.advance();
                    break;
                }

                let expr = self.parse_expression(0)?;
                row.push(expr);

                match self.current() {
                    Token::Operator(Operators::Comma) => {
                        self.advance(); // запятая – следующий элемент в той же строке
                        continue;
                    }
                    Token::Operator(Operators::Semicolon) => {
                        self.advance(); // точка с запятой – конец строки
                        break;
                    }
                    Token::Operator(Operators::RBracket) => {
                        break; // конец матрицы
                    }
                    _ => {
                        return Err(format!(
                            "Expected ',', ';' or ']' after matrix element, got {:?}",
                            self.current()
                        ));
                    }
                }
            }

            rows.push(row);
        }

        self.consume(Token::Operator(Operators::RBracket), "Expected ']' at end of matrix")?;
        Ok(Node::Matrix { rows })
    }

    fn parse_function_def(&mut self, name: String) -> Result<Node, String> {
        self.consume(Token::Operator(Operators::LParen), "Expected '('")?;
        let mut params = Vec::new();
        if !matches!(self.current(), Token::Operator(Operators::RParen)) {
            loop {
                if let Token::Identifier(param) = self.current().clone() {
                    params.push(param);
                    self.advance();
                } else {
                    return Err("Expected parameter identifier in function def".to_string());
                }
                if matches!(self.current(), Token::Operator(Operators::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::Operator(Operators::RParen), "Expected ')'")?;
        self.consume(Token::Operator(Operators::Equal), "Expected '=' in function definition")?;
        let body = self.parse_expression(0)?;
        Ok(Node::FunctionDef { name, params, body: Box::new(body) })
    }

    fn parse_function_call(&mut self, name: String) -> Result<Node, String> {
        self.consume(Token::Operator(Operators::LParen), "Expected '('")?;
        let mut args = Vec::new();
        if !matches!(self.current(), Token::Operator(Operators::RParen)) {
            loop {
                args.push(self.parse_expression(0)?);
                if matches!(self.current(), Token::Operator(Operators::Comma)) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::Operator(Operators::RParen), "Expected ')'")?;
        Ok(Node::FunctionCall { name, args: Box::new(args) })
    }

    fn is_implicit_multiplication(&self) -> bool {
        matches!(
            (self.previous(), self.current()),
            (
                Token::Integer(_) | Token::Number(_) | Token::Complex(_, _) | Token::Identifier(_) | Token::Operator(Operators::RParen) | Token::Operator(Operators::RBracket),
                Token::Number(_) | Token::Complex(_, _) | Token::Identifier(_) | Token::Operator(Operators::LParen)
            )
        )
    }

    fn is_function_def_lookahead(&self) -> bool {
        let mut p = self.pos;
        if p >= self.tokens.len() || !matches!(self.tokens[p], Token::Operator(Operators::LParen)) { return false; }
        p += 1;
        while p < self.tokens.len() && !matches!(self.tokens[p], Token::Operator(Operators::RParen)) {
            p += 1;
        }
        p += 1;
        p < self.tokens.len() && matches!(self.tokens[p], Token::Operator(Operators::Equal))
    }

    fn consume(&mut self, expected: Token, err_msg: &str) -> Result<(), String> {
        if self.current() == &expected { self.advance(); Ok(()) } else { Err(err_msg.to_string()) }
    }

    fn advance(&mut self) { if !self.is_at_end() { self.pos += 1; } }
    fn is_at_end(&self) -> bool { self.tokens[self.pos] == Token::EOF }
    fn current(&self) -> &Token { &self.tokens[self.pos] }
    fn previous(&self) -> &Token { if self.pos > 0 { &self.tokens[self.pos - 1] } else { &self.tokens[0] } }
    fn look_ahead(&self, distance: usize) -> &Token {
        if self.pos + distance < self.tokens.len() { &self.tokens[self.pos + distance] } else { &Token::EOF }
    }
}