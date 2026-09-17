use std::sync::LazyLock;
use regex::Regex;

// use crate::compiler::parser::Token::Literal;
/* 
    To parse a list of strings into a syntax, we need to understand every token's viability. 
    So we define it in the following way:-
    - Keywords: They are reserved words which carry meanings for operations
    - Identifiers: They serve as names for identification, not operation
    - Literals: Tokens containing numeric or literal values to be used for operations
    - Operators: Characters to perform certain operations
 */
use crate::db::types::Value;

use crate::compiler::{KEYWORDS, OPERATORS};

// define reserved keywords


// define regex strings for token parsing

/*
    Identifiers are names used for naming values. Like results of numerical operations
    or setting new aliases for tables. 

    Identifiers can be present as names or aliases. They are used as names for 
    columns, tables and databases
*/

static RE_IDENTIFIER: LazyLock<Regex> = 
    LazyLock::new(|| Regex::new(r"^[A-Za-z_][A-Za-z0-9_$#]*$").unwrap());

static RE_STRING_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?:'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*")$"#).unwrap());

static RE_INTEGER_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-?\d+$").unwrap());

static RE_DECIMAL_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-?\d+\.\d+$").unwrap());

/*
    Literals can be of 3 types:-
    - Integers
    - Decimals
    - Strings

    Integers and Decimals are differentiated by the period character, where the former doesn't have it
*/

#[derive(PartialEq, Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    // Operand(String), // operand can be a literal or an identifier
    Literal(Value)
}

impl Token {
    pub fn parse_token(token: &String) -> Result<Token, String> {
        /* 
            Parse string into individual token value. Token values will be chosen
            based on the category a token matches with. 
        */
        // check if it is a keyword
        if Self::is_keyword(token) {
            Ok(Token::Keyword(token.clone()))
        } 
        // else check if it is a literals
        else if let Some(parsed_value) = Self::is_literal(token) {
            Ok(Token::Literal(parsed_value))
        } 
        // check if it is an operator
        else if Self::is_operator(token) {
            Ok(Token::Operator(token.clone()))
        }
        // check if it is an identifier
        else if Self::is_identifier(token) {
            Ok(Token::Identifier(token.clone()))
        }
        else {
            Err(format!("ERROR:\nCouldn't parse token {token}"))
        }
    }

    fn is_keyword(value: &String) -> bool {
        KEYWORDS
        .iter()
        .any(|&word| word == value.as_str())
    }
    
    fn is_operator(value: &String) -> bool {
        OPERATORS
        .iter()
        .any(|&operator| operator == value.as_str())
    }

    fn is_literal(value: &String) -> Option<Value> {
        // create the 2 string and numeric type of regex
        if RE_STRING_LITERAL.is_match(value) {
            Some(Value::Text(value.clone()))
        } else if RE_INTEGER_LITERAL.is_match(value) {
            Some(Value::Integer(
                value
                .clone()
                .as_str()
                .trim()
                .parse::<i32>()
                .expect("ERROR:\nCouldn't parse {value} into type INTEGER")
            ))
        } else if RE_DECIMAL_LITERAL.is_match(value) {
            Some(Value::Float(
                value
                .clone()
                .as_str()
                .trim()
                .parse::<f64>()
                .expect("ERROR:\nCouldn't parse {value} into type FLOAT")
            ))
        } else {
            None
        }
    }

    fn is_identifier(value: &String) -> bool {
        // identifiers are names and they begin with a letter or an underscore
        // and containno other special characters but $ and #
        RE_IDENTIFIER.is_match(value)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_token_parsing() {
        let tokens1: Vec<String> = vec!(
            "SELECT".to_string(), 
            "column1".to_string(), 
            "column2".to_string(), 
            "column3".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string()
        );
        let tokens2: Vec<String> = vec!(
            "SELECT".to_string(), 
            "*".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(), 
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<".to_string(), 
            "3".to_string()
        );
        let tokens3: Vec<String> = vec!(
            "SELECT".to_string(), 
            "ADD(".to_string(), 
            "column1".to_string(), 
            ")".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(), 
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<".to_string(), 
            "3".to_string()
        );
        let tokens4: Vec<String> = vec!(
            "SELECT".to_string(), 
            "ADD(".to_string(), 
            "column1".to_string(), 
            ")".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(),
            "\"smoosh\"".to_string(),
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<=".to_string(), 
            "3".to_string()
        );
        let tokens5: Vec<String> = vec!(
            "SELECT".to_string(), 
            "ARD(".to_string(), 
            "column1".to_string(), 
            ")".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(),
            "\"smoosh\"".to_string(),
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<=".to_string(), 
            "3".to_string()
        );

        let parsed_tokens1 : Result<Vec<Token>, String> = tokens1
                                            .iter()
                                            .map(|token| Token::parse_token(token))
                                            .collect();

        let parsed_tokens2 : Result<Vec<Token>, String> = tokens2
                                            .iter()
                                            .map(|token| Token::parse_token(token))
                                            .collect();
        let parsed_tokens3 : Result<Vec<Token>, String> = tokens3
                                            .iter()
                                            .map(|token| Token::parse_token(token))
                                            .collect();
        let parsed_tokens4 : Result<Vec<Token>, String> = tokens4
                                            .iter()
                                            .map(|token| Token::parse_token(token))
                                            .collect();
        let parsed_tokens5 : Result<Vec<Token>, String> = tokens5
                                            .iter()
                                            .map(|token| Token::parse_token(token))
                                            .collect();

        let actual_tokens1 : Vec<Token> = vec!(
            Token::Keyword(String::from("SELECT")),
            Token::Identifier(String::from("column1")),
            Token::Identifier(String::from("column2")),
            Token::Identifier(String::from("column3")),
            Token::Keyword(String::from("FROM")),
            Token::Identifier(String::from("table_name")),
        );

        let actual_tokens2 : Vec<Token> = vec!(
            Token::Keyword("SELECT".to_string()), 
            Token::Operator("*".to_string()), 
            Token::Keyword("FROM".to_string()), 
            Token::Identifier("table_name".to_string()), 
            Token::Keyword("WHERE".to_string()), 
            Token::Identifier("column1".to_string()), 
            Token::Operator("<".to_string()), 
            Token::Literal(Value::Integer(3))
        );

        let actual_tokens3: Vec<Token> = vec!(
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

        let actual_tokens4: Vec<Token> = vec!(
            Token::Keyword("SELECT".to_string()), 
            Token::Operator("ADD(".to_string()), 
            Token::Identifier("column1".to_string()), 
            Token::Operator(")".to_string()), 
            Token::Keyword("FROM".to_string()), 
            Token::Identifier("table_name".to_string()),
            Token::Literal(Value::Text("\"smoosh\"".to_string())),
            Token::Keyword("WHERE".to_string()), 
            Token::Identifier("column1".to_string()), 
            Token::Operator("<=".to_string()), 
            Token::Literal(Value::Integer(3))
        );

        let actual_tokens5: String = "ERROR:\nCouldn't parse token ARD(".to_string();

        assert_eq!(parsed_tokens1, Ok(actual_tokens1));
        assert_eq!(parsed_tokens2, Ok(actual_tokens2));
        assert_eq!(parsed_tokens3, Ok(actual_tokens3));
        assert_eq!(parsed_tokens4, Ok(actual_tokens4));
        assert_eq!(parsed_tokens5, Err(actual_tokens5));
    }
}