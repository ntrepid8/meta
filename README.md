# Meta

Welcome to the meta-server project.

## Notes

LMDB has the following:
- environments
- databases within an environment

Some possible data models would be:

- Environment is defined by universe_id and tenant_id
- Database is defined by record type
- Keys are 128 bit UUIDs (uuid4)
- Values are MsgPack binaries

## Useful Resources

- https://doc.rust-lang.org/libc/x86_64-unknown-linux-gnu/libc/fn.mmap.html
- https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
- https://www.gnu.org/software/libc/manual/html_node/Memory_002dmapped-I_002fO.html
- http://beej.us/guide/bgipc/output/html/multipage/mmap.html
- https://en.wikipedia.org/wiki/Mmap
- https://docs.rs/memmap/0.5.2/x86_64-unknown-linux-gnu/memmap/struct.Mmap.html

## License

The project is currently licensed under the GPLv3 license. This encourages
sharing improvements with the community.

If your use case requires a different license, alternative licenses are
available for a fee along with an annual support contract. Please email
josh.austin@gmail.com for details.
