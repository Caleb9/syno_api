# syno_api

[![Crates.io Version](https://img.shields.io/crates/v/syno_api)](https://crates.io/crates/syno_api)

An incomplete set of data transfer objects (DTOs) and errors for
Synology DSM APIs.

## Features

By default, all of the following features are enabled.

* `dto` - provides DTOs for various JSON responses produced by
  Synology DSM APIs
* `error` - provides enums implementing `Error` and `Display` traits,
  and `From`/`TryFrom` for `u16` to convert from error codes returned
  by the API to enum values
* `serde` - adds `Serialize` and `Deserialize` trait implementations
  to DTOs (adds dependency on `serde` library).

## Contributing

I would very much welcome pull requests adding more types /
functionality. Current set is driven by what my other projects need
from the APIs.

## Disclaimer

This project is an independent, open-source library and is not
affiliated, associated, authorized, endorsed by, or in any way
officially connected with Synology Inc. "Synology" and any related
product names, logos, and trademarks are the property of Synology Inc.

The use of Synology APIs in this project is solely for
interoperability purposes, and the project does not provide any
official support from Synology. All trademarks, product names, and
company names mentioned in this repository belong to their respective
owners.
