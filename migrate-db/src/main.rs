use std::{env::args, error::Error};

use redb::Database;

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = args().nth(1).expect("database path was missing!");
    Database::open(db_path)?.upgrade()?;
    Ok(())
}
