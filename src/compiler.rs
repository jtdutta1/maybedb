pub mod lexer;
pub mod tokenizer;
pub mod granular_tokenizer;

// const DB_TABLE_OPS: [&str; 4] = ["CREATE", "ALTER", "DROP", "TRUNCATE"];
const KEYWORDS: &[&str] = &[
    // table/db ops
    "CREATE", "ALTER", "DROP", "TRUNCATE",
    // special keywords
    "TABLE", "DATABASE",
    // data ops
    "INSERT", "INTO", "SELECT", "FROM", "VALUES",
    // conditional statements
    "WHERE",
    // miscellaneous
    // "AS"

];

// define operators
const OPERATORS: &[&str] = &[
    // logical
    ">", "<", "=", ">=", "<=", "!=", "AND", "OR", "NOT",
    // miscellaneous
    "*",
    // containers
    "(", ")", 
    // Arithmetic
    "ADD(", "SUB(", "MUL(", "DIV(",
];