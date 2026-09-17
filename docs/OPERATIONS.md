# Operations
A basic Relational Database should be able to create and manage individual databases and different tables inside them. That means **CREATION**, **UPDATION**, **ALTERATION** and **DELETION** should be the basic operations. Furthermore these operations can be **CONDITIONAL** and support **INFERRING** from its own data. Examples will be provided on these topics. Addtionally, the main operation which is **DATA STORAGE** and **RETRIEVAL** should be supported. 

We can condense these operations for database/table creation, alteration or deletion into the following keywords that will help the compiler translate the script into the required operations.

#### Database/Table Operations:-
- [`CREATE`](#create) - Create a table or database. Can be specified which one using the keyword `TABLE` or `DATABASE` for table or database respectively. E.g. `CREATE TABLE hello_world <operation>`.
- [`ALTER`](#alter) - Change an attribute of a table, like its name. E.g. `ALTER TABLE table_name <operation>`.
- [`DROP`](#drop) - Used to delete a table or a database using the same differentiating keywords as the above commands. E.g. `DROP TABLE table_name <constraints>`.
- [`TRUNCATE`](#truncate) - Used to remove all the data inside a table, without deleting the structure and constraints, unlike `DROP`. E.g. `TRUNCATE TABLE table_name`. 

Following is the data insertions, retrieval and updation related commands for a table.
#### Data Operations:-
- [`INSERT INTO ...`](#insert) - Add data to table. E.g. `INSERT INTO table_name <operation>`. 
- [`SELECT ... FROM ...`](#select) - Retrieve data from table. E.g. `SELECT column1, column2, ... FROM table_name <constraints>`.
- <s>[`UPDATE ... SET ...`](#update) - Update existing values in a table. E.g. `UPDATE table_name SET column1=new_value1, ... <constraint>`.</s>

__IMPORTANT__: All SQL statements must be terminated by a `;` else the compiler will not know when to compile the provided script or may lead to trailing independent commands getting added to the previous command.

### CREATE
The `CREATE` command will be used to create new tables and databases. For databases the syntax will simply be

    CREATE DATABASE database_name;

For tables, however, we will have additional syntax.

    CREATE TABLE table_name (
        column1 datatype constraint,
        column2 datatype constraint,
        ...
    );

Here the `datatype` is important as the compiler needs to know the type of data to store, so it can be used for other operations (like add/subtract/append) correctly, depending on the type. Like the operation `ADD` can only be used with datatype `INTEGER` or `DECIMAL` values only. Thus missing the `datatype` during column definition, will throw an error. 

<s>The `constraint` is a special statement that let's the compiler know how the column is to be treated while storing, retrieving or updating the table. The constraints can be modified post table creation using the `ALTER` command (more on that later). The `constraint` are optional and can be skipped during table creation. They are defined in the [constraint](#constraints) section.</s>


It is also possible to create a table from an existing table (with its column constraints) in the following way

    CREATE TABLE table_name
    SELECT column1, column2, ...
    FROM other_table_name
    WHERE conditions

Here the constraints for the columns from the table `other_table_name` comes into play saving the trouble for the user to define them again.

### ALTER
The `ALTER` command is used to change certain aspects of a table, like its name, column names and datatype <s>and constraints</s> for a column.

    ALTER TABLE table_name alteration_expression;

The `alteration_expression` is the defining aspect for alteration. It includes:-
- Changing table name: `ALTER TABLE table_name RENAME TO new_table_name;`
- Changing column name: `ALTER TABLE table_name RENAME COLUMN column1 TO new_column1;`
- Adding new columns: `ALTER TABLE table_name ADD column_name datatype;`
- Deleting columns: `ALTER TABLE table_name DROP column_name;`
- Change datatype: `ALTER TABLE table_name MODIFY column_name new_datatype constraint;`
- <s>Add constraint: `ALTER TABLE table_name ADD CONSTRAINT constraint;`</s>

__NOTICE__ unlike the `CREATE` clause, there is no inferring the alteration from another table

### DROP
The `DROP` clause simply deletes tables and databases.

    DROP DATABASE database_name;
    DROP TABLE table_name;

Following also works but not yet supported <s>`DROP TABLE IF EXISTS table_name;`</s>.

### TRUNCATE
Truncate is for tables only, for when you want to remove all the data in a table, without deleting the table itself. This is uselful for periodic table refreshes.

    TRUNCATE TABLE table_name;

### SELECT
The `SELECT` is used to read data from tables. A simple way to read all the data inside a table is

    SELECT * FROM table_name;

or if you only want specific data (based on [conditions](#conditions))

    SELECT * FROM table_name WHERE conditions

or if you want data from specific columns

    SELECT column1, column2, ... FROM table_name;

### INSERT
The `INSERT` clause is used to insert data into a table. You can insert a row of value like

    INSERT INTO table_name (column1, column2, ...)
    VALUES (value1, value2, ...);

or a group of values like 

    INSERT INTO table_name (column1, column2, ...)
    VALUES 
    (value1_1, value1_2, ...),
    (value2_1, value2_2, ...),
    ...;

or if you want to insert values to all the columns, you can skip the column names altogether

    INSERT INTO table_name
    VALUES (value1, value2, ...);

You can also select values from another table and insert it into the table like

    INSERT INTO table_name
    SELECT column1, column2, ...
    FROM other_table_name
    WHERE conditions

or if you want to select all the columns from the `other_table_name`, you can skip the names of the columns and replace it with an `*`. However, the columns in the `other_table_name` should match the columns' datatype and their constraints in the `table_name`.

### Conditions
Conditions or conditional statements are used to perform operations selectively. Conditional statements are highlighted using the `WHERE` clause and logical operators like `>`, `<`, `=`, `>=`, `<=` and `!=`. <s>There are other operators for pattern matching like `BETWEEN`, `LIKE` and `IN`.</s> The definitions are:-
- `>`: greater than. Checks if the left operand is greater than the right operand
- `<`: less than. Checks if the left operand is lesser than the right operand
- `=`: equality. Check if the left and right operands are equal in value
- `>=`: greater than equal to. Check if the left operand is greater than equal to the right operand
- `<=`: lesser than equal to. Check if the left operand is less than equal to the right operand
- `!=`: not equal to. Checks if the left and right operands are not equal to value

Before defining the meaning of the pattern matching operators, we should also learn about some other logical operators. These are used to chain multiple conditional statements or flip the result of a conditional statement. They are `AND`, `OR` and `NOT`.
- `AND`: groups multiple conditions together. Results in a true if and only if all the conditions in the chain result to true. E.g. `... WHERE condition1 AND condition2 AND condition3 ...`
- `OR`: groups multiple conditions together. Results in a false if and only if all the conditions in the chain result to false. E.g. `... WHERE condition1 OR condition2 OR condition3 ...`
- `NOT`: flips the result of a condition. `... WHERE NOT 1 = 2` will result in true.

<s>
Now we define the pattern matching operations
- `BETWEEN`: results in true when a value is in a range defined by 2 values. 
- `LIKE`: results in true if a string matches a pattern defined by `LIKE`
- `IN`: results in true if a string contains a substring
</s>

### Constraints
__NOTE__: For now, constraints have been discarded from development.

<s>Constraints are aspects of the columns defined during table creation or alterations. They include the following properties:-
- `NULL`(default): setting this contraint allows empty values to be inserted to a column. This is the default value for a constraint if nothing is specified.
- `NOT NULL`: setting this constraint will lead to errors during data insertion if no data is provided for the column
- `UNIQUE`: setting this will lead to the column carrying unique values only. Leads to errors if duplicate values are attempted to be inserted
</s>