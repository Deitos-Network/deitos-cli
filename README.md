# deitos-cli

A command-line utility for interacting with the Deitos chain.

You can see the full list of commands with deitos-cli --help. Most commands have additional help available with
deitos-cli <command> --help.

## Upload a file

To upload a file to the Deitos chain, use the upload command:

```sh
deitos-cli upload --file-path <path> --deitos-url <url> --ip-url <url> --agreement <id> --suri <suri>
```

This will calculate the hash of the file, register it on the Deitos chain, and then upload the file to the
Infrastructure Provider (IP).

### Example

```sh
deitos-cli upload --file-path=./README.md --deitos-url=ws://localhost:9944 --ip-url=http://localhost:9090 --agreement=1 --suri=//Bob
```

This will connect to the Deitos node at ws://localhost:9944 using Bob's keypair,
register README.md on the Deitos chain for the agreement with ID 1, and then upload the file to the IP at
http://localhost:9090.
