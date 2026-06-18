use rusqlite::Connection;

use crate::db::schema::{CREATE_SCHEMA, CURRENT_VERSION};
use crate::errors::Result;

pub fn run(conn: &Connection) -> Result<()> {
    let version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version < CURRENT_VERSION {
        conn.execute_batch(CREATE_SCHEMA)?;
        conn.pragma_update(None, "user_version", CURRENT_VERSION)?;
    }
    Ok(())
}
