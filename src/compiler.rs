use constcat::concat_slices;

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
const LOGICAL_OPERATOR: &[&str] = &[
    ">", "<", "=", ">=", "<=", "!=", "AND", "OR", "NOT"
];

const CONTAINER_OPERATOR: &[&str] = &[
    "(", ")"
];

const ARITHMETIC_OPERATOR: &[&str] = &[
    "ADD(", "SUB(", "MUL(", "DIV("
];

const MISC_OPERATOR: &[&str] = &[
    "*"
];

const OPERATORS: &[&str] = concat_slices!(
    [&str]: LOGICAL_OPERATOR, CONTAINER_OPERATOR, ARITHMETIC_OPERATOR, MISC_OPERATOR
);