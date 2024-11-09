use std::error::Error;

use diesel::connection::TransactionManager;
use diesel::r2d2::{ConnectionManager, PoolTransactionManager, PooledConnection};
use diesel::result::DatabaseErrorKind;
use diesel::SqliteConnection;

use crate::common::Result;
use crate::datasource::new_db;

pub fn is_not_found(err: &(dyn Error + 'static)) -> bool {
    matches!(
        err.downcast_ref::<diesel::result::Error>(),
        Some(diesel::NotFound)
    )
}

pub fn is_duplicate(err: &(dyn Error + 'static)) -> bool {
    match err.downcast_ref::<diesel::result::Error>() {
        Some(diesel::result::Error::DatabaseError(db_error_kind, _)) => {
            matches!(db_error_kind, DatabaseErrorKind::UniqueViolation)
        }
        _ => false,
    }
}

pub struct TM {
    conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    commit_flag: bool,
}

impl TM {
    pub fn commit(&mut self) -> Result<()> {
        PoolTransactionManager::commit_transaction(&mut self.conn)?;
        self.commit_flag = true;
        Ok(())
    }

    pub fn conn(&mut self) -> &mut PooledConnection<ConnectionManager<SqliteConnection>> {
        &mut self.conn
    }
}

impl Drop for TM {
    fn drop(&mut self) {
        if !self.commit_flag {
            PoolTransactionManager::rollback_transaction(&mut self.conn)
                .expect("Failed to rollback transaction.")
        }
    }
}

pub fn new_transaction() -> Result<TM> {
    let mut tm = TM {
        conn: new_db(),
        commit_flag: false,
    };
    PoolTransactionManager::begin_transaction(&mut tm.conn)?;
    Ok(tm)
}
