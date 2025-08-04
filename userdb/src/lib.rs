use std::sync::LazyLock;

use redb::{Database, ReadableTable as _, TableDefinition, WriteTransaction};
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

pub fn unbind_user(id: i64) -> Result<bool, redb::Error> {
    let txn = write_txn()?;
    let result;
    {
        let mut table = txn.open_table(QQ_USERID)?;
        result = table.remove(id)?.is_some();
    }

    txn.commit()?;
    Ok(result)
}

pub fn record_userid(id: i64, user_id: u32) -> Result<Option<u32>, redb::Error> {
    let txn = write_txn()?;
    {
        let mut table = txn.open_table(QQ_USERID)?;
        if let Some(uid) = table.get(id)? {
            return Ok(Some(uid.value()));
        }
        table.insert(id, user_id)?;
        info!("new record: {id} <=> {user_id}");
    }

    txn.commit()?;
    Ok(None)
}
