# Grammar
We define the grammar surrounding the SQL operation here so the compiler can create the **A**bstract **S**yntax **T**ree or **AST** from the expressions. The **AST** can then be converted into a series of method calls for the table and databases involved in the expression. 

## Symbols
We define a set of acceptable symbols for the SQL language. For simplicity, we will stick to the ASCII 8-bit format and extend this rule for other unicode characters post creation of the compiler, to support other symbols.

Starting with terminal symbols, we will have:-

$T = \{a-z, A-Z, 0-9\}$

## Expression
The user provided query is called an expression. It contains all the defined operations that needs to be performed in the relational database. An expression is a series of operations. And operation contains an operator and the operands supported by the operator. An operator can also have additional supporting operators that enhances/supports its operation. 

Operators can also be containers like `"`, `'` and `(` `)`. They can contain multiple operations and or operands. 

We then define 2 types of operators -- main operator and supporting operator. The main operator might or might not be followed by a supporting operator. Following is a list of main operators and their supporting operators

| Main Operator | Supporting Operator(s) |
| ------------: | :------------------ |
| `CREATE`      | `TABLE`, `DATABASE` |
| `ALTER`       | `TABLE`             |
| `DROP`        | `TABLE`, `DATABASE` |
| `TRUNCATE`    | `TABLE`             |
| `INSERT`      | `INTO`              |
| `SELECT`      | `FROM`              |
| `WHERE`       | ❌                  |
| `VALUES`      | ❌                  |


Here ❌ indicates that these statements are not main, neither supporting. They are statements no doubt but cannot exist on their own, nor are required. These would be conditions <s>and constraints</s>. 

__NOTE__: For now this list is limited due to development constraints and will be expanded in the future to support the full set of operators.

## Tokenization
Here we define the algorithm to tokenize the expression.

1. Whitespaces and commas (`\b`, `\n` and `,`) are delimiters for terms in an expression, unless superseded by an unclosed container (`"`, `'` or `(`).
2. Anything contained inside a pair of `"` or `'` are part of a string literal. 
3. `(` and `)` are containers that contain other terms/expressions.
4. `"` or `'` have to be superseded by a term delimiter defined in rule `1`.
5. All expressions must be terminated by a `;`.
6. Valid allowed punctuations outside of string containers are the term delimiters, conditional delimiters (`>`, `<`, `=` and `!`), and `_` and `*` as part of valid identifiers and standalone respectively.
7. All containers must end in a valid expression. 

With the above rules, following are some expressions and the result of tokenizing them.

Example 1:

    SELECT column1, column2, column3 FROM table_name;

    "SELECT", "column1", "column2", "column3", "FROM", "table_name"

Example 2: 

    SELECT"12 * FROM table_name;

    ERROR!
    Found a " without an empty space, newline or comma before it in the query!

Example 3:

    SELECT "12 * FROM table_name;

    ERROR!
    Non terminated string

Example 4:

    SELECT * FROM table_name WHERE column1 < 3;

    "SELECT", "*", "FROM", "table_name", "WHERE", "column1", "<", "3"

Example 5:

    SELECT ADD(column1) FROM table_name WHERE column1 < 3;

    

## `CREATE` operator
`CREATE` operator should always be followed by either of its supporting operator and an identifier indicator the name of the entity to be created.

    CREATE (TABLE or DATABASE) identifier;