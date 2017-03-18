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

extern crate lmdb_rs;

use std::env;
use std::fs::{self};
use std::path::{PathBuf};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::sync::{Once, ONCE_INIT};
use std::thread;

//use libc::c_int;

use lmdb_rs::core::{self, EnvBuilder, DbFlags, MdbValue, EnvNoMemInit, EnvNoMetaSync, KeyExists};
//use ffi::MDB_val;
//use traits::FromMdbValue;

const USER_DIR: u32 = 0o777;

fn create_db() {
    let path = PathBuf::from("/tmp/meta_lmdb_test");
    let mut env = EnvBuilder::new()
        .max_readers(33)
        .open(&path, USER_DIR).unwrap();

    env.sync(true).unwrap(); 

    let test_flags = EnvNoMemInit | EnvNoMetaSync;

    env.set_flags(test_flags, true).unwrap();
    let new_flags = env.get_flags().unwrap();
    assert!((new_flags & test_flags) == test_flags, "Get flags != set flags");

    let db = env.get_default_db(DbFlags::empty()).unwrap();
    let txn = env.new_transaction().unwrap();

	{
        let db = txn.bind(&db);

        let key = "hello";
        let value = "world";

        db.set(&key, &value).unwrap();

        let v = db.get::<&str>(&key).unwrap();


        println!("from lmdb: key={} v={}", key, v);
	}
    txn.commit().unwrap();
}

fn main() {
	create_db();
    println!("Hello, world!");
}


