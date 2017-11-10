// Meta - The Meta Server (TMS)
// Copyright (C) 2017 Josh Austin

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(unique)]
pub mod mmdb;
extern crate lmdb_rs;
use std::path::PathBuf;
use lmdb_rs::core::{DbFlags, DbHandle, EnvBuilder, EnvNoMemInit, EnvNoMetaSync, Environment,
                    KeyExists, MdbValue};

const USER_DIR: u32 = 0o777;

// pub fn create_db() {
//     let path = PathBuf::from("/tmp/meta_lmdb_test");
//     let mut env = EnvBuilder::new()
//         .max_readers(33)
//         .open(&path, USER_DIR).unwrap();

//     env.sync(true).unwrap();

//     let test_flags = EnvNoMemInit | EnvNoMetaSync;

//     env.set_flags(test_flags, true).unwrap();
//     let new_flags = env.get_flags().unwrap();
//     assert!((new_flags & test_flags) == test_flags, "Get flags != set flags");

//     let db = env.get_default_db(DbFlags::empty()).unwrap();
//     let txn = env.new_transaction().unwrap();

//     {
//         let db = txn.bind(&db);

//         let key = "hello";
//         let value = "world";

//         db.set(&key, &value).unwrap();

//         let v = db.get::<&str>(&key).unwrap();


//         println!("from lmdb: key={} v={}", key, v);
//     }
//     txn.commit().unwrap();
// }

pub fn open_env() -> Environment {
    let path = PathBuf::from("/tmp/meta_lmdb_test");
    let mut env = EnvBuilder::new()
        .max_readers(33)
        .open(&path, USER_DIR)
        .unwrap();

    env.sync(true).unwrap();

    let test_flags = EnvNoMemInit | EnvNoMetaSync;

    env.set_flags(test_flags, true).unwrap();
    let new_flags = env.get_flags().unwrap();
    assert!(
        (new_flags & test_flags) == test_flags,
        "Get flags != set flags"
    );
    env
}

pub fn open_db_default(db_env: &Environment) -> DbHandle {
    let db = db_env.get_default_db(DbFlags::empty()).unwrap();
    db
}

pub fn set(db: &DbHandle, env: &Environment, key: &str, value: &str) {
    let txn = env.new_transaction().unwrap();
    {
        let db_txn = txn.bind(&db);
        db_txn.set(&key, &value).unwrap();
    }
    txn.commit().unwrap();
}

pub fn get(db: &DbHandle, env: &Environment, key: &str) -> String {
    let reader = env.get_reader().unwrap();
    let db_reader = reader.bind(&db);
    let val = db_reader.get::<&str>(&key).unwrap();
    val.to_string()
}


#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_open_db() {
        // ensure the file exists
        {
            fs::create_dir("/tmp/meta_lmdb_test");
        }
        let env = open_env();
        let db = open_db_default(&env);
        let key = "hello";
        let value = "world";
        set(&db, &env, key, value);
        let val = get(&db, &env, key);
        assert_eq!(val, "world")
    }
}
