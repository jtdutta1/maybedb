use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Datatype {
    Integer,
    Float,
    Varchar(usize),
    Text    
}

impl Datatype {
    pub fn parse_type(string_type: &String) -> Result<Datatype, String> {
        if string_type.eq_ignore_ascii_case("INTEGER") {
            Ok(Datatype::Integer)
        } else if string_type.eq_ignore_ascii_case("FLOAT") {
            Ok(Datatype::Float)
        } else if string_type.eq_ignore_ascii_case("TEXT") {
            Ok(Datatype::Text)
        } else if let Some(range) = Self::parse_varchar(&string_type.as_str()) {
            if range < 256 {
                Ok(Datatype::Varchar(range))
            } else {
                Err(format!("ERROR:\nRange for VARCHAR too large!\nExpected <255. Found {range}."))
            }
        } else {
            Err(format!("ERROR:\nType {string_type} is not supported!"))
        }
    }

    pub fn parse_varchar(string_type: &str) -> Option<usize> {
        let re = Regex::new(r"VARCHAR\((\d+)\)").unwrap();
        if re.is_match(string_type) {
            let Some(capture) = re.captures(string_type) else {return None};
            match capture[1].parse() {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn type_to_string(&self) -> String {
        if *self == Datatype::Integer {
            "Integer".to_string()
        } else if *self == Datatype::Float {
            "Float".to_string()
        } else if *self == Datatype::Text {
            "Text".to_string()
        } else {
            "Varchar".to_string()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Varchar(String, usize),
    Text(String),
    None
}

impl Value {
    pub fn parse(value: String, col_type: &Datatype) -> Result<Value, String> {
        if Datatype::Integer == *col_type {
            Ok(Value::Integer(
                value
                .as_str()
                .trim()
                .parse::<i32>()
                .expect("ERROR:\nCouldn't parse {value} into type INTEGER.")))
        } else if Datatype::Float == *col_type {
            Ok(Value::Float(
                value
                .as_str()
                .trim()
                .parse::<f64>()
                .expect("ERROR:\nCouldn't parse {value} into type FLOAT.")))
        } else if Datatype::Text == *col_type {
            Ok(Value::Text(value))
        } else if let Datatype::Varchar(max_len) = *col_type {
            if value.len() <= max_len {
                Ok(Value::Varchar(value, max_len))
            } else {
                Err(format!("ERROR:\nValue length exceeds specified VARCHAR length of {max_len}"))
            }
        } else {
            Err(format!("ERROR:\nUnsupported datatype!"))
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct ColumnDef {
    name: String,
    col_type: Datatype,
    // nullable: bool,
}

impl ColumnDef {
    pub fn build(
        name: String,
        col_type: Datatype
        // nullable: bool
    ) -> ColumnDef {
        ColumnDef {name, col_type}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Row {
    pub values: Vec<Vec<Value>>,
}

impl Row {
    pub fn new() -> Row {
        let new_values = Vec::new();
        Row {values: new_values}
    }
    pub fn build(values: Vec<Value>) -> Row {
        let mut new_values = Vec::new();
        new_values.push(values);
        Row {values: new_values}
    }

    pub fn add_values(&mut self, values: Vec<Value>) {
        self.values.push(values);
    }
}

#[derive(PartialEq, Debug)]
pub struct Table {
    pub cols: Vec<ColumnDef>,
    pub records: Option<Row>,
    pub name: String,
}

impl Table {
    pub fn build(table_name: String, col_defs: Vec<ColumnDef>) -> Table {
        Table {cols: col_defs, records: None, name: table_name}
    }

    pub fn insert_data(&mut self, values: Vec<String>) -> Result<(), String> {
        if self.values_match_types(&values) {
            let parsed_values = values.into_iter()
                .zip(self.cols.iter())
                .map(|(x, col_def)| Value::parse(x, &col_def.col_type))
                .collect::<Result<Vec<Value>, String>>()?;
            if self.records.is_none() {
                self.records = Some(Row::build(parsed_values));
                Ok(())
            } else {
                self.records.as_mut().unwrap().add_values(parsed_values);
                Ok(())
            }
        } else {
            Err(format!("ERROR:\nValue types mismatched!"))
        }
    }

    pub fn values_match_types(&self, values: &Vec<String>) -> bool {
        for (idx, value) in values.iter().enumerate() {
            let supposed_datatype = &self.cols[idx].col_type;
            if *supposed_datatype == Datatype::Integer {
                if value.as_str().trim().parse::<i32>().is_ok() {
                    continue;
                } else {
                    return false;
                }
            } else if *supposed_datatype == Datatype::Float {
                if value.as_str().trim().parse::<f64>().is_ok() {
                    continue;
                } else {
                    return false;
                }
            } else{
                continue;
            }
        }
            true
    }

    // pub fn get_rows(&self, constraint: &Vec<String>) -> Result<Option<Vec<Row>>, String> {

    // }
}

#[derive(PartialEq)]
pub struct Database {
    pub tables: Vec<Table>,
    pub name: String,
}

impl Database {
    pub fn build(name: String) -> Database {
        Database { tables: Vec::new(), name }
    }

    pub fn add_table_to_database(&mut self, table: Table) -> Result<(), String> {
        if !self.table_exists(&table.name) {
            self.tables.push(table);
            Ok(())
        } else {
            Err(format!("ERROR:\nTable with name {} already exists in the database {}.", &table.name, &self.name))
        }
    }

    pub fn delete_table_from_database(&mut self, table_name: &String) -> Result<(), String> {
        if self.table_exists(table_name) {
            let _ = self.tables.remove(self.tables.iter().position(|x| &x.name == table_name).unwrap());
            Ok(())
        } else {
            Err(format!("ERROR:\nTable with name {} does not exist in the database {}.", table_name, &self.name))
        }
    }

    pub fn table_exists(&self, table_name: &String) -> bool {
        if self.tables.iter().any(|x| &x.name == table_name) {
            true
        } else {
            false
        }
    }

    pub fn list_tables(&self) -> Option<Vec<String>> {
        if self.tables.len() > 0 {
            let table_names = self.tables.iter().map(|x| x.name.clone()).collect();
            Some(table_names)
        } else {
            None
        }
    }
}


pub struct DatabaseRegistry {
    pub databases: Vec<Database>,
}

pub struct DatabaseConfig {
    pub registry_path: String,
}

impl DatabaseRegistry {
    pub fn new() -> DatabaseRegistry{
        DatabaseRegistry{databases: Vec::new()}
    }

    pub fn add_db_to_registry(&mut self, database: Database) -> Result<(), String> {
        if self.database_exists(&database.name) {
            Err(format!("ERROR:\nDatabase with name {} already exists.\nConsider changing the name.", database.name))
        } else {
            self.databases.push(database);
            Ok(())
        }
    }

    pub fn get_database(&self, db_name: &String) -> Option<&Database> {
        if self.database_exists(db_name) {
            let db = self.databases.iter().find(|x| &x.name == db_name).unwrap();
            Some(db)
        } else {
            None
        }
    }

    pub fn delete_db_from_registry(&mut self, db_name: &String) -> Result<(), String> {
        if self.database_exists(db_name) {
            self.databases.remove(
                self.databases.iter().position(|x| &x.name == db_name).unwrap()
            );
            Ok(())
        } else {
            Err(format!("ERROR:\nDatabase with name {db_name} does not exist!"))
        }
    }

    pub fn database_exists(&self, db_name: &String) -> bool {
        if self.databases.iter().any(|x| &x.name == db_name) {
            true
        } else {
            false
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let mut db = Database::build("test".to_string());

        let col1 = ColumnDef::build(
            "name".to_string(),
            Datatype::Text
        );
        let col2 = ColumnDef::build(
            "age".to_string(),
            Datatype::Integer
        );

        // create a tables
        let table1 = Table::build(
            "test".to_string(),
            vec!(col1.clone(), col2.clone())
        );
        let table2 = Table::build(
            "test".to_string(), 
            vec!(col1, col2)
        );
        let _ = db.add_table_to_database(table1);

        let result2 = db.add_table_to_database(table2);
        
        assert_eq!(result2, Err(String::from("ERROR:\nTable with name test already exists in the database test.")));
    }

    #[test]
    fn test_database_registry() {
        let mut db_reg = DatabaseRegistry::new();
        
        // create a database
        let db1 = Database::build("test".to_string());
        let db2 = Database::build("test2".to_string());
        let db3 = Database::build("test".to_string());

        assert_eq!(db_reg.add_db_to_registry(db1), Ok(()));
        assert_eq!(db_reg.add_db_to_registry(db2), Ok(()));
        assert_eq!(
            db_reg.add_db_to_registry(db3), 
            Err(String::from(
                    "ERROR:\nDatabase with name test already exists.\nConsider changing the name."
                )
            )
        );
    }
}