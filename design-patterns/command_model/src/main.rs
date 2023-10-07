use common::split_line;

pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
}


fn trait_objcet() {
    split_line();
    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);
    let migration = schema.execute();
    let rollback = schema.rollback();
    println!("Migration: {:?}", migration);
    println!("Rollback: {:?}", rollback);
    split_line();
}

type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct SchemaFn {
    commands: Vec<Command>,
}

impl SchemaFn {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback});
    }

    fn execute(&self) -> Vec<String>{
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

fn fn_object() {
    split_line();
    let mut schema = SchemaFn::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field, remove_field);
    let migration = schema.execute();
    let rollback = schema.rollback();
    println!("Migration: {:?}", migration);
    println!("Rollback: {:?}", rollback);
    split_line();
}

type MigrationFnT<'a> = Box<dyn Fn() -> &'a str>;
struct SchemaFnT<'a> {
    executes: Vec<MigrationFnT<'a>>,
    rollbacks: Vec<MigrationFnT<'a>>,
}

impl<'a> SchemaFnT<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }

    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }

    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field_FT() -> &'static str {
    "Add field"
}

fn remove_field_FT() -> &'static str {
    "Remove field"
}

fn fn_trait_object() {
    split_line();
    let mut schema = SchemaFnT::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(add_field_FT, remove_field_FT);
    let migration = schema.execute();
    let rollback = schema.rollback();
    println!("Migration: {:?}", migration);
    println!("Rollback: {:?}", rollback);
    split_line();
}

fn main() {
    trait_objcet();
    fn_object();
    fn_trait_object();
}
