fn is_operation_delimiter(value: &char) -> bool {
    if *value == ' ' || *value == '\n' || *value == ',' {
        true
    } else {use std::vec::IntoIter;

// use crate::compiler::{KEYWORDS, OPERATORS};
// use crate::compiler::tokenizer::Token;

// pub enum GranularToken{
//     Begin,
//     End,
//     CreateTable,
//     CreateDatabase,
//     AlterTable,
//     AlterDatabase,
//     DropTable,
//     DropDatabase,
//     TruncateTable,
//     InsertInto,
//     SelectFrom,
//     ValuesWithSubtree,
//     Values(Vec<GranularToken>),
//     WhereCondition,
//     AndOperator,
//     OrOperator,
//     NotOperator,
//     AddOperator,
//     SubtractOperator,
//     MultiplyOperator,
//     DivideOperator,
//     StringLiteral(String),
//     IntegerLiteral(i32),
//     DecimalLiteral(f64),
//     Identifier(String)
// }


// pub struct Node {
//     token: Box<GranularToken>,
//     left: Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

// struct AST {
//     root: Box<Node>
// }

// impl AST {
//     pub fn new() -> AST {
//         let begin_node = Node {
//             token: Box::new(GranularToken::Begin),
//             left: None,
//             right: None
//         };
//         AST{root: Box::new(begin_node)}
//     }

//     pub fn parse_tokens_into_ast(&mut self, tokens: Vec<Token>) -> Result<(), String> {
//         let token_vector_iterator = tokens.into_iter();
//         let mut node = self.root;
//         let (mut insert_flag, mut select_flag) = (false, false);
//         while let token = token_vector_iterator.next().unwrap() {
//             match token {
//                 Token::Keyword(value) => {
//                     match value.as_str() {
//                         "CREATE" => {
//                             let next_token = token_vector_iterator
//                                                     .next()
//                                                     .expect("ERROR:\nExpected to find more after CREATE");
//                             let mut new_node : Node;
//                             if next_token == Token::Keyword("TABLE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::CreateTable),
//                                     left: None,
//                                     right: None,
//                                 };
//                             } 
//                             else if next_token == Token::Keyword("DATABASE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::CreateDatabase),
//                                     left: None,
//                                     right: None,
//                                 };
//                             }
//                             else {
//                                 return Err(
//                                     "ERROR:\nCREATE should be followed by a TABLE or a DATABASE keyword."
//                                     .to_string()
//                                 );
//                             }
//                             node.right = Some(Box::new(new_node));
//                         },
//                         "ALTER" => {
//                             let next_token = token_vector_iterator
//                                                     .next()
//                                                     .expect("ERROR:\nExpected to find more after ALTER");
//                             let mut new_node: Node;
//                             if next_token == Token::Keyword("TABLE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::AlterTable),
//                                     left: None,
//                                     right: None,
//                                 };
//                             } 
//                             else if next_token == Token::Keyword("DATABASE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::AlterDatabase),
//                                     left: None,
//                                     right: None,
//                                 };
//                             }
//                             else {
//                                 return Err(
//                                     "ERROR:\nALTER should be followed by a TABLE or a DATABASE keyword."
//                                     .to_string()
//                                 );
//                             }
//                             node.right = Some(Box::new(new_node));
//                         },
//                         "DROP" => {
//                             let next_token = token_vector_iterator
//                                                     .next()
//                                                     .expect("ERROR:\nExpected to find more after DROP");
//                             let mut new_node: Node;
//                             if next_token == Token::Keyword("TABLE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::DropTable),
//                                     left: None,
//                                     right: None,
//                                 };
//                             } 
//                             else if next_token == Token::Keyword("DATABASE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::DropDatabase),
//                                     left: None,
//                                     right: None,
//                                 };
//                             }
//                             else {
//                                 return Err(
//                                     "ERROR:\nDROP should be followed by a TABLE or a DATABASE keyword."
//                                     .to_string()
//                                 );
//                             }
//                             node.right = Some(Box::new(new_node));
//                         },
//                         "TRUNCATE" => {
//                             let next_token = token_vector_iterator
//                                                     .next()
//                                                     .expect("ERROR:\nExpected to find more after TRUNCATE");
//                             let mut new_node: Node;
//                             if next_token == Token::Keyword("TABLE".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::TruncateTable),
//                                     left: None,
//                                     right: None,
//                                 };
//                             } 
//                             else {
//                                 return Err(
//                                     "ERROR:\nTRUNCATE should be followed by a TABLE or a DATABASE keyword."
//                                     .to_string()
//                                 );
//                             }
//                             node.right = Some(Box::new(new_node));
//                         },
//                         "INSERT" => {
//                             insert_flag = true;
//                             let next_token = token_vector_iterator
//                                                     .next()
//                                                     .expect("ERROR:\nExpected to find more after INSERT");
//                             let mut new_node: Node;
//                             if next_token == Token::Keyword("INTO".to_string()) {
//                                 new_node = Node {
//                                     token: Box::new(GranularToken::InsertInto),
//                                     left: None,
//                                     right: None
//                                 };
//                             } else {
//                                 return Err(
//                                     "ERROR:\nINSERT should be followed by INTO!"
//                                     .to_string()
//                                 );
//                             }
//                             node.right = Some(Box::new(new_node));
//                         },
//                         "VALUES" => {
//                             let values = Self::parse_values(&token_vector_iterator)?;
//                         }
                    
//                     }
//                 },
//             }
//         }
//         Ok()
//     }

//     fn parse_values(token_vector_iterator: &IntoIter<Token>) -> Result<Node, String> {

//     }

//     fn create_value_subtree(root: &mut Node, token: Token) -> Result<Node, String> {
//         if token
//     }
// }
        false
    }
}

fn is_expression_delimiter(value: &char) -> bool {
    if *value == ';' {
        true
    } else {
        false
    }
}

fn is_conditional_operand(value: &char) -> bool {
    if *value == '>' || *value == '<' || *value == '=' || *value == '!' {
        true
    } else {
        false
    }
}

fn is_string_container(value: &char) -> bool {
    if *value == '\'' || *value == '"' {
        true
    } else {
        false
    }
}

fn is_paren_container(value: &char) -> bool {
    if *value == '(' || *value == ')' {
        true
    } else {
        false
    }
}

fn is_other_allowed_delimiter(value: &char) -> bool {
    if *value == '_' || *value == '*' {
        true
    } else {
        false
    }
}

pub fn tokenize(expression: &String) -> Result<Vec<String>, String> {
    /* Tokenizing SQL expression relies on the following order of precedence. 
    First we check if a character is a trailing container. A container will be either of the following:-
    - Single quote '
    - Double quotes "
    - Parenthesis ()
    Symbols within them are to be treated differently. 
    For single and double quotes, everything inside 2 pair of them are to be treated as a single unit
    For parenthesis, everything is to be treated normally but the parenthesis is to be treated as a dilimiter.
    
    Then there are delimiters in the following precedence:-
    - Semicolon (;) is always used as an expression ender. There should only be 1 semicolon per expression
    - Periods (.) are dilimiters for polymorphism and should not have any other dilimiters
    - Whitespaces (\b or \n) and commas (,) and comma-whitespaces (,\b or ,\n or ,\b\n) are treated as 
    the lowest level delimiters that serve the function of token spacing
    
    Finally there are operators and operand. In SQL operators can be entire word or words along with symbols as well.
    Logical operators like >, <, =, >=, <= and != are also counted among them and are not to be treated as 
    delimiters
        */
    let mut final_tokens: Vec<String> = vec!();
    let mut temp_token = String::new();
    let mut container_stack: Vec<char> = vec!();
    let mut is_in_string_container = false;
    let mut is_in_paren_container = false;
    // scan the expression one character at a time
    for (idx, ch) in expression.chars().enumerate() {
        // println!("Checking {ch}");
        if ch.is_ascii_whitespace() {
            // if character is an ascii whitespace and is not part of a string container
            // push everything in temp_token to final_token
            if !is_operation_delimiter(&ch) { 
                return Err(String::from("ERROR!\nIllegal character found {ch}"));
            }
            if !is_in_string_container {
                if !temp_token.is_empty() {
                    final_tokens.push(temp_token);
                    temp_token = String::new();
                }
            } 
            // if whitespace part of string container then add it to temp_token and move on
            else {
                temp_token.push(ch);
            }
        }
        // check if character is a punctuation
        else if ch.is_ascii_punctuation() {
            // if it is a string contiainer symbol
            if is_string_container(&ch) {
                // check if a string container flag is set
                if is_in_string_container {
                    temp_token.push(ch);
                    // check if the string marker is also in the container stack
                    // and is the exact same marker (single quotes and double quotes must match
                    // only with themself)
                    if !container_stack.is_empty() &&
                        container_stack.last() == Some(&ch) {
                        // pop the container stack and prepend the last string marker
                        // to the beginning of the temp_token
                        let temp_char = container_stack.pop().unwrap();
                        temp_token.insert(0, temp_char);
                        final_tokens.push(temp_token);
                        temp_token = String::new();
                        is_in_string_container = false;
                    } 
                    // else the markers don't match since is_in_string_container flag
                    // cannot be true without a marker already within the container stack
                }
                // else it is a beginning of a string container
                else {
                    // ensure that the temp_token is empty
                    if temp_token.is_empty() {
                        is_in_string_container = true;
                        container_stack.push(ch);
                    } else {
                        return 
                        Err(format!("ERROR!\nFound a {ch} without an empty space, newline or comma before it in the query!"));
                    }
                }
            }
            // else check if it is a parenthesis container
            else if is_paren_container(&ch) {
                // check if a string container flag is set
                if is_in_string_container {
                    // add it to temp_token and move on
                    temp_token.push(ch);
                }
                // else check if a parenthesis container flag is set
                else if is_in_paren_container {
                    // check if the opening parenthesis marker is also in the container stack
                    if !container_stack.is_empty() && 
                        container_stack.last() == Some(&'(') {
                        // check if the character is a closing parenthesis
                        if ch == ')' {
                            // push everything in temp_token (if exists) to final tokens
                            if !temp_token.is_empty() {
                                final_tokens.push(temp_token);
                                temp_token = String::new();
                            }
                            // pop the last "(" from the container stack
                            if container_stack.contains(&'(') {
                                let end_open_paren_index = container_stack
                                    .iter()
                                    .rposition(|&c| c == '(')
                                    .unwrap();
                                container_stack.remove(end_open_paren_index);
                            }
                            // push closing parenthesis to the final_tokens as well
                            final_tokens.push(ch.to_string());
                            // check if the container stack contains opening
                            // parenthesis. If not then disable the flag
                            if !container_stack.contains(&'(') {
                                is_in_paren_container = false;
                            }
                        }
                        // if it is an opening parenthesis
                        else if ch == '(' {
                            // push everything in temp_token to final_tokens
                            if !temp_token.is_empty() {
                                // add opening parenthesis to temp_token 
                                // as it can indicate an operation
                                temp_token.push(ch.clone());
                                final_tokens.push(temp_token);
                                temp_token = String::new();
                            }
                            // add the parenthesis to container stack
                            container_stack.push(ch);
                        }
                    }
                    // else it is not a part of the parenthesis container stack
                    // and an error has occurred while tokenizing the expression
                    else {
                        return Err(format!("ERROR!\nCouldn't parse expression at index {idx} for character {ch}"));
                    }
                        
                }
                // else check if it is an opening parenthesis
                else if ch == '(' {
                    // add it to the container_stack and set the is_in_paren_container
                    // flag to true
                    container_stack.push(ch.clone());
                    is_in_paren_container = true;
                    // push everything in temp_token to final_tokens
                    if !temp_token.is_empty() {
                        // add opening parenthesis to temp_token 
                        // as it can indicate an operation
                        temp_token.push(ch);
                        final_tokens.push(temp_token);
                        temp_token = String::new();
                    }
                }
                // else check if it is a closing parenthesis
                else if ch == ')' {
                    // return error as a closing parenthesis
                    // cannot occur without an opening one
                    // except inside a string container
                    return Err(format!("ERROR!\nEncountered {ch} which cannot occure without a '('"));
                }
            }
            // else if it is a whitespace or comma
            else if is_operation_delimiter(&ch) {
                // push everything in temp_token (if not empty) to 
                // final_tokens if not in string container
                if !is_in_string_container && !temp_token.is_empty(){
                    final_tokens.push(temp_token);
                    temp_token = String::new();
                }
                // else if in a string container then add to temp_token
                else {
                    temp_token.push(ch);
                }
            }
            // else if it is a conditional operator
            else if is_conditional_operand(&ch) {
                // check if control is in string container
                if is_in_string_container {
                    temp_token.push(ch);
                }
                // else it is a conditional operation
                else {
                    // check if temp_token is empty or 
                    // it contains only conditional delimiters
                    if temp_token.is_empty() {
                        temp_token.push(ch);
                    } else if temp_token.chars().all(|c| is_conditional_operand(&c)) {
                        temp_token.push(ch);
                    }
                    // else temp_token contains string that 
                    // is not related to conditional operations 
                    else {
                        return Err(format!("ERROR!\nThere should be a blankspace between conditional operation and other tokens surrounding it!"));
                    }
                }
            }
            // check if it is an expression delimiter
            else if is_expression_delimiter(&ch) {
                // check if control not in string container
                if is_in_string_container {
                    temp_token.push(ch);
                }
                // else it is mark of an expression termination
                else {
                    // check if the index is the last index of the expression
                    if idx == expression.len() - 1 {
                        if !temp_token.is_empty() {
                            final_tokens.push(temp_token);
                        }
                        // println!("final_tokens: {final_tokens:?}");
                        // check if the parenthesis container has closed
                        if is_in_paren_container {
                            return Err(format!("ERROR!\nMissing closing parenthesis before expression termination"));
                        }
                        return Ok(final_tokens);
                    }
                    // else there exists more after the termination which
                    // is not parsable
                    else {
                        return Err(format!("ERROR!\nExpression terminates at index {idx} but contains more query after it!"))
                    }
                }
            }
            // check if it is an allowed delimiter
            else if is_other_allowed_delimiter(&ch) {
                temp_token.push(ch);
            }
        }
        // else it is a non delimited character
        else {
            temp_token.push(ch);
            // println!("temp_token = {temp_token}");
        }
        // println!("temp_token: {temp_token}");
    }
    if !temp_token.is_empty() {
        // check if the string container has closed
        if is_in_string_container {
            return Err(format!("ERROR!\nNon terminated string"));
        }
        return Err(String::from("ERROR!\nMissing ; as expression delimiter"));
    }
    Ok(final_tokens)
}



// fn parse_expression(expression: &String) -> Result<Vec<Token>, String> {
//     let tokens = Self::tokenize(expression)?;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_tokenization() {
        let expression1 = String::from("SELECT column1, column2, column3 FROM table_name;");
        let expression2 = String::from("SELECT\"12 * FROM table_name;");
        let expression3 = String::from("SELECT \"12 * FROM table_name;");
        let expression4 = String::from("SELECT * FROM table_name WHERE column1 < 3;");
        let expression5 = String::from("SELECT ADD(column1) FROM table_name WHERE column1 < 3;");
        let expression6  = String::from("SELECT ADD(column1) FROM table_name WHERE column1 <= 3;");

        let actual_tokens1: Result<Vec<String>, String> = Ok(vec!(
            "SELECT".to_string(), 
            "column1".to_string(), 
            "column2".to_string(), 
            "column3".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string()
        ));

        let actual_tokens2: Result<Vec<String>, String> = Err(String::from("ERROR!\nFound a \" without an empty space, newline or comma before it in the query!"));
        let actual_tokens3: Result<Vec<String>, String> = Err(String::from("ERROR!\nNon terminated string"));
        let actual_tokens4: Result<Vec<String>, String> = Ok(vec!(
            "SELECT".to_string(), 
            "*".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(), 
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<".to_string(), 
            "3".to_string()
        ));
        let actual_tokens5: Result<Vec<String>, String> = Ok(vec!(
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
        ));
        let actual_tokens6: Result<Vec<String>, String> = Ok(vec!(
            "SELECT".to_string(), 
            "ADD(".to_string(), 
            "column1".to_string(), 
            ")".to_string(), 
            "FROM".to_string(), 
            "table_name".to_string(), 
            "WHERE".to_string(), 
            "column1".to_string(), 
            "<=".to_string(), 
            "3".to_string()
        ));

        let predicted_tokens1 = tokenize(&expression1);
        let predicted_tokens2 = tokenize(&expression2);
        let predicted_tokens3 = tokenize(&expression3);
        let predicted_tokens4 = tokenize(&expression4);
        let predicted_tokens5 = tokenize(&expression5);
        let predicted_tokens6 = tokenize(&expression6);
        
        assert_eq!(predicted_tokens1, actual_tokens1);
        assert_eq!(predicted_tokens2, actual_tokens2);
        assert_eq!(predicted_tokens3, actual_tokens3);
        assert_eq!(predicted_tokens4, actual_tokens4);
        assert_eq!(predicted_tokens5, actual_tokens5);
        assert_eq!(predicted_tokens6, actual_tokens6);
    }
}