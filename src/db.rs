use rusqlite::{params, Connection};
use time::Timespec;
use crate::util::{ resolve_file_path, XResult, };

const DEFAULT_DB: &str = "~/.opendid.db";

#[derive(Debug)]
pub struct DbEntry {
    id: i32,
    time_created: Timespec,
    time_modified: Timespec,
    ty: String,
    key: String,
    value: String,
}

impl DbEntry {

    pub fn new(ty: &str, key: &str, value: &str) -> Self {
        Self {
            id: Default::default(),
            time_created: time::get_time(),
            time_modified: time::get_time(),
            ty: ty.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug)]
pub struct Db {
    file: String,
    conn: Connection,
}

impl Db {

    pub fn new_default() -> XResult<Self> {
        Self::new(DEFAULT_DB)
    }

    pub fn new(file: &str) -> XResult<Self> {
        let path = resolve_file_path(file);
        let conn = Connection::open(path)?;
        Ok(Db { file: file.into(), conn: conn, })
    }

    pub fn init(&self) -> XResult<()> {
        let mut stmt = self.conn.prepare(
            "select type, name, tbl_name, rootpage, sql from sqlite_master where tbl_name = ?1"
        )?;
        let mut dbentry_iter = stmt.query_map(params!["entry"], |row| {
            let tbl_name: String = row.get(2)?;
            Ok(tbl_name)
        })?;
        if let None = dbentry_iter.next() {
            self.conn.execute(
                r##"CREATE TABLE entry (
                    id              INTEGER PRIMARY KEY,
                    time_created    TEXT NOT NULL,
                    time_modified   TEXT NOT NULL,
                    ty              TEXT NOT NULL,
                    key             TEXT NOT NULL,
                    value           TEXT
                )"##,
                params![]
            )?;
        }
        Ok(())
    }

    pub fn insert(&self, entry: &DbEntry) -> XResult<()> {
        self.conn.execute(
            "INSERT INTO entry (time_created, time_modified, ty, key, value) values (?1, ?2, ?3, ?4, ?5)",
            params![time::get_time(), time::get_time(), entry.ty, entry.key, entry.value]
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i32) -> XResult<()> {
        self.conn.execute(
            "DELETE FROM entry where id = ?1",
            params![id]
        )?;
        Ok(())
    }

    pub fn find_by_id(&self, id: i32) -> XResult<Option<DbEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, time_created, time_modified, ty, key, value FROM entry WHERE id = ?1"
        )?;
        let mut dbentry_iter = stmt.query_map(params![id], |row| {
            Ok(DbEntry {
                id:            row.get(0)?,
                time_created:  row.get(1)?,
                time_modified: row.get(2)?,
                ty:            row.get(3)?,
                key:           row.get(4)?,
                value:         row.get(5)?,
            })
        })?;

        let e = dbentry_iter.next();
        match e {
            None => Ok(None),
            Some(e) => Ok(Some(e?)),
        }
    }

    pub fn find_first_by_key(&self, ty: &str, key: &str) -> XResult<Option<DbEntry>> {
        let mut entries = self.find_by_key_with_limit(ty, key, 1_usize)?;
        Ok(entries.pop())
    }

    pub fn find_by_key(&self, ty: &str, key: &str) -> XResult<Vec<DbEntry>> {
        self.find_by_key_with_limit(ty, key, 0_usize)
    }

    pub fn find_by_key_with_limit(&self, ty: &str, key: &str, limit: usize) -> XResult<Vec<DbEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, time_created, time_modified, type, key, value FROM entry WHERE type =?1 and key = ?2"
        )?;
        let dbentry_iter = stmt.query_map(params![ty, key], |row| {
            Ok(DbEntry {
                id:            row.get(0)?,
                time_created:  row.get(1)?,
                time_modified: row.get(2)?,
                ty:            row.get(3)?,
                key:           row.get(4)?,
                value:         row.get(5)?,
            })
        })?;

        let mut cnt = 0;
        let mut ret = vec![];
        for e in dbentry_iter {
            ret.push(e?);
            cnt += 1;
            if limit != 0 && cnt >= limit { break; }
        }
        Ok(ret)
    }
}
