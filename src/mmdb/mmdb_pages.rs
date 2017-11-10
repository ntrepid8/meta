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

/*
 * see: https://doc.rust-lang.org/nomicon/vec.html
 */

use std::ptr::{Unique, self};

struct MmdbPage {
    // page number
    page_no: i32,
    // upper and lower bound of free space in the page
    index_upper: i32,
    index_lower: i32,
    // overflow page count
    overflow_pages: i32,
    // data (remainder of space on the 4096 byte page)
    data: [u8; 4080],
}

impl MmdbPage {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (self as *const MmdbPage) as *const u8,
                ::std::mem::size_of::<MmdbPage>(),
            )
        }
    }

    pub fn from_raw(ptr: *const u8) -> MmdbPage {
        let new_page: MmdbPage = unsafe { ptr::read(ptr as *const MmdbPage) };
        new_page
    }
}

struct MmdbPages<MmdbPage> {
    ptr: Unique<MmdbPage>,
    cap: usize,
    len: usize,
}

#[cfg(test)]
mod test {
    extern crate rand;
    // use rand::{Rng, self};
    use super::*;
    use std::{fs, io, mem};
    use std::fs::{File, OpenOptions};
    use std::io::{Write, Read};
    use mmdb::rand::Rng;

    #[test]
    fn test_size() {
        assert_eq!(4, mem::size_of::<i32>());
        assert_eq!(4096, mem::size_of::<MmdbPage>());
    }

    #[test]
    fn test_serialize_page() {
        let rnd_str: String = rand::thread_rng().gen_ascii_chars().take(6).collect();
        let path = format!("/tmp/meta_mmdb_test_serialize_page_{}.dat", rnd_str);
        let mmdb_page = MmdbPage{
            page_no: 1,
            index_upper: 2,
            index_lower: 3,
            overflow_pages: 4,
            data: [0; 4080],
        };
        let mmdb_page_bytes = mmdb_page.as_bytes();

        // write the page
        {
            // open the data file (create if does not exist)
            let mut page_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)
                .unwrap();
            // write the buffer
            page_file.write_all(mmdb_page_bytes);
        }
        // init the read buffer
        // let mut buffer = [0; 4096];
        let mut buffer: Vec<u8> = Vec::with_capacity(4096);
        // read the page
        let mut page_file = OpenOptions::new()
            .read(true)
            .open(&path)
            .unwrap();
        page_file.read_to_end(&mut buffer);
        // transmute to struct
        let mmdb_page_read = MmdbPage::from_raw(buffer.as_ptr());
        // verify values
        assert_eq!(1, mmdb_page_read.page_no);
        assert_eq!(2, mmdb_page_read.index_upper);
        assert_eq!(3, mmdb_page_read.index_lower);
        assert_eq!(4, mmdb_page_read.overflow_pages);
    }
}
