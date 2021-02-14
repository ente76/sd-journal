# sd-journal

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-journal?label=github&logo=github)](https://github.com/ente76/sd-journal)  [![Crates.io](https://img.shields.io/crates/v/sd-journal)](https://crates.io/crates/sd-journal)  [![docs.rs](https://docs.rs/sd-journal/badge.svg)](https://docs.rs/sd-journal/)  ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ente76/sd-journal/test?label=test&logo=github) [![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)  

[sd-journal](https://github.com/ente76/sd-journal) is a rust wrapper for sd-journal in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-journal.html).  sd-journal is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project.

## Introduction

### Structure

libsystemd is developed in C around a single struct `journal` with no differentiation whether a function refers to the journal in total or the function relates to a single record within the journal.  This library also offers all the wrapped functions on the main struct `Journal`. Additionally two iterators are implemented for `Journal`: `CursorIterator` and `CursorReverseIterator` which both return a `Result<Cursor, Error>`. All methods implemented on `Cursors` do call a method implemented on `Journal`. For that reason, documentation of `Cursor` is always referring back to the documentation for `Journal`.  
libsystemd implements some additional enumerations. For each of those, an iterator has been implemented as well.

### Compatibility

This library is developed against the latest version of systemd. Unfortunately not all systems are up to date in that regard. Compatibility can be mastered using features. Each feature is named after the corresponding systemd version. The following features exist currently:

- 246
- 245
- 229

All features are in the default feature set. If required, default-features must be turned off. Features are stacking: if you select feature 246, you will get 245 and 229 included.

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
sd-journal = "0.1"
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

## Version History

## Planned Development

- [ ] further rustification
  - [ ] remove Cursor methods from Journal
  - [ ] CursorMovement return Cursor instead of just a Done
- [ ] additional trait implementation
- [ ] Logger implementation
- [ ] encoding support

## System Issues

There are some weird issues with systemd. At first I thought I made some mistakes when wrapping functions. I implemented some of my tests in pure C, just to be sure. I reported issues to the systemd project. I am too small and unimportant to get someone working on the issues unless you point on the exact line in their source code, where they would have to fix something. Unfortunately I am not a C expert. I don't know how to debug libsystemd calls. A major issue is also: you won't be able to reproduce the errors easily.

- `seek_head()` followed by `previous()` or `seek_tail()` followed by `next()`
  - One can assume that `seek_head()` brings you to the head of the journal! Do not doubt! That is. One should just not assume that there is not a `previous()` item.
  - `seek_head()` --> `previous()` succeeds
    - the cursor will be on a valid item
    - the number of items "before the beginning" is limited; the number seems to match the number of log files
    - the items before the start of log are not in order
  - `seek_head()` --> `next()` --> `previous()` works correctly (the last call returns a 0)

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
