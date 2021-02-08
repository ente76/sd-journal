# sd-journal

[![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)

sd-journal is a rust wrapper for sd-journal in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html).  sd-journal is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project.

github.com | crates.io | docs.rs
-----------|-----------|--------
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-sys)](https://github.com/ente76/sd-sys) | [![Crates.io](https://img.shields.io/crates/v/sd-sys)](https://crates.io/crates/sd-sys) | [![docs.rs](https://docs.rs/sd-sys/badge.svg)](https://docs.rs/sd-sys/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-id128)](https://github.com/ente76/sd-id128) | [![Crates.io](https://img.shields.io/crates/v/sd-id128)](https://crates.io/crates/sd-id128) | [![docs.rs](https://docs.rs/sd-id128/badge.svg)](https://docs.rs/sd-id128/)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-journal)](https://github.com/ente76/sd-journal) | [![Crates.io](https://img.shields.io/crates/v/sd-journal)](https://crates.io/crates/sd-journal) | [![docs.rs](https://docs.rs/sd-journal/badge.svg)](https://docs.rs/sd-journal)

## Structure

libsystemd is developed in C around a single struct "journal" with no differentiation whether a function refers to the journal in total or whether the fuction relates to a single record within the journal.  
This library also offers all the wrapped functions on the main struct `Journal`. Additionally two iterators are implemented for Journal: `CursorIterator` and `CursorReverseIterator` which both return a `Result<Cursor, Error>`. All methods implemented on Cursor do call a method implemented on Journal. For that reason, documentation of Cursor is always referring back to the documentation for Journal.  
libsystemd implements some additional enumerations. For each of those, an iterator has been implemented as well.

## Status & Stability

This library is still under development. There are various methods marked with the feature "experimental". These methods are not considered finalized yet. The documentation of each of these methods contains further information. Additionally the library structure is currently under investigation. Currently all methods are implemented for struct Journal. This may change soon: methods that refer to a single record may be moved to struct Cursor and methods performing cursor movements (next(), previous() and similar ones) will return a Cursor.

### Planned Development

- [ ] further rustification
  - [ ] remove Cursor methods from Journal
  - [ ] CursorMovement return Cursor instead of just a Done
- [ ] additional trait implementation
- [ ] Logger implementation
- [ ] encoding support

### Encoding

Journald stores data as "FIELDNAME=field value". While field names are
strict UTF-8 encoded and field value are usually encoded in UTF-8, field
values may as well be in any encoding including binary data.
This library allows logging to the journal in any encoding although using
UTF-8 only is highly recommended. While reading from the journal this
library will strictly raise an error whenever non-UTF-8 data is encountered.
In future releases decoding support and a lossy decoding may be added.

## Examples

### cargo.toml

```toml
[dependencies]
sdJournal = "0.1"
```

### Logging

```rust
use sd_journal::*;
Journal::log_message(Level::Info, "Hello World!").unwrap();
Journal::log_raw_record(&["MESSAGE=Hello World!",
                          &format!("PRIORITY={}", Level::Info),
                          &format!("CODE_FILE={}", file!()),
                          &format!("CODE_LINE={}", line!()),
                          "CUSTOM_FIELD=42"]).unwrap();
```

### Read Access

```rust
use sd_journal::*;
use std::path::PathBuf;

// load local test data
let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
test_data.push("test-data/");
println!("looking for test data in folder {}", test_data.display());
let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();

// loop over journal records
while let Ok(CursorMovement::Done) = journal.next() {
    // do something on each cursor, e.g. print the MESSAGE
    println!("{}", journal.get_data("MESSAGE").unwrap());
}
```

## License

sd-journal is published under the AGPL-3.0, individual licenses may be granted upon request.

```license
sd-journal: a wrapper for sd-journal of libsystemd
Copyright (C) 2020 Christian Klaue [mail@ck76.de]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Individual licenses may be granted upon request.
```
