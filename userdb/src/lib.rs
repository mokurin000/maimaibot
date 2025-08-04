use std::sync::LazyLock;

use redb::{Database, TableDefinition, WriteTransaction};
use spdlog::{error, info};

const QQ_USERID: TableDefinition<'_, i64, u32> = TableDefinition::new("userid");

static DATABASE: LazyLock<Database> = LazyLock::new(|| {
    info!("initializing database...");
    let db = Database::create("userdata.db")
        .inspect_err(|e| error!("failed initializing db: {e}"))
        .unwrap();
    let write_txn = db.begin_write().unwrap();
    _ = write_txn.open_table(QQ_USERID);
    db
});

pub fn write_txn() -> Result<WriteTransaction, redb::Error> {
    Ok(DATABASE.begin_write()?)
}

pub fn record_userid(id: i64, user_id: u32) -> Result<(), redb::Error> {
    let txn = write_txn()?;
    {
        let mut table = txn.open_table(QQ_USERID)?;
        table.insert(id, user_id)?;
    }

    txn.commit()?;
    Ok(())
}
