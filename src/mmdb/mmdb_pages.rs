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

use memmap::{Mmap, Protection};

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

    pub fn from_raw<'a, MmdbPage>(ptr: *const u8) -> &'a MmdbPage {
        // see: https://doc.rust-lang.org/book/first-edition/raw-pointers.html
        // see: https://doc.rust-lang.org/std/mem/fn.transmute.html
        // see: https://doc.rust-lang.org/std/primitive.pointer.html
        unsafe {
            // this is so unsafe
            // coerce the pointer into the correct type
            let page_ptr = ptr as *const MmdbPage;
            // dereference the pointer into a reference (safer than transmute)
            let ref_page: &MmdbPage = &*page_ptr;
            // return the result
            ref_page
        }
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
        let mmdb_page_read: &MmdbPage = MmdbPage::from_raw(buffer.as_ptr());
        // verify values
        assert_eq!(1, mmdb_page_read.page_no);
        assert_eq!(2, mmdb_page_read.index_upper);
        assert_eq!(3, mmdb_page_read.index_lower);
        assert_eq!(4, mmdb_page_read.overflow_pages);
    }

    #[test]
    fn test_read_page_with_mmap() {
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

        // read via memmap
        let file_mmap = Mmap::open_path(path, Protection::Read).unwrap();
        {
            // offset pointer by page_offset (offset 0 for single page)
            let ptr = file_mmap.ptr();
            let ptr_mem_addr = format!("{:p}", ptr);
            println!("ptr: {:p}", ptr);
            let ptr_start = unsafe {ptr.offset(0)};
            let ptr_start_mem_addr = format!("{:p}", ptr_start);
            assert_eq!(ptr_mem_addr, ptr_start_mem_addr);
            println!("ptr_start: {:p}", ptr_start);
            // transmute to struct (not sure why type can't be inferred here)
            let mmdb_page_read: &MmdbPage = MmdbPage::from_raw(ptr_start);
            let mmdb_page_read_mem_addr = format!("{:p}", mmdb_page_read);
            // verify memory address of new page struct matches the original pointers
            assert_eq!(ptr_mem_addr, mmdb_page_read_mem_addr);
            assert_eq!(ptr_start_mem_addr, mmdb_page_read_mem_addr);
            // verify values
            assert_eq!(1, mmdb_page_read.page_no);
            assert_eq!(2, mmdb_page_read.index_upper);
            assert_eq!(3, mmdb_page_read.index_lower);
            assert_eq!(4, mmdb_page_read.overflow_pages);
            println!("mmdb_page_read: {:p}", mmdb_page_read);
        }
    }
}
