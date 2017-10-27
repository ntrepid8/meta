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

mod mmdb_env;

extern crate rand;
extern crate sysconf;

use std::path::PathBuf;
// use std::fs::File;
use std::io::prelude::*;

extern crate memmap;
use self::memmap::{Mmap, Protection};

use std::env;
use std::io::{self, Write};
use std::fs::File;

static LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::slice;

// use rand::ThreadRng;

pub fn read_page() {
    let path = PathBuf::from("/tmp/meta_mmdb_test.dat");
    let file = File::open(path).expect("failed to open the file");
}

pub fn get_pagesize() -> usize {
    sysconf::page::pagesize()
}

pub fn open_db(path: &str) -> Mmap {
    let file_mmap = Mmap::open_path(path, Protection::Read).unwrap();
    file_mmap
}

pub fn get_page(db: &Mmap, page_no: isize) -> &[u8] {
    // get page size
    let pagesize = sysconf::page::pagesize() as isize;
    // offset for read start
    let page_offset = pagesize * page_no;
    // read bytes
    let bytes = unsafe {
        // offset pointer by page_offset
        let ptr_start = db.ptr().offset(page_offset);
        // slice just one page
        slice::from_raw_parts(ptr_start, pagesize as usize)
    };
    &bytes
}

#[cfg(test)]
mod test {
    use super::*;
    use mmdb::rand::Rng;

    #[test]
    fn test_open_db() {
        let path = Path::new("/tmp/meta_mmdb_test.dat");
        let display = path.display();
        let rstr: String = rand::thread_rng().gen_ascii_chars().take(4096).collect();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };
        // configure pages
        let mut page_one = "0123456789".to_string();
        page_one.push_str(&rstr);
        page_one.truncate(4096);
        let mut page_two = "9876543210".to_string();
        page_two.push_str(&rstr);
        page_two.truncate(4096);
        let mut pages = "".to_string();
        pages.push_str(&page_one);
        pages.push_str(&page_two);
        // rstr_bytes.push_str(&rstr);
        // Write` the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(&pages.into_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        let db = open_db("/tmp/meta_mmdb_test.dat");
        {
            // read page_one
            let bytes: &[u8] = get_page(&db, 0);
            assert_eq!(b"0123456789", &bytes[0..10]);
            assert_eq!(4096, bytes.len());
        }
        {
            // read page_two
            let bytes: &[u8] = get_page(&db, 1);
            assert_eq!(b"9876543210", &bytes[0..10]);
            assert_eq!(4096, bytes.len());
        }
    }

    #[test]
    fn test_get_pagesize() {
        let page_size = get_pagesize();
        assert_eq!(4096, page_size)
    }
}
