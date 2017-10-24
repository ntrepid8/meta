# Meta

Welcome to the meta-server project. The goal of this project is to build
a key/value server that can operate in a clustered group.

Current goals:

- provide [gRPC](https://grpc.io/) interface
- provide GET/POST/PUT/DELETE web interface
- implement full auth layer
- provide ability to define search indices
- provide streaming/generator style response cursors
- provide automatic sharding within a cluster
- provide automatic network partition handling
- provide good write performance
- provide optimized read performance
- provide ACID compliant transactions
- provide Multiversion concurrency control MVCC

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

## Maintainers

This project is maintained by:

- Josh Austin (josh.austin@gmail.com)

## Contributing

If you would like to contribute to this project you will need to complete the
Contributor Agreement. You can find the agreements in the `./contributor_agreements`
folder of this repository. Please complete it and email a signed copy to the
project maintainer.
