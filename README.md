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
