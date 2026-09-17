use std::env;

use maybedb::compiler::lexer::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // check if it has a second argument
    if let Some(query) = args.get(1) {
        let tokenized_query = tokenize(query).expect("Coudln't tokenize!\n");
        println!("{tokenized_query:?}");
    } else {
        println!("No query received! Please pass in a query as an argument!\nExample: ./maybedb \"SELECT * FROM table_name;\"");
    }
}
