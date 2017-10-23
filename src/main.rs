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

extern crate libmeta;

fn main() {
    // meta::create_db();
    println!("Hello, world!");

    let env = libmeta::open_env();
    let db = libmeta::open_db_default(&env);
    let key = "hello";
    let value = "world";
    libmeta::set(&db, &env, key, value);
    println!("set lmdb: key={} val={}", key, value);
    let val = libmeta::get(&db, &env, key);
    println!("get lmdb: key={} v={}", key, val);
}
