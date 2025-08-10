use std::{
    path::PathBuf,
    sync::{LazyLock, OnceLock},
};

use redb::{Database, ReadTransaction, ReadableDatabase as _, ReadableTable as _, TableDefinition, WriteTransaction};
use spdlog::{error, info};

const ID_USERID: TableDefinition<'_, i64, u32> = TableDefinition::new("userid");
const ID_DIVINGFISH: TableDefinition<'_, i64, String> = TableDefinition::new("divingfish");

pub static DATABASE_PATH: OnceLock<PathBuf> = OnceLock::new();

static DATABASE: LazyLock<Database> = LazyLock::new(|| {
    info!("initializing database...");
    let db = Database::create(
        DATABASE_PATH
            .get()
            .expect("you must set `DATABASE_PATH` before initializing database"),
    )
    .inspect_err(|e| error!("failed initializing db: {e}"))
    .unwrap();
    let write_txn = db.begin_write().unwrap();
    _ = write_txn.open_table(ID_USERID);
    _ = write_txn.open_table(ID_DIVINGFISH);
    _ = write_txn.commit();
    db
});

pub fn write_txn() -> Result<WriteTransaction, redb::Error> {
    Ok(DATABASE.begin_write()?)
}

pub fn read_txn() -> Result<ReadTransaction, redb::Error> {
    Ok(DATABASE.begin_read()?)
}

pub fn query_user(id: i64) -> Result<Option<u32>, redb::Error> {
    let txn = read_txn()?;
    {
        let table = txn.open_table(ID_USERID)?;
        Ok(table.get(id)?.map(|v| v.value()))
    }
}
pub fn query_user_df(id: i64) -> Result<Option<String>, redb::Error> {
    let txn = read_txn()?;
    {
        let table = txn.open_table(ID_DIVINGFISH)?;
        Ok(table.get(id)?.map(|v| v.value()))
    }
}

pub fn unbind_user(id: i64) -> Result<bool, redb::Error> {
    let txn = write_txn()?;
    let result;
    {
        let mut table = txn.open_table(ID_USERID)?;
        result = table.remove(id)?.is_some();
    }

    txn.commit()?;
    Ok(result)
}
pub fn unbind_user_df(id: i64) -> Result<bool, redb::Error> {
    let txn = write_txn()?;
    let result;
    {
        let mut table = txn.open_table(ID_DIVINGFISH)?;
        result = table.remove(id)?.is_some();
    }

    txn.commit()?;
    Ok(result)
}

pub async fn record_userid(id: i64, user_id: u32) -> Result<Option<u32>, redb::Error> {
    let txn = tokio::task::spawn_blocking(|| write_txn()).await.unwrap()?;
    {
        let mut table = txn.open_table(ID_USERID)?;
        if let Some(uid) = table.get(id)? {
            return Ok(Some(uid.value()));
        }
        table.insert(id, user_id)?;
        info!("new record: {id} <=> {user_id}");
    }

    txn.commit()?;
    Ok(None)
}

/// return: `bool`, if recorded return true
pub async fn record_df_token(id: i64, divingfish_token: &str) -> Result<bool, redb::Error> {
    let txn = tokio::task::spawn_blocking(|| write_txn()).await.unwrap()?;
    {
        let mut table = txn.open_table(ID_DIVINGFISH)?;
        if let Some(_) = table.get(id)? {
            return Ok(false);
        }
        table.insert(id, divingfish_token.to_string())?;
        info!("new record: {id} <=> {divingfish_token}");
    }

    txn.commit()?;
    Ok(true)
}
