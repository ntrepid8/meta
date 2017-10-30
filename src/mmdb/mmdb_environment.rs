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

// TODO: update types of struct properties (i32 is just a placeholder)
use std::fs::File;
use std::path::Path;

struct MmdbEnvironment {
    // data file descriptor
    data_file: File,
    // configuration
    config: MmdbEnvironmentConfig,
}

#[derive(Default)]
struct MmdbEnvironmentConfig {
    // data file path
    data_file_path: String,
    // lock file descriptor
    me_lock_fd: i32,
    // meta page file descriptor
    me_meta_fd: i32,
    // environment flags
    me_flags: i32,
    // db page size
    me_page_size: i32,
    // os page size
    me_os_page_size: i32,
    // maximum number of readers
    me_max_readers: i32,
    // number of DBs
    me_num_dbs: i32,
    // maximum number of DBs
    me_max_dbs: i32,
    // PID of this environment
    me_pid: i32,
    // path to the DB files
    me_path: String,
    // pointer to memory map of data file
    me_mmap_data: i32,
    // pointer to memory map of lock file
    me_mmap_lock: i32,
    // pointers to the meta pages
    me_metas: i32,
    // pointer to the current write transaction
    me_txn: i32,
    // current size of the data memory map
    me_mmap_data_size: i32,
    // current size of the data file
    me_file_data_size: i32,
    // current page count (me_mmap_data_size / me_page_size)
    me_mmap_data_page_count: i32,
    // DB meta data
    me_db_info: i32,
    // DB flags
    me_db_flags: i32,
    // allocated pages ready for use
    me_ready_pages: i32,
    // pages freed during a write transaction
    me_free_pages: i32,
    // pages written during a write transaction
    me_dirty_pages: i32,
    // maximum key size
    me_max_key: i32,
    // maximum value size
    me_max_val: i32,
}

impl MmdbEnvironment {
    // construct a new MmdbEnvironment
    pub fn open(path: &str) -> Result<MmdbEnvironment, &'static str> {
        let db_file_path = Path::new(&path);
        let mut file = match File::open(&db_file_path) {
            Err(e) => return Err("error opening file"),
            Ok(f) => f,
        };
        let new_env = MmdbEnvironment {
            data_file: file,
            config: MmdbEnvironmentConfig {
                data_file_path: path.to_string(),
                ..Default::default()
            }
        };
        Ok(new_env)
    }

    // return data_file_path
    pub fn get_data_file_path(&self) -> &str {
        &self.config.data_file_path
    }

}

#[cfg(test)]
mod test {
    extern crate rand;
    use mmdb::rand::Rng;
    use super::*;

    #[test]
    fn test_open() {
        let rnd_str: String = rand::thread_rng().gen_ascii_chars().take(6).collect();
        let path = format!("/tmp/meta_mmdb_environment_test_{}.dat", rnd_str);
        {
            File::create(&path);
        }
        let env = MmdbEnvironment::open(&path).unwrap();
        assert_eq!(path, env.get_data_file_path());
    }
}
