use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct TableIndexScans {
    pub name: String,
    pub count: i64,
}

impl Query for TableIndexScans {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            count: row.try_get("count").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.count]
    }

    fn headers() -> prettytable::Row {
        row!["name", "count"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/table_index_scans.sql").to_string()
    }
}
