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

## License

The project is currently licensed under the GPLv3 license. This encourages
sharing improvements with the community.

If your use case requires a different license, alternative licenses are
available for a fee along with an annual support contract. Please email
josh.austin@gmail.com for details.
