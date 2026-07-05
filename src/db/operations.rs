use crate::db::types::{Datatype, ColumnDef, Table, Database, DatabaseRegistry};

// table ops
fn create_table(
    registry: &mut DatabaseRegistry,
    db_name: String,
    table_name: String,
    col_list: Vec<String>,
    col_types: Vec<String>
) -> Result<(), String> {
    // check if the database exists in the registry first
    if let Some(database) = registry.databases.iter_mut().find(|x| x.name == db_name) {
        // first parse the string types into their Datatype enum
        let parse_types: Vec<Datatype> = col_types.iter()
            .map(
                |x| Datatype::parse_type(x)
            )
            .collect::<Result<Vec<Datatype>, String>>()?;

        // create vector of column defs for each column
        let col_defs: Vec<ColumnDef> = col_list.into_iter()
            .zip(parse_types.into_iter())
            .map(|(n, t)| ColumnDef::build(n, t))
            .collect();

        // create the table with the columns
        let new_table = Table::build(
            table_name, 
            col_defs
        );

        // add table to database
        database.add_table_to_database(new_table)?;
        Ok(())
    } else {
        Err(format!("ERROR:\nDatabase with name {db_name} does not exist!"))
    }
}

fn insert_into_table(
    registry: &mut DatabaseRegistry,
    db_name: String, 
    table_name: String,
    value_list: Vec<String>
) -> Result<(), String> {
    // check if the database exists
    if let Some(database) = registry.databases.iter_mut().find(|x| x.name == db_name) {
        // check if the table in the database exists
        if let Some(table) = database.tables.iter_mut().find(|x| x.name == table_name) {
            // insert data into table
            table.insert_data(value_list)?;
            Ok(())
        } else {
            Err(format!("ERROR:\nTable with name {table_name} does not exist in the database {db_name}."))
        }
    } else {
        Err(format!("ERROR:\nDatabase with name {db_name} does not exist!"))
    }
}

// fn select_from_table(
//     registry: &mut DatabaseRegistry,
//     db_name: String,
//     table_name: String,
//     cols: String,
//     conditions: Vec<Condition>
// )

// database ops
fn create_db(
    registry: &mut DatabaseRegistry,
    db_name: String
) -> Result<(), String> {
    if registry.database_exists(&db_name) {
        Err(format!("ERROR:\nDatabase with name {db_name} already exists!"))
    } else {
        let db = Database::build(db_name);
        registry.add_db_to_registry(db)?;
        Ok(())
    }
}