# sdJournal

sdJournal is a rust wrapper for the sd-journal API. sdJournal publishes the ffi interface, a low level interface and a high level interface to the sd-journal API.

## Examples

In `cargo.toml`:

```rust
[dependencies]
sdJournal = "0.1"
```

## Logging

```rust
fn main() {
    sdJournal::log(Priority::Information, "Hello journald.");
}
```

## License

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
