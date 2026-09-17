// use std::vec::IntoIter;

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