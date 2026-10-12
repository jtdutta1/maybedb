use std::fmt::format;
use std::iter::Peekable;
use std::vec::IntoIter;
use std::mem::replace;

use crate::compiler::{KEYWORDS, OPERATORS, LOGICAL_OPERATOR, CONTAINER_OPERATOR, ARITHMETIC_OPERATOR, MISC_OPERATOR};
use crate::compiler::tokenizer::Token;
use crate::db::types::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum GranularToken{
    Begin,
    End,
    CreateTable(String),
    CreateDatabase(String),
    AlterTable(String),
    AlterDatabase(String),
    DropTable(String),
    DropDatabase(String),
    TruncateTable(String),
    InsertInto(String),
    SelectFrom(String),
    // ValuesWithSubtree,
    Values(Vec<Box<GranularToken>>),
    WhereCondition,
    SelectAllOperator, // * operator
    AddOperator(Vec<Box<GranularToken>>),
    SubtractOperator(Vec<Box<GranularToken>>),
    MultiplyOperator(Vec<Box<GranularToken>>),
    DivideOperator(Vec<Box<GranularToken>>),
    AndOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    OrOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    NotOperator{operand: Box<GranularToken>},
    LessThanOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    LEOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    GreaterThanOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    GEOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    EqualityOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    NonEqualityOperator{left_operand: Box<GranularToken>, right_operand: Box<GranularToken>},
    StringLiteral(String),
    IntegerLiteral(i32),
    DecimalLiteral(f64),
    Identifier(String)
}

impl GranularToken {
    // pub fn get_gn_from_token(token: &Token) -> Result<GranularToken, String> {
    //     match token {
    //         Token::Keyword(value) => Self::parse_keyword(value),
    //         Token::Identifier(value) => Ok(Self::Identifier(value.clone())),
    //         Token::Operator(value) => Self::parse_operator(value),
    //         Token::Literal(value) => Self::parse_literal(value),
    //         _ => Err(String::from("ERROR:\nFound unsupported token during parsing!"))
    //     }
    // }

    fn is_insert_token(token: &GranularToken) -> bool {
        if matches!(token, &GranularToken::InsertInto(_)) {
            true
        } else {
            false
        }
    }

    fn is_identifier(token: &GranularToken) -> bool {
        if let GranularToken::Identifier(_) = token {
            true
        } else {
            false
        }
    }

    fn is_literal(token: &GranularToken) -> bool {
        if let GranularToken::StringLiteral(_) = token {
            true
        } else if let GranularToken::IntegerLiteral(_) = token {
            true
        } else if let GranularToken::DecimalLiteral(_) = token {
            true
        } else {
            false
        }
    }
}

pub struct Node {
    token: Box<GranularToken>,
    next: Option<Box<Node>>
}

struct AST {
    root: Box<Node>
}

impl AST {
    pub fn new() -> AST {
        let begin_node = Node {
            token: Box::new(GranularToken::Begin),
            next: None
        };
        AST{root: Box::new(begin_node)}
    }

    pub fn parse_tokens_into_ast(&mut self, tokens: Vec<Token>) -> Result<Vec<GranularToken>, String> {
        let mut token_vector_iterator = tokens.into_iter();
        let mut node = &mut self.root;
        let mut new_node: Node;
        // let (mut insert_flag, mut select_flag) = (false, false);
        while let Some(token) = token_vector_iterator.next() {
            match token {
                Token::Keyword(value) => {
                    new_node = Self::parse_keyword(value, &mut token_vector_iterator)?;
                    if GranularToken::is_insert_token(&new_node.token) {
                        // insert_flag = true;
                    }
                    node.next = Some(Box::new(new_node));
                    node = node.next.as_mut().unwrap();
                },
                Token::Identifier(value) => {
                    new_node = Node {
                        token: Box::new(GranularToken::Identifier(value)),
                        next: None
                    };
                    node.next = Some(Box::new(new_node));
                    node = node.next.as_mut().unwrap();
                },
                Token::Operator(value) => {
                    // parse operator
                    // when the return value is Some, then the node's next element will be the new
                    // node in Some. Otherwise, the node has been replaced and there is nothing to do
                    match Self::parse_operator(node, value, &mut token_vector_iterator)? {
                        Some(n) => {node.next = Some(n); node = node.next.as_mut().unwrap()},
                        None => (),
                    }
                },
                Token::Literal(value_type) => {
                    let literal_granular_token: GranularToken;
                    if let Value::Float(value) = value_type {
                        literal_granular_token = GranularToken::DecimalLiteral(value);
                    }
                    else if let Value::Integer(value) = value_type {
                        literal_granular_token = GranularToken::IntegerLiteral(value);
                    }
                    else if let Value::Text(value) = value_type {
                        literal_granular_token = GranularToken::StringLiteral(value);
                    }
                    else {
                        return Err(String::from("ERROR:\nCurrently literals other than type TEXT, INTEGER AND DECIMAL are not supported!"));
                    }
                    new_node = Node {
                        token: Box::new(literal_granular_token),
                        next: None
                    };
                    node.next = Some(Box::new(new_node));
                    node = node.next.as_mut().unwrap();
                },
                // _ => {return Err(String::from("ERROR:\nUnimplemented logic error!"));}
            }
        }
        // attach an end token node
        node.next = Some(
            Box::new(
                Node {
                    token: Box::new(GranularToken::End),
                    next: None
                }
            )
        );
        Ok(self.to_vec())
    }

    fn parse_keyword(keyword: String, token_vector_iterator: &mut IntoIter<Token>) -> Result<Node, String> {
        let new_node = match keyword.as_str() {
            "CREATE" => {
                let next_token = token_vector_iterator
                                        .next()
                                        .expect("ERROR:\nExpected to find more after CREATE");
                let new_node : Node;
                if next_token == Token::Keyword("TABLE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::CreateTable (Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                } 
                else if next_token == Token::Keyword("DATABASE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::CreateDatabase (Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                }
                else {
                    return Err(
                        "ERROR:\nCREATE should be followed by a TABLE or a DATABASE keyword."
                        .to_string()
                    );
                }
                new_node
            },
            "ALTER" => {
                let next_token = token_vector_iterator
                                        .next()
                                        .expect("ERROR:\nExpected to find more after ALTER");
                let new_node: Node;
                if next_token == Token::Keyword("TABLE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::AlterTable(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                } 
                else if next_token == Token::Keyword("DATABASE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::AlterDatabase(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                }
                else {
                    return Err(
                        "ERROR:\nALTER should be followed by a TABLE or a DATABASE keyword."
                        .to_string()
                    );
                }
                new_node
            },
            "DROP" => {
                let next_token = token_vector_iterator
                                        .next()
                                        .expect("ERROR:\nExpected to find more after DROP");
                let new_node: Node;
                if next_token == Token::Keyword("TABLE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::DropTable(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                } 
                else if next_token == Token::Keyword("DATABASE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::DropDatabase(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                }
                else {
                    return Err(
                        "ERROR:\nDROP should be followed by a TABLE or a DATABASE keyword."
                        .to_string()
                    );
                }
                new_node
            },
            "TRUNCATE" => {
                let next_token = token_vector_iterator
                                        .next()
                                        .expect("ERROR:\nExpected to find more after TRUNCATE");
                let new_node: Node;
                if next_token == Token::Keyword("TABLE".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::TruncateTable(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None,
                    };
                } 
                else {
                    return Err(
                        "ERROR:\nTRUNCATE should be followed by a TABLE or a DATABASE keyword."
                        .to_string()
                    );
                }
                new_node
            },
            "INSERT" => {
                let next_token = token_vector_iterator
                                        .next()
                                        .expect("ERROR:\nExpected to find more after INSERT");
                let new_node: Node;
                if next_token == Token::Keyword("INTO".to_string()) {
                    new_node = Node {
                        token: Box::new(GranularToken::InsertInto(Self::parse_table_or_db_name(&token_vector_iterator.next())?)),
                        next: None
                    };
                } else {
                    return Err(
                        "ERROR:\nINSERT should be followed by INTO!"
                        .to_string()
                    );
                }
                new_node
            },
            "VALUES" => Self::parse_values(token_vector_iterator)?,
            "SELECT" => {
                // find the FROM keyword
                // to do this first collect the remaining elements in the iterator
                // into a vector.
                // then in that vector find the existence of FROM. If it doesn't exists, 
                // return an error. Once found, pop FROM out and the name of the table
                // and modify the iterator with the iterator of the remaining vector.
                let mut remaining_tokens: Vec<Token> = token_vector_iterator.collect();
                if !remaining_tokens.iter().any(|t| t == &Token::Keyword("FROM".to_string())) {
                    return Err(String::from("ERROR:\nExpected a FROM associated with the SELECT keyword."));
                } else {
                    // find table name after FROM
                    let iterator = &mut remaining_tokens.into_iter();
                    let remaining_tokens1: Vec<Token> = iterator
                                                    .into_iter()
                                                    .take_while(|t| t != &Token::Keyword("FROM".to_string()))
                                                    .collect();
                    let table_name = match iterator.next() {
                        Some(Token::Identifier(name)) => name,
                        _ => {return Err(String::from("ERROR:\nExpected a table name after FROM!"));}
                    };
                    let remaining_tokens2: Vec<Token> = iterator.collect();
                    remaining_tokens = remaining_tokens1
                                        .into_iter()
                                        .chain(remaining_tokens2.into_iter())
                                        .collect();
                    *token_vector_iterator = remaining_tokens.into_iter();
                    
                    Node {
                        token: Box::new(GranularToken::SelectFrom(table_name)),
                        next: None
                    }

                }
            },
            "WHERE" => {
                Node {
                    token: Box::new(GranularToken::WhereCondition),
                    next: None
                }
            }
            _ => {return Err(format!("ERROR:\nUnexpected keyword {keyword}!"));}
        
        };
        Ok(new_node)
    }

    fn parse_values(token_vector_iterator: &mut IntoIter<Token>) -> Result<Node, String> {
        let mut next_token = token_vector_iterator
                                            .next()
                                            .expect("ERROR:\nUnexpected end of tokens. Expected a \"(\" after \"VALUES\"");
        // Check if the next token is a ( 
        // marking the beginning of the values list
        if next_token == Token::Operator("(".to_string()) {
            let mut value_granular_tokens: Vec<Box<GranularToken>> = Vec::new();
            loop {
                next_token = token_vector_iterator
                                .next()
                                .expect("ERROR:\nUnexpected end of tokens while parsing values after the \"VALUES\" token!");
                if next_token == Token::Operator(")".to_string()) {
                    return Ok(Node {
                        token: Box::new(GranularToken::Values(value_granular_tokens)),
                        next:None
                    });
                }
                let internal_token_value = match next_token {
                    Token::Literal(Value::Integer(value)) => GranularToken::IntegerLiteral(value),
                    Token::Literal(Value::Float(value)) => GranularToken::DecimalLiteral(value),
                    Token::Literal(Value::Text(value)) => GranularToken::StringLiteral(value),
                    _ => {return Err(String::from("ERROR:\nAt this point only integer, text and float literals are supported of type Value!"));}
                };
                value_granular_tokens.push(Box::new(internal_token_value));
            }
        } else {
            Err(String::from("ERROR:\nExpected a \"(\" operator as the start of \"VALUES\" definition!"))
        }
    }

    fn parse_table_or_db_name(token: &Option<Token>) -> Result<String, String> {
        match token {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            _ => Err(String::from("ERROR:\nExpected an identifier for the table/database name!")),
        }
    }

    fn parse_operator(node: &mut Box<Node>, operator: String, token_vector_iterator: &mut IntoIter<Token>) -> Result<Option<Box<Node>>, String> {
        let mut peekable_iterator = token_vector_iterator.peekable();

        // check if operator is a logical operator
        if LOGICAL_OPERATOR.iter().any(|&op| op == operator.as_str()) {
            // find the left operand
            let left_operand = node.token.clone();

            // make sure it is a literal or an identifier
            if !GranularToken::is_identifier(&left_operand) &&
            !GranularToken::is_literal(&left_operand) {
                // check if the operator is NOT
                if operator.as_str() != "NOT" {
                    return Err(format!("ERROR:\nToken before {operator} expected to be a literal or identifier (except in case of NOT operator)"));
                }
            }

            // check if right operand is an identifier
            let right_operand: Box<GranularToken>;
            if let Token::Identifier(value) = peekable_iterator.peek().unwrap() {
                right_operand = Box::new(GranularToken::Identifier(value.clone()));
                peekable_iterator.next();
            }
            // check if the right operand is a literal 
            else if let Token::Literal(value_type) = peekable_iterator.peek().unwrap() {
                if let Value::Float(value) = value_type {
                    right_operand = Box::new(GranularToken::DecimalLiteral(*value));
                }
                else if let Value::Integer(value) = value_type {
                    right_operand = Box::new(GranularToken::IntegerLiteral(*value));
                }
                else if let Value::Text(value) = value_type {
                    right_operand = Box::new(GranularToken::StringLiteral(value.clone()));
                }
                else {
                    return Err(String::from("ERROR:\nCurrently literals other than type TEXT, INTEGER AND DECIMAL are not supported!"));
                }
                peekable_iterator.next();
            }
            // other than these 2 is an error
            else {
                return Err(format!("ERROR:\nExpected an identifier or literal after \"{operator}\"."));
            }
            if operator == "NOT" {
                // we return the new node
                Ok(Self::parse_logical_operator(&operator, None, right_operand))
                // node.next = new_node;
                // final_node = node.next.unwrap();
                // Ok(new_node)
            } else {
                // the new node we get back is supposed to replace the existing node
                // as the existing node is a literal/identifier used by the new node (op node)
                // and we return None.
                // if, however, we get back nothing, then we return an error
                match Self::parse_logical_operator(&operator, Some(left_operand), right_operand) {
                    Some(n) => {let _ = replace(node, n); return Ok(None)},
                    None => {return Err(format!("ERROR:\nExpected a valid logical operator. Found {operator}"))},
                }
            }
            // Ok(Some(final_node))
        } 
        // check if operator an arithmetic operator
        else if ARITHMETIC_OPERATOR.iter().any(|&op| op == operator.as_str()) {
            //  find all the operands within the bounds of the operator open token and close container token
            let operands: Vec<Token> = peekable_iterator
                                .take_while(|t| t != &Token::Operator(")".to_string()))
                                .collect();
            // make sure the tokens are either all literals
            if operands.len() > 1 && !operands.iter().any(|t| matches!(t, Token::Literal(_))) {
                Err(format!("ERROR:\nEncountered {operator} but the operands should all be a literal or a single identifier"))
            }
            // or at most 1 identifier/literal
            else if operands.len() == 1 && !operands.iter().all(|t| matches!(t, Token::Literal(_))) && 
            !operands.iter().all(|t| matches!(t, &Token::Identifier(_))) {
                Err(format!("ERROR:\nEncountered {operator} but the operands should be a single identifier or literals"))
            }
            // and more than length 0
            else if operands.len() < 1 {
                Err(format!("ERROR:\nEncountered {operator} but couldn't find any operands"))
            }
            else {
                Ok(Self::parse_arithmetic_operator(&operator, operands))
            }
        }
        // check if the operator is a miscellaneous operator
        else if MISC_OPERATOR.iter().any(|&op| op == operator.as_str()) {
            // handle miscellaneous ops
            // for now just selectall op is the miscellaneous op and
            // return this new node
            Ok(
                Some(
                    Box::new(
                        Node {
                            token: Box::new(GranularToken::SelectAllOperator),
                            next: None
                        }
                    )
                )
            )
        }
        // check if the operator is an opening container operator only
        else if operator.as_str() == "(" {
            Err(format!("ERROR:\nContainer operator \"(\" not yet supported."))
        }
        else {
            Err(format!("ERROR:\nCoudln't parse operator {operator}"))
        }
    }
    fn parse_logical_operator(operator: &String, left: Option<Box<GranularToken>>, right: Box<GranularToken>) -> Option<Box<Node>> {
        let new_granular_token = match operator.as_str() {
            ">" => GranularToken::GreaterThanOperator { 
                left_operand: left.unwrap(), 
                right_operand: right 
            },
            "<" => GranularToken::LessThanOperator{
                left_operand: left.unwrap(),
                right_operand: right
            },
            "=" => GranularToken::EqualityOperator {
                left_operand: left.unwrap(),
                right_operand: right
            },
            ">=" => GranularToken::GEOperator {
                left_operand: left.unwrap(),
                right_operand: right
            },
            "<=" => GranularToken::LEOperator {
                left_operand: left.unwrap(),
                right_operand: right
            },
            "!=" => GranularToken::NonEqualityOperator {
                left_operand: left.unwrap(),
                right_operand: right
            },
            "AND" => GranularToken::AndOperator {
                left_operand: left.unwrap(),
                right_operand: right
            },
            "OR" => GranularToken::OrOperator{
                left_operand: left.unwrap(),
                right_operand: right
            },
            "NOT" => GranularToken::NotOperator {
                operand: right
            },
            _ => {return None;},
        };
        Some(
            Box::new(
                Node { 
                    token: Box::new(new_granular_token), 
                    next: None 
                }
            )
        )
    }

    fn parse_arithmetic_operator(operator: &String, operands: Vec<Token>) -> Option<Box<Node>> {
        // map the tokens into Vec<Box<GranularToken>>
        let modified_operands: Vec<Box<GranularToken>> = operands.into_iter().map(|t| {
            if let Token::Identifier(value) = t {
                Box::new(GranularToken::Identifier(value))
            } else {
                if let Token::Literal(Value::Float(value)) = t {
                    Box::new(GranularToken::DecimalLiteral(value))
                } else if let Token::Literal(Value::Integer(value)) = t {
                    Box::new(GranularToken::IntegerLiteral(value))
                } else if let Token::Literal(Value::Text(value)) = t {
                    Box::new(GranularToken::StringLiteral(value))
                } else {
                    panic!("ERROR:\nEncountered an illegal operand for the operator {operator}");
                }
            }
        }).collect();
        let new_token = match operator.as_str() {
            "ADD(" => GranularToken::AddOperator (
                modified_operands
            ),
            "SUB(" => GranularToken::SubtractOperator (
                modified_operands
            ),
            "MUL(" => GranularToken::MultiplyOperator (
                modified_operands
            ),
            "DIV(" => GranularToken::DivideOperator (
                modified_operands
            ),
            _ => {return None}
        };
        Some(
            Box::new(
                Node { 
                    token: Box::new(new_token), 
                    next: None 
                }
            )
        )
    }

    fn is_literal_token(token: &Token) -> bool {
        if let Token::Literal(_) = token {
            true
        } else {
            false
        }
    }

    fn is_identifier_token(token: &Token) -> bool {
        if let Token::Identifier(_) = token {
            true
        } else {
            false
        }
    }
    pub fn to_vec(&self) -> Vec<GranularToken> {
        let mut node = &self.root;
        let mut vec: Vec<GranularToken> = vec!();
        while node.next.is_some() {
            vec.push(*node.token.clone());
            node = node.next.as_ref().unwrap();
        }
        vec.push(*node.token.clone());
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_granular_token_parsing() {
        let tokens1 : Vec<Token> = vec!(
            Token::Keyword(String::from("CREATE")),
            Token::Keyword(String::from("TABLE")),
            Token::Identifier(String::from("table_name")),
        );

        let tokens2 : Vec<Token> = vec!(
            Token::Keyword("SELECT".to_string()), 
            Token::Operator("*".to_string()), 
            Token::Keyword("FROM".to_string()), 
            Token::Identifier("table_name".to_string()), 
            Token::Keyword("WHERE".to_string()), 
            Token::Identifier("column1".to_string()), 
            Token::Operator("<".to_string()), 
            Token::Literal(Value::Integer(3))
        );

        let tokens3: Vec<Token> = vec!(
            Token::Keyword("SELECT".to_string()), 
            Token::Operator("ADD(".to_string()), 
            Token::Identifier("column1".to_string()),
            Token::Operator(")".to_string()), 
            Token::Keyword("FROM".to_string()), 
            Token::Identifier("table_name".to_string()), 
            Token::Keyword("WHERE".to_string()), 
            Token::Identifier("column1".to_string()), 
            Token::Operator("<".to_string()), 
            Token::Literal(Value::Integer(3))
        );

        let tokens4: Vec<Token> = vec!(
            Token::Keyword("INSERT".to_string()),
            Token::Keyword("INTO".to_string()),
            Token::Identifier("table_name".to_string()),
            Token::Keyword("VALUES".to_string()),
            Token::Operator("(".to_string()),
            Token::Literal(Value::Text("name".to_string())),
            Token::Literal(Value::Integer(22)),
            Token::Operator(")".to_string())
        );

        // let tokens4: Vec<Token> = vec!(
        //     Token::Keyword("SELECT".to_string()), 
        //     Token::Operator("ADD(".to_string()), 
        //     Token::Identifier("column1".to_string()), 
        //     Token::Operator(")".to_string()), 
        //     Token::Keyword("FROM".to_string()), 
        //     Token::Identifier("table_name".to_string()),
        //     Token::Literal(Value::Text("\"smoosh\"".to_string())),
        //     Token::Keyword("WHERE".to_string()), 
        //     Token::Identifier("column1".to_string()), 
        //     Token::Operator("<=".to_string()), 
        //     Token::Literal(Value::Integer(3))
        // );

        // let actual_tokens5: String = "ERROR:\nCouldn't parse token ARD(".to_string();
        let mut ast = AST::new();
        let predicted_tokens1 = ast.parse_tokens_into_ast(tokens1);
        let predicted_tokens2 = ast.parse_tokens_into_ast(tokens2);
        let predicted_tokens3 = ast.parse_tokens_into_ast(tokens3);
        let predicted_tokens4 = ast.parse_tokens_into_ast(tokens4);

        let actual_tokens1: Result<Vec<GranularToken>, String> = Ok(vec!(
            GranularToken::Begin,
            GranularToken::CreateTable("table_name".to_string()),
            GranularToken::End
        ));

        let actual_tokens2: Result<Vec<GranularToken>, String> = Ok(vec!(
            GranularToken::Begin,
            GranularToken::SelectFrom("table_name".to_string()),
            GranularToken::SelectAllOperator,
            GranularToken::WhereCondition,
            GranularToken::LessThanOperator { 
                left_operand: Box::new(GranularToken::Identifier("column1".to_string())), 
                right_operand: Box::new(GranularToken::IntegerLiteral(3)) 
            },
            GranularToken::End
        ));

        let actual_tokens3: Result<Vec<GranularToken>, String> = Ok(vec!(
            GranularToken::Begin,
            GranularToken::SelectFrom("table_name".to_string()),
            GranularToken::AddOperator(
                vec!(
                    Box::new(
                        GranularToken::Identifier("column1".to_string())
                    )
                )
            ),
            GranularToken::WhereCondition,
            GranularToken::LessThanOperator { 
                left_operand: Box::new(GranularToken::Identifier("column1".to_string())), 
                right_operand: Box::new(GranularToken::IntegerLiteral(3)) 
            },
            GranularToken::End
        ));

        let actual_tokens4: Result<Vec<GranularToken>, String> = Ok(vec!(
            GranularToken::Begin,
            GranularToken::InsertInto("table_name".to_string()),
            GranularToken::Values(
                vec!(
                    Box::new(GranularToken::StringLiteral("name".to_string())),
                    Box::new(GranularToken::IntegerLiteral(22))
                )
            ),
            GranularToken::End
        ));

        assert_eq!(predicted_tokens1, actual_tokens1);
        assert_eq!(predicted_tokens2, actual_tokens2);
        assert_eq!(predicted_tokens3, actual_tokens3);
        assert_eq!(predicted_tokens4, actual_tokens4);
    }
}