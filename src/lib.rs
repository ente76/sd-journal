// sd-journal: rust wrapper on sd-journal implemented in libsystemd
// Copyright (C) 2020 Christian Klaue ente@ck76.de
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! sd-journal is a rust wrapper for sd-journal in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html).  sd-journal is part of the [systemd.rs](https://gitlab.com/systemd.rs) project.
//!
//! gitlab.com | crates.io | docs.rs
//! -----------|-----------|--------
//! [sd-sys](https://gitlab.com/systemd.rs/sd-sys) | [![Crates.io](https://img.shields.io/crates/v/sd-sys)](https://crates.io/crates/sd-sys) | [![docs.rs](https://docs.rs/sd-sys/badge.svg)](https://docs.rs/sd-sys/)
//! [sd-id128](https://gitlab.com/systemd.rs/sd-id128) | [![Crates.io](https://img.shields.io/crates/v/sd-id128)](https://crates.io/crates/sd-id128) | [![docs.rs](https://docs.rs/sd-id128/badge.svg)](https://docs.rs/sd-id128/)
//! [sd-journal](https://gitlab.com/systemd.rs/sd-journal) | [![Crates.io](https://img.shields.io/crates/v/sd-journal)](https://crates.io/crates/sd-journal) | [![docs.rs](https://docs.rs/sd-journal/badge.svg)](https://docs.rs/sd-journal)
//!
//! systemd.rs is an alternative to the [systemd-rust](https://github.com/jmesmon/rust-systemd) project.
//!
//! - systemd.rs is published under the AGPL-3.0 license. Individual/commercial
//!   licenses are available upon request.
//! - focused coverage of sd-id128 & sd-journal only (currently there are no
//!   plans to extend this coverage)
//! - good documentation with links to the libsystemd documentation
//! - 100% coverage of libsystemd within the area of focus
//! - good test coverage
//! - focus on usability
//!
//! ## Structure
//!
//! libsystemd is developed in C around a single struct "journal" with no
//! differentiation whether a function refers to the journal in total or whether
//! the fuction relates to a single record within the journal. This library also
//! offers all the wrapped functions on the main struct `Journal`. Additionally
//! two iterators are implemented for Journal: `CursorIterator` and
//! `CursorReverseIterator` which both return a `Result<Cursor, Error>`. All
//! methods implemented on Cursor do call a method implemented on Journal. For
//! that reason, documentation of Cursor is always referring back to the
//! documentation for Journal. libsystemd implements some additional
//! enumerations. For each of those, an iterator has been implemented as well.
//!
//! ## Status & Stability
//!
//! This library is still under development. There are various methods marked
//! with the feature "experimental". These methods are not considered finalized
//! yet. The documentation of each of these methods contains further
//! information. Additionally the library structure is currently under
//! investigation. Currently all methods are implemented for struct Journal.
//! This may change soon: methods that refer to a single record may be moved to
//! struct Cursor and methods performing cursor movements (next(), previous()
//! and similar ones) will return a Cursor.
//!
//! ### Planned Development
//!
//! - [ ] further rustification
//!   - [ ] remove Cursor methods from Journal
//!   - [ ] CursorMovement return Cursor instead of just a Done
//! - [ ] additional trait implementation
//! - [ ] Logger implementation
//! - [ ] encoding support
//!
//! ### Encoding
//!
//! Journald stores data as `FIELDNAME=field value`. While field names are
//! strict UTF-8 encoded and field value are usually encoded in UTF-8, field
//! values may as well be in any encoding including binary data.
//! This library allows logging to the journal in any encoding although using
//! UTF-8 only is highly recommended. While reading from the journal this
//! library will strictly raise an error whenever non-UTF-8 data is encountered.
//! In future releases decoding support and a lossy decoding may be added.
//!
//! ## Examples
//!
//! ### cargo.toml
//!
//! ```toml
//! [dependencies]
//! sdJournal = "0.1"
//! ```
//!
//! ### Logging
//!
//! ```rust
//! use sd_journal::*;
//! Journal::log_message(Level::Info, "Hello World!").unwrap();
//! Journal::log_raw_record(&["MESSAGE=Hello World!",
//!                           &format!("PRIORITY={}", Level::Info),
//!                           &format!("CODE_FILE={}", file!()),
//!                           &format!("CODE_LINE={}", line!()),
//!                           "CUSTOM_FIELD=42"]).unwrap();
//! ```
//!
//! ### Read Access
//!
//! ```rust
//! use sd_journal::*;
//! use std::path::PathBuf;
//!
//! // load local test data
//! let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//! test_data.push("test-data/");
//! println!("looking for test data in folder {}", test_data.display());
//! let journal =
//!     Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
//!
//! // loop over journal records
//! while let Ok(CursorMovement::Done) = journal.next() {
//!     // do something on each cursor, e.g. print the MESSAGE
//!     println!("{}", journal.get_data("MESSAGE").unwrap());
//! }
//! ```
//!
//! ## License
//!
//! sd-journal: a wrapper for sd-journal of libsystemd
//!
//! Copyright (C) 2020 Christian Klaue [mail@ck76.de]
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! Individual licenses may be granted upon request.
mod enums;
pub mod iterators;

use chrono::{Duration, NaiveDateTime};
pub use enums::{CursorMovement, Enumeration, Error, Event, FileFlags, Level, NamespaceFlags,
                PathFlags, UserFlags};
use iterators::{CursorIterator, CursorReverseIterator, FieldNames, Fields, UniqueValues};
use libc::{c_char, c_int, c_uchar, c_void, iovec, size_t};
use sd_id128::ID128;
use sd_sys::journal as ffi;
use std::{ffi::{CStr, CString},
          fmt::Debug,
          path::PathBuf,
          ptr};

/// A wrapper for sd-journal as offered by libsystemd based on FFI bindings
/// offered in crate [sd-sys](https://gitlab.com/systemd.rs/sd-sys).
///
/// Journal is a fully implemented, wrapper for submitting and querying log
/// entries from the systemd journal.
#[derive(Debug)]
pub struct Journal {
    ffi: *mut ffi::sd_journal
}

/// A journal entry record
#[derive(Debug)]
pub struct Cursor<'a> {
    pub(crate) journal: &'a Journal
}

impl Journal {
    /// Submits a simple, plain text log message with a chosen syslog level to
    /// the journal (implements
    /// [`sd_journal_print()`](<https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>)).
    ///
    /// The message submitted to `log_message()` may be anything that can be
    /// turned into a vector of bytes. Journald considers non-UTF-8 values as
    /// valid message although 0-bytes within the message cause an error.
    ///
    /// # Examples
    /// ```
    /// use sd_journal::*;
    /// use std::ffi::CString;
    /// // the following lines are all synonyms
    /// Journal::log_message(Level::Info, "Hello World!").unwrap();
    /// Journal::log_message(Level::Info, String::from("Hello World!").as_str()).unwrap();
    /// Journal::log_message(Level::Info, String::from("Hello World!")).unwrap();
    /// Journal::log_message(Level::Info, CString::new("Hello World!").unwrap()).unwrap();
    /// ```
    ///
    /// # Return Values
    /// - Ok(): success
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::NullError): the message contained a 0-byte
    pub fn log_message<T: Into<Vec<u8>>>(level: Level, message: T) -> Result<(), Error> {
        let c_message = CString::new(message).map_err(Error::NullError)?;
        let result = unsafe { ffi::sd_journal_print(level as c_int, c_message.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Send a raw log record to the journal (implements
    /// [`sd_journal_sendv()`](<https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>))
    ///
    /// This method may be used to submit structured log entries to the system
    /// journal. It takes any slice of byte-slices (e.g. &[String] or &[&str]).
    ///
    /// For libsystemd a single log record consists of multiple tuples each in
    /// the format "FIELDNAME=fieldvalue". The field name must be in uppercase
    /// and consist only of characters, numbers and underscores, and may not
    /// begin with an underscore. All assignments that do not follow this
    /// syntax will silently be ignored. A variable may be assigned more than
    /// one value per entry. Well known field names are defined in enum
    /// [`Field`](Field) or may be [looked up](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html#).
    ///
    /// The value can be of any size and format, i.e. the data encoding may
    /// differ from UTF8 and may contain binary coding.
    ///
    /// # Examples
    /// ```
    /// use sd_journal::*;
    /// // the first two lines are synonyms
    /// Journal::log_message(Level::Info, "Hello World!").unwrap();
    /// Journal::log_raw_record(&["PRIORITY=6", "MESSAGE=Hello World!"]).unwrap();
    /// // data: &Vec<String>
    /// Journal::log_raw_record(&vec![format!("PRIORITY={}", Level::Info),
    ///                               "MESSAGE=Hello World!".to_string(),
    ///                               format!("CODE_LINE={}", line!()),
    ///                               format!("CODE_FILE={}", file!()),
    ///                               "CUSTOM_FIELD=42".to_string()]).unwrap();
    /// // data: &[&str]
    /// Journal::log_raw_record(&["MESSAGE=Hello World!",
    ///                           &format!("PRIORITY={}", Level::Info),
    ///                           &format!("CODE_FILE={}", file!()),
    ///                           &format!("CODE_LINE={}", line!()),
    ///                           "CUSTOM_FIELD=42"]).unwrap();
    /// ```
    ///
    /// # Return Values
    /// - Ok(): success
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn log_raw_record<T: AsRef<[u8]>>(data: &[T]) -> Result<(), Error> {
        let mut iovec_vec: Vec<iovec> = Vec::new();
        for field in data {
            let field = field.as_ref();
            iovec_vec.push(iovec { iov_base: field.as_ptr() as *mut c_void,
                                   iov_len:  field.len() });
        }
        let result = unsafe { ffi::sd_journal_sendv(iovec_vec.as_ptr(), iovec_vec.len() as c_int) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Determine the message cataloge entry for a message id (implements
    /// [`sd_journal_get_catalog_for_message_id()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#)).
    ///
    /// # Return Values
    /// - Ok(String): message catalogue
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn get_catalog_for_message_id(id: ID128) -> Result<String, Error> {
        let mut data: *mut c_char = ptr::null_mut();
        let result =
            unsafe { ffi::sd_journal_get_catalog_for_message_id(id.into_ffi(), &mut data) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let catalog = unsafe { CStr::from_ptr(data) };
        let catalog = match catalog.to_str() {
            Err(error) => {
                unsafe { libc::free(data as *mut c_void) };
                Err(Error::UTF8Error(error))?
            },
            Ok(value) => value.to_owned()
        };
        unsafe { libc::free(data as *mut c_void) };
        Ok(catalog)
    }

    /// Open a journal for read access (implements
    /// [`sd_journal_open()`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Opens the log journal for reading. It will find all journal files
    /// and interleave them automatically when reading.
    ///
    /// # Examples
    /// ```
    /// use sd_journal::*;
    /// Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// Journal::open(FileFlags::LocalOnly, UserFlags::CurrentUserOnly).unwrap();
    ///  ```
    ///
    /// # Return values
    /// - Ok(Journal): initialized journal
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn open(file_flags: FileFlags, user_flags: UserFlags) -> Result<Journal, Error> {
        let mut pointer = ptr::null_mut() as *mut sd_sys::journal::sd_journal;
        let flags = file_flags as c_int | user_flags as c_int;
        let result = unsafe { ffi::sd_journal_open(&mut pointer, flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(Journal { ffi: pointer })
    }

    /// Open the journal for reading records in a specific namespace (implements
    /// [`sd_journal_open_namespace()`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Opens the log journal for reading on a selected
    /// [namespace](https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html#Journal%20Namespaces)
    /// only. It will find all journal files and interleave them automatically
    /// when reading. This method does not support the
    /// `SD_JOURNAL_ALL_NAMESPACES` flag. If you want to open all namespaces,
    /// see `open_all_namespaces()`.
    ///
    /// # Return values
    /// - Ok(Journal): initialized journal
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::NullError): the namespace contained a 0-byte
    #[cfg(any(feature = "245", feature = "246"))]
    pub fn open_namespace<T: Into<Vec<u8>>>(namespace: T,
                                            namespace_flags: NamespaceFlags,
                                            file_flags: FileFlags,
                                            user_flags: UserFlags)
                                            -> Result<Journal, Error> {
        let c_namespace = CString::new(namespace).map_err(Error::NullError)?;
        let mut pointer = ptr::null_mut() as *mut ffi::sd_journal;
        let flags = file_flags as c_int | user_flags as c_int | namespace_flags as c_int;
        let result =
            unsafe { ffi::sd_journal_open_namespace(&mut pointer, c_namespace.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Open the journal for read access including all available namespaces
    /// (implements
    /// [`sd_journal_open_namespace()`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)
    /// with flag `SD_JOURNAL_ALL_NAMESPACES` set).
    ///
    /// Opens the log journal for reading for all
    /// [namespaces](https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html#Journal%20Namespaces).
    /// It will find all journal files automatically and interleave
    /// them automatically when reading.
    ///
    /// # Return values
    /// - Ok(Journal): initialized journal
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn open_all_namespaces(file_flags: FileFlags,
                               user_flags: UserFlags)
                               -> Result<Journal, Error> {
        let mut pointer = ptr::null_mut() as *mut ffi::sd_journal;
        let flags = file_flags as c_int | user_flags as c_int | ffi::SD_JOURNAL_ALL_NAMESPACES;
        let result =
            unsafe { ffi::sd_journal_open_namespace(&mut pointer, std::ptr::null(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Open the journal located at a specific path (implements
    /// [`sd_journal_open_directory()`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Open the journal located at a specific path: takes an *absolute*
    /// directory path as argument. All journal files in this directory
    /// will be opened and interleaved automatically.
    ///
    /// # Examples
    /// ```
    /// use sd_journal::*;
    /// use std::path::{Path, PathBuf};
    /// // open the system journal by pointing to root with path flags set to
    /// // PathToOSRoot
    /// Journal::open_directory("/", PathFlags::PathToOSRoot, UserFlags::AllUsers).unwrap();
    /// Journal::open_directory(Path::new("/"), PathFlags::PathToOSRoot, UserFlags::AllUsers).unwrap();
    /// Journal::open_directory(PathBuf::from("/"),
    ///                         PathFlags::PathToOSRoot,
    ///                         UserFlags::AllUsers).unwrap();
    /// // open test data included in a project located in a folder "test-data" in the
    /// // project root
    /// let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// test_data.push("test-data/");
    /// println!("looking for test data in folder {}", test_data.display());
    /// Journal::open_directory(test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// ```
    /// # Return values
    /// - Ok(Journal): initialized journal
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::NullError): the path contains a 0-byte
    pub fn open_directory<P: Into<PathBuf>>(path: P,
                                            path_flags: PathFlags,
                                            user_flags: UserFlags)
                                            -> Result<Journal, Error> {
        #[cfg(unix)]
        use std::os::unix::ffi::OsStringExt;
        let c_path =
            CString::new(path.into().into_os_string().into_vec()).map_err(Error::NullError)?;
        let mut pointer = ptr::null_mut() as *mut ffi::sd_journal;
        let flags = path_flags as c_int | user_flags as c_int;
        let result =
            unsafe { ffi::sd_journal_open_directory(&mut pointer, c_path.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Open the journal stored in a list of files (implements
    /// [`sd_journal_open_files()`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// # Examples
    /// ```
    /// use sd_journal::*;
    /// use std::path::PathBuf;
    /// // to open the curreńt system.journal file in the default location for
    /// // journals: /var/log/journal/<MACHINE-ID>/system.journal
    /// let machine_id = sd_id128::ID128::machine_id().unwrap()
    ///                                               .to_string_sd()
    ///                                               .unwrap();
    /// let mut sdjournal_path = PathBuf::from("/var/log/journal/");
    /// sdjournal_path.push(&machine_id);
    /// sdjournal_path.push("system.journal");
    /// println!("looking for sd-journal in {}", sdjournal_path.display());
    /// Journal::open_files([sdjournal_path]).unwrap();
    /// // to open test data included in a project located in a folder
    /// // "test-data" in the project root
    /// let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// test_data.push("test-data/test.journal");
    /// println!("looking for test data in folder {}", test_data.display());
    /// Journal::open_files([test_data]).unwrap();
    /// ```
    ///
    /// # Return values
    /// - Ok(Journal): initialized journal
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::NullError): a file path contains a 0-byte
    pub fn open_files<A: Into<Vec<P>>, P: Into<PathBuf>>(files: A) -> Result<Journal, Error> {
        #[cfg(unix)]
        use std::os::unix::ffi::OsStringExt;
        let files: Vec<P> = files.into();
        let mut c_files_vec: Vec<CString> = Vec::with_capacity(files.len());
        // convert Vec<PathBuf> to Vec<CString>
        for file in files {
            let pb_file: PathBuf = file.into();
            let os_file = pb_file.into_os_string();
            c_files_vec.push(CString::new(os_file.into_vec()).map_err(Error::NullError)?);
        }
        // convert Vec<CString> to Vec<*const c_char>
        let mut ptr_vec: Vec<*const c_char> =
            c_files_vec.iter().map(|file| file.as_ptr()).collect();
        ptr_vec.push(0 as *const c_char);
        let mut pointer = std::ptr::null_mut() as *mut ffi::sd_journal;
        let flags: c_int = 0;
        let result = unsafe { ffi::sd_journal_open_files(&mut pointer, ptr_vec.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Advance the read pointer of the journal by one entry (implements
    /// [`sd_journal_next()`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// This method wraps the sd-journal native function. There is also a
    /// rustified iterator [`CursorIterator`](CursorIterator) avalaible via
    /// the `iter()` method or the `IntoIterator` trait implemented for
    /// `&Journal`.
    /// Although the official documentation doesn't mention any error handling,
    /// libsystemd may return an error on performing next().
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// // loop over a journal & print it's messages
    /// while let Ok(CursorMovement::Done) = journal.next() {
    ///     // do something on each cursor, e.g. print the MESSAGE
    ///     println!("{}", journal.get_data("MESSAGE").unwrap());
    /// }
    /// ```
    ///
    /// # Return values
    /// - Ok(CursorMovement::Done): full success
    /// - Ok(CursorMovement::EoF): no movement was executed, since the cursor is
    ///   already placed at EoF.
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn next(&self) -> Result<CursorMovement, Error> {
        let result = unsafe { ffi::sd_journal_next(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(CursorMovement::EoF);
        }
        Ok(CursorMovement::Done)
    }

    /// Returns an iterator on the journal.
    ///
    /// [CursorIterator](CursorIterator) is the rustified version of the
    /// `next()` method. Since `next()` may fail, the advanced cursor may be
    /// invalid. For such reason, the iterator returns a Result<Cursor, _> on
    /// each `next()` and thus the cursor must be unwrapped first.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// // loop over a journal & print it's messages
    /// for cursor in journal.iter() {
    ///     match cursor {
    ///         Err(_) => break,
    ///         Ok(cursor) => println!("{}", cursor.get_data("MESSAGE").unwrap())
    ///     }
    /// }
    /// // ...
    /// # journal.seek_head().unwrap();
    /// let cursor = journal.iter_reverse().next().unwrap().unwrap();
    /// // the following two lines are actually return the same value
    /// let m1 = cursor.get_data("MESSAGE").unwrap();
    /// let m2 = journal.get_data("MESSAGE").unwrap();
    /// assert_eq!(m1, m2);
    /// ```
    pub fn iter(&self) -> CursorIterator {
        CursorIterator { journal: &self }
    }

    /// Set back the read pointer of the journal by one entry (implements
    /// [`sd_journal_previous()`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// This method wraps the sd-journal native function. There is also a
    /// rustified iterator [`CursorReverseIterator`](CursorReverseIterator)
    /// avalaible via the [`iter_reverse()`](Journal::iter_reverse) method.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// journal.seek_tail().unwrap();
    /// // loop over a journal in reverse order & print it's messages
    /// while let Ok(CursorMovement::Done) = journal.previous() {
    ///     // do something on each cursor, e.g. print the MESSAGE
    ///     println!("{}", journal.get_data("MESSAGE").unwrap());
    /// }
    /// ```
    ///
    /// # Return values
    /// - Ok(CursorMovement::Done): full success
    /// - Ok(CursorMovement::EoF): no movement was executed, since the cursor is
    ///   already placed at EoF.
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn previous(&self) -> Result<CursorMovement, Error> {
        let result = unsafe { ffi::sd_journal_previous(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(CursorMovement::EoF);
        }
        Ok(CursorMovement::Done)
    }

    /// Returns an iterator on the journal that runs in reverse order.
    ///
    /// [CursorReverseIterator](CursorReverseIterator) is the rustified version
    /// of the `previous()` method. Since `previous()` may fail, the advanced
    /// cursor may be invalid. For such reason, the iterator returns a
    /// Result<Cursor, _> on each `next()` and thus the cursor must be
    /// unwrapped first.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// journal.seek_tail().unwrap();
    /// // loop over a journal & print it's messages
    /// for cursor in journal.iter_reverse() {
    ///     match cursor {
    ///         Err(_) => break,
    ///         Ok(cursor) => println!("{}", cursor.get_data("MESSAGE").unwrap())
    ///     }
    /// }
    /// // ...
    /// # journal.seek_tail().unwrap();
    /// let cursor = journal.iter_reverse().next().unwrap().unwrap();
    /// // the following two lines are actually return the same value
    /// let m1 = cursor.get_data("MESSAGE").unwrap();
    /// let m2 = journal.get_data("MESSAGE").unwrap();
    /// assert_eq!(m1, m2);
    /// ```
    pub fn iter_reverse(&self) -> CursorReverseIterator {
        CursorReverseIterator { journal: &self }
    }

    /// Advance the read pointer of the journal by multiple entries (implements
    /// [`sd_journal_next_skip()`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// # Return values
    /// - Ok(CursorMovement::Done): full success
    /// - Ok(CursorMovement::Limited(actual)): the movement was executed but
    ///   limited by the EoF of the journal. The actual movement is given in the
    ///   parameter.
    /// - Ok(CursorMovement::EoF): no movement was executed, since the cursor is
    ///   already placed at EoF.
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn next_skip(&self, skip: c_int) -> Result<CursorMovement, Error> {
        if skip < 0 {
            return Err(Error::RangeError);
        }
        let result = unsafe { ffi::sd_journal_next_skip(self.ffi, skip as u64) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(CursorMovement::EoF);
        }
        if result < skip {
            return Ok(CursorMovement::Limited(result));
        }
        Ok(CursorMovement::Done)
    }

    /// Set back the read pointer by multiple entries at once (implements
    /// [`sd_journal_previous_skip()`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// - Ok(CursorMovement::Done): full success
    /// - Ok(CursorMovement::Limited(actual)): the movement was executed but
    ///   limited by the EoF of the journal. The actual movement is given in the
    ///   parameter.
    /// - Ok(CursorMovement::EoF): no movement was executed, since the cursor is
    ///   already placed at EoF.
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn previous_skip(&self, skip: c_int) -> Result<CursorMovement, Error> {
        if skip < 0 {
            return Err(Error::RangeError);
        }
        let result = unsafe { ffi::sd_journal_previous_skip(self.ffi, skip as u64) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(CursorMovement::EoF);
        }
        if result < skip {
            return Ok(CursorMovement::Limited(result));
        }
        Ok(CursorMovement::Done)
    }

    /// Seek to the head of the journal (implements
    /// [`sd_journal_seek_head`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seek to the beginning of the journal, i.e. to the position **before**
    /// the oldest available entry. Be aware that after a seek_head() the
    /// journal cursor does not point to a valid entry. One must perform a
    /// cursor movement before being able to retrieve data.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// journal.seek_head().unwrap();
    /// // seek_head() should be followed by a next() before any previous() --> issues
    /// journal.next().unwrap();
    /// // previous() should hit EoF
    /// assert_eq!(journal.previous(), Ok(CursorMovement::EoF));
    /// ```
    ///
    /// # libsystemd Issues
    /// While `seek_head()` is supposed to move the cursor before the first
    /// available journal entry, libsystemd may still successfully perform
    /// `previous()` cursor movements for multiple times. All these unexpected
    /// entries will report a full set of data and may appear fully valid
    /// although be assured they are not. An
    /// [error](https://github.com/systemd/systemd/issues/17662) has been
    /// reported to the systemd project. The issue can be avoided if a `next()`
    /// operation is executed immediately after `seek_head()` before issuing any
    /// `previous()`. If done, `previous()`thereafter will correctly report EoF.
    ///
    /// # Return values
    /// - Ok(()): success
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn seek_head(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_seek_head(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Seek to the tail of the journal (implements
    /// [`sd_journal_seek_tail`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seek to the end of the journal, i.e. the position after the most recent
    /// available entry. Be aware that after a seek_head() the
    /// journal cursor does not point to a valid entry. One must perform a
    /// cursor movement before being able to retrieve data.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// journal.seek_tail().unwrap();
    /// // seek_head() should be followed by a previous() before any next() --> issues
    /// journal.previous().unwrap();
    /// // next() should hit EoF
    /// assert_eq!(journal.next(), Ok(CursorMovement::EoF));
    /// ```
    ///
    /// # libsystemd Issues
    /// While `seek_head()` is supposed to move the cursor before the first
    /// available journal entry, libsystemd may still successfully perform
    /// `previous()` cursor movements for multiple times. All these unexpected
    /// entries will report a full set of data and may appear fully valid
    /// although be assured they are not. An
    /// [error](https://github.com/systemd/systemd/issues/17662) has been
    /// reported to the systemd project. The issue can be avoided if a `next()`
    /// operation is executed immediately after `seek_head()` before issuing any
    /// `previous()`. If done, `previous()`thereafter will correctly report EoF.
    ///
    /// # Return values
    /// - Ok(()): success
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn seek_tail(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_seek_tail(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// **UNSTABLE API** Seek to a monotonic timestamp of a certain boot id
    /// (implements [`sd_journal_seek_monotonic_usec()`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seek to a position with the specified monotonic timestamp, i.e.
    /// `clockMonotonic'. Since monotonic time restarts on every reboot a
    /// boot ID needs to be specified as well.
    ///
    /// # Examples
    /// ```
    /// use sd_id128::*;
    /// use sd_journal::*;
    /// let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// // get the current system boot id
    /// let boot_id = ID128::boot_id().unwrap();
    /// // get the monotonic clock range of today
    /// let (from, _) = journal.get_monotonic_cutoff(boot_id.clone()).unwrap();
    /// // seek to the start of journal for the current boot
    /// journal.seek_monotonic(boot_id.clone(), from).unwrap();
    /// journal.previous().unwrap();
    /// // do something with the first cursor of today...
    /// ```
    ///
    /// # libsystemd Issues
    /// According to the specification of `sd_journal_seek_monotonic_usec()`:
    ///
    /// > If no entry exists that matches exactly the specified seek address,
    /// the next closest is sought to.
    ///
    /// Unfortunately libsystemd fails to comply if the monotonic timestamp
    /// provided points to a position outside the journal range. Lets assume the
    /// first valid log entry for a certain boot id exists at timestamp 500usec.
    /// Seeking to anything beyond 500usec will work as expected while seeking
    /// to anything before 500usec followed by a next() won't position the
    /// cursor at the first entry of that boot id but rather position the cursor
    /// at some random position. An
    /// [issue](https://github.com/systemd/systemd/issues/17763) has been
    /// reported to the systemd project.
    ///
    /// # Stability
    /// This method expects `chrono::Duration` in the same way as
    /// `get_monotonic()` for the same reasons: `get_realtime()` refers to
    /// `chrono::NaiveDateTime`. In future releases this method may be changed
    /// to microsenconds (u128) or std::time::Duration. Such change is
    /// reasonable likely and will be made based on user feedback.
    ///
    /// # Return values
    /// - Ok(())
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::TimeStampOutOfRange): the `clock_monotonic` time stamp
    ///   either reflects a negative duration or the duration exceeds i64
    ///   microseconds
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn seek_monotonic(&self, boot_id: ID128, clock_monotonic: Duration) -> Result<(), Error> {
        // let usec = clock_monotonic.to_std()
        //                           .map_err(|_| Error::TimeStampOutOfRange)?
        //                           .as_micros();
        // let usec = u64::try_from(usec).map_err(|_| Error::TimeStampOutOfRange);
        let usec: u64 = match clock_monotonic.num_microseconds() {
            None => Err(Error::TimeStampOutOfRange)?,
            Some(t) if t < 0 => Err(Error::TimeStampOutOfRange)?,
            Some(t) => t as u64
        };

        let ffi_boot_id = boot_id.into_ffi();
        let result = unsafe { ffi::sd_journal_seek_monotonic_usec(self.ffi, ffi_boot_id, usec) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// **UNSTABLE API** Seek to realtime timestamp (implements
    /// [`sd_journal_seek_realtime_usec()`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seeks to a position with the specified realtime (wallclock) timestamp,
    /// i.e. 'clockRealtime'. Note that the realtime clock is not necessarily
    /// monotonic. If a realtime timestamp is ambiguous, it is not defined which
    /// position is sought to.
    ///
    /// # Stability
    /// Currently the function expects a chrono::NaiveDateTime. In future
    /// releases this method may be changed to expect microseconds (u128) or
    /// std::time::Duration although this is very unlikely. Changes will be
    /// made based on user feedback.
    ///
    /// # Return values
    /// - Ok(())
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn seek_realtime(&self, clock_realtime: NaiveDateTime) -> Result<(), Error> {
        let usec = clock_realtime.timestamp_subsec_micros() as u64
                   + clock_realtime.timestamp() as u64 * 1_000_000;
        let result = unsafe { ffi::sd_journal_seek_realtime_usec(self.ffi, usec) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// **UNSTABLE API** Seeks the journal to the position of the cursor
    /// provided (implements [`sd_journal_seek_cursor()`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// # Stability
    /// See [`get_cursor_id()`](get_cursor_id) for reasons why there is a small
    /// chance this method may be adjusted in future releases.
    ///
    /// # Return Values
    /// - Ok(())
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::NullError): a file path contains a 0-byte
    #[cfg(feature = "experimental")]
    pub fn seek_cursor_id(&self, cursor_id: String) -> Result<(), Error> {
        let c_cursor = CString::new(cursor_id).map_err(Error::NullError)?;
        let result = unsafe { ffi::sd_journal_seek_cursor(self.ffi, c_cursor.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Adds a match to filter journal entries (implements
    /// [`sd_journal_add_match()`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#)).
    ///
    /// Adds a filter on a field that will be applied on cursor movement
    /// thereafter. The filter must follow the format "FIELDNAME=fieldvalue".
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// journal.add_match("MESSAGE=Hello World!").unwrap();
    /// # assert_eq!(journal.next().unwrap(), CursorMovement::Done);
    /// while let Ok(CursorMovement::Done) = journal.next() {
    ///     // do something on the journal entries
    /// }
    /// ```
    ///
    /// # Return Values
    /// - Ok(()): done
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn add_match<T: AsRef<[c_uchar]>>(&self, filter: T) -> Result<(), Error> {
        let filter = filter.as_ref();
        let result = unsafe {
            ffi::sd_journal_add_match(self.ffi, filter.as_ptr() as *const c_void, filter.len())
        };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Adds a disjuntion marker to match definitions (implements
    /// [`sd_jounal_add_disjunction()`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#).)
    ///
    /// # Return Values
    /// - Ok(()): done
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn add_disjunction(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_add_disjunction(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Adds a conjuntion marker to match definitions (implements
    /// [`sd_jounal_add_conjunction()`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#).)
    ///
    /// # Return Values
    /// - Ok(()): done
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn add_conjunction(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_add_conjunction(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Flushes the match definition (implements
    /// [`sd_journal_flush_matches()`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#))
    pub fn flush_matches(&self) {
        unsafe { ffi::sd_journal_flush_matches(self.ffi) }
    }

    /// **UNSTABLE API** Determines the timestamps of the first and last entry
    /// in journal (implements [`sd_journal_get_cutoff_realtime_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_realtime_usec.html#)).
    ///
    /// # Stability
    /// Currently the function returns a chrono::NaiveDateTime calculated from
    /// the microseconds since EPOCH returned by the wrapped libsystemd
    /// function. In future releases this method may be changed to return
    /// microseconds (u128) or std::time::Duration although this is very
    /// unlikely. Changes will be made based on user feedback.
    ///
    /// # Return Values:
    /// - Ok((NaiveDateTime, NaiveDateTime)): (from, to) timestamps of the
    ///   journal
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn get_realtime_cutoff(&self) -> Result<(NaiveDateTime, NaiveDateTime), Error> {
        let mut from_usec: u64 = 0;
        let mut to_usec: u64 = 0;
        let result = unsafe {
            ffi::sd_journal_get_cutoff_realtime_usec(self.ffi, &mut from_usec, &mut to_usec)
        };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let from = NaiveDateTime::from_timestamp((from_usec / 1_000_000) as i64,
                                                 ((from_usec % 1_000_000) * 1_000) as u32);
        let to = NaiveDateTime::from_timestamp((to_usec / 1_000_000) as i64,
                                               ((to_usec % 1_000_000) * 1_000) as u32);
        Ok((from, to))
    }

    /// **UNSTABLE API** Determines the duration since boot of the first and
    /// last entry in journal for a specific boot id (implements
    /// [`sd_journal_get_cutoff_realtime_usec()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_monotonic_usec.html#)).
    ///
    /// # Stability
    /// Currently the function returns a chrono::Duration calculated from the
    /// microseconds since boot returned by the wrapped libsystemd function. The
    /// choice for chrono::Duration has been made based on the return value for
    /// `get_realtime()`. In future releases this method may be changed to
    /// microsenconds (u128) or std::time::Duration. Such change is reasonable
    /// likely and will be made based on user feedback.
    ///
    /// # Return Values
    /// - Ok((Duration, Duration)): (from, to) respective duration since boot
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn get_monotonic_cutoff(&self, boot_id: ID128) -> Result<(Duration, Duration), Error> {
        let mut from_usec: u64 = 0;
        let mut to_usec: u64 = 0;
        let result = unsafe {
            ffi::sd_journal_get_cutoff_monotonic_usec(self.ffi,
                                                      boot_id.into_ffi(),
                                                      &mut from_usec,
                                                      &mut to_usec)
        };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let from = Duration::seconds((from_usec / 1_000_000) as i64)
                   + Duration::microseconds((from_usec % 1_000_000) as i64);
        let to = Duration::seconds((to_usec / 1_000_000) as i64)
                 + Duration::microseconds((to_usec % 1_000_000) as i64);
        Ok((from, to))
    }

    /// Sets the data treshold limit for certain methods returning data fields
    /// (implements [`sd_journal_set_data_threshold()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// # Return Values
    /// - Ok(())
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn set_data_treshold(&self, size: size_t) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_set_data_threshold(self.ffi, size) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Gets the currently applied data treshold (implements
    /// [`sd_journal_get_data_threshold()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// # Return Values
    /// - Ok(size_t)
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn get_data_treshold(&self) -> Result<size_t, Error> {
        let mut size: size_t = 0;
        let result = unsafe { ffi::sd_journal_get_data_threshold(self.ffi, &mut size) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(size)
    }

    /// Enumerate the field names of the journal (implements
    /// [`sd_journal_enumerate_fields()`](https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#)).
    ///
    /// This method follows the principle of libsystemd to call this method
    /// repeatedly until you reach EoF. See [`iter_fields`](iter_fields)
    /// for a rustified iterator over fields.
    ///
    /// # Return Values
    /// - Ok(Enumeration::EoF): no more fields
    /// - Ok(Enumeration::Value(String)): field name
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(any(feature = "246", feature = "245", feature = "229"))]
    pub fn enumerate_field_names(&self) -> Result<Enumeration<String>, Error> {
        let mut field: *const c_char = ptr::null();
        let result = unsafe { ffi::sd_journal_enumerate_fields(self.ffi, &mut field) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(field).to_str()
                                 .map_err(Error::UTF8Error)?
                                 .to_owned()
        }))
    }

    /// Restart field enumeration (implements
    /// [`sd_journal_restart_fields()`](https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#)).
    #[cfg(any(feature = "246", feature = "245", feature = "229"))]
    pub fn restart_field_name_enumeration(&self) {
        unsafe { ffi::sd_journal_restart_fields(self.ffi) }
    }

    /// Get an iterator of the field names of the journal.
    ///
    /// This is the rustified version of `enumerate_field_names()`.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// // loop over field names and print them
    /// for fieldname in journal.iter_field_names() {
    ///     println!("{}", fieldname.unwrap());
    /// }
    /// ```
    pub fn iter_field_names<'a>(&'a self) -> FieldNames<'a> {
        FieldNames { journal: self }
    }

    /// Returns a read only file descriptor to be used in polling the journal
    /// (implements [`sd_journal_get_fd()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// # Return Values
    /// - Ok(RawFd)
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn get_fd(&self) -> Result<std::os::unix::io::RawFd, Error> {
        let result = unsafe { ffi::sd_journal_get_fd(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result)
    }

    /// Returns events to be used in polling the journal on the file descriptor
    /// (implements [`sd_journal_get_events()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// # Return Values
    /// - Ok(c_int): events to be used in polling the file descriptor
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn get_events(&self) -> Result<c_int, Error> {
        let result = unsafe { ffi::sd_journal_get_fd(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result)
    }

    /// Returns the timeout to be used in polling the journal on the file
    /// descriptor (implements
    /// [`sd_journal_get_timeout()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// # Return Values
    /// - Ok(u64): timeout
    /// - Err([Error::SDError](Error)): sd-journal returned an error code
    pub fn get_timeout(&self) -> Result<u64, Error> {
        let mut timeout: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_timeout(self.ffi, &mut timeout) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(timeout)
    }

    /// Processes events after each wake-up and returns the type of events
    /// (implements [`sd_journal_process()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// # Return Values
    /// - Ok(Event): journal wake event
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn process(&self) -> Result<Event, Error> {
        let result = unsafe { ffi::sd_journal_process(self.ffi) };
        match result {
            ffi::SD_JOURNAL_NOP => Ok(Event::NOOP),
            ffi::SD_JOURNAL_APPEND => Ok(Event::Append),
            ffi::SD_JOURNAL_INVALIDATE => Ok(Event::Invalidate),
            _ => Err(Error::SDError(result))
        }
    }

    /// Wait for changes in the journal for a maximum period defined in timeout
    /// (implements [`sd_journal_wait()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Use uint64_t-1 for timeout to wait indefinitely.
    ///
    /// # Return Values
    /// - Ok(Event): journal wake event
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn wait(&self, timeout: u64) -> Result<Event, Error> {
        let result = unsafe { ffi::sd_journal_wait(self.ffi, timeout) };
        match result {
            ffi::SD_JOURNAL_NOP => Ok(Event::NOOP),
            ffi::SD_JOURNAL_APPEND => Ok(Event::Append),
            ffi::SD_JOURNAL_INVALIDATE => Ok(Event::Invalidate),
            _ => Err(Error::SDError(result))
        }
    }

    /// Checks whether the journal owns runtime files (implements
    /// [`sd_journal_has_runtime_files()`](https://www.freedesktop.org/software/systemd/man/sd_journal_has_runtime_files.html#)).
    ///
    /// # Return Values
    /// - Ok(bool)
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(any(feature = "246", feature = "245", feature = "229"))]
    pub fn has_runtime_files(&self) -> Result<bool, Error> {
        let result = unsafe { ffi::sd_journal_has_runtime_files(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result > 0)
    }

    /// Checks whether the journal owns persistent files (implements
    /// [`sd_journal_has_persistent_files()`](https://www.freedesktop.org/software/systemd/man/sd_journal_has_persistent_files.html#)).
    ///
    /// # Return Values
    /// - Ok(bool)
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(any(feature = "246", feature = "245", feature = "229"))]
    pub fn has_persistent_files(&self) -> Result<bool, Error> {
        let result = unsafe { ffi::sd_journal_has_persistent_files(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result > 0)
    }

    /// Determines the disk space used by the journal in Bytes
    /// (implements [`sd_journal_get_usage()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_usage.html#)).
    ///
    /// # Stability
    /// Currently a plain u64 is returned. There is a reasonable chance for a
    /// change in future releases towards a SI unit type.
    ///
    /// # Return Values
    /// - Ok(u64): space required in Bytes
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn get_usage(&self) -> Result<u64, Error> {
        let mut usage: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_usage(self.ffi, &mut usage) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(usage)
    }

    /// **UNSTABLE API** Retrieves the realtime timestamp as
    /// chrono::NaiveDateTime of the current record (implements
    /// [`sd_journal_get_realtime_usec()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#)).
    ///
    /// # Stability
    /// Currently the function returns a chrono::NaiveDateTime calculated from
    /// the microseconds since EPOCH returned by the wrapped libsystemd
    /// function. In future releases this method may be changed to return
    /// microseconds (u128) or std::time::Duration although this is very
    /// unlikely. Changes will be made based on user feedback.
    ///
    /// # Return Values:
    /// - Ok(NaiveDateTime): realtime timestamp of current record
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn get_realtime(&self) -> Result<NaiveDateTime, Error> {
        let mut usec: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_realtime_usec(self.ffi, &mut usec) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let dt = NaiveDateTime::from_timestamp((usec / 1_000_000) as i64,
                                               ((usec % 1_000_000) * 1_000) as u32);
        Ok(dt)
    }

    /// **UNSTABLE API** Retrieves the monotonic timestamp of the current record
    /// altogether with it's boot id (implements [`sd_journal_get_monotonic_usec()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#)).
    ///
    /// # Stability
    /// Currently the function returns a chrono::Duration calculated from the
    /// microseconds since boot returned by the wrapped libsystemd function. The
    /// choice for chrono::Duration has been made based on the return value for
    /// `get_realtime()`. In future releases this method may be changed to
    /// microsenconds (u128) or std::time::Duration. Such change is reasonable
    /// likely and will be made based on user feedback.
    ///
    /// # Return Values
    /// - Ok(chrono::Duration, ID128): tuple of a monotonic timestamp since boot
    ///   and boot id
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "td_chrono")]
    #[cfg(feature = "experimental")]
    pub fn get_monotonic(&self) -> Result<(Duration, sd_id128::ID128), Error> {
        let mut usec: u64 = 0;
        let mut boot_id = ID128::default().into_ffi();
        let result =
            unsafe { ffi::sd_journal_get_monotonic_usec(self.ffi, &mut usec, &mut boot_id) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let duration = Duration::seconds((usec / 1_000_000) as i64)
                       + Duration::microseconds((usec % 1_000_000) as i64);
        Ok((duration, ID128::from_ffi(boot_id)))
    }

    /// **UNSTABLE API** Retrieve a text representation of the cursor
    /// (implements [`sd_journal_get_cursor()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#)).
    ///
    /// # Stability
    /// `sd_journal_get_cursor()` returns a ownership of a memory location.
    /// Currently the content is copied into a rustified String and the memory
    /// freed immediately. In future releases a new data type could be defined
    /// which avoids the immediate conversion into a String. The new data type
    /// could be handed over into `seek_cursor()`. The chance for such change is
    /// low. The decission will be taken based on typical usage scenarios and
    /// user feedback.
    ///
    /// # Return values
    /// - Ok(String): cursor representation of sd-journal
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured; although this
    ///   should never happen since the journal internal cursor id is stored in
    ///   valid UTF-8
    #[cfg(feature = "experimental")]
    pub fn get_cursor_id(&self) -> Result<String, Error> {
        let mut ptr: *mut c_char = ptr::null_mut();
        let result = unsafe { ffi::sd_journal_get_cursor(self.ffi, &mut ptr) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let cursor_id = unsafe { CStr::from_ptr(ptr) };
        let cursor_id = match cursor_id.to_str() {
            Err(error) => {
                unsafe { libc::free(ptr as *mut c_void) };
                Err(Error::UTF8Error(error))?
            },
            Ok(value) => value.to_owned()
        };
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(cursor_id)
    }

    /// **UNSTABLE API** Checks whether the current journal position matches a
    /// cursor id (implements [`sd_journal_get_cursor`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#)).
    ///
    /// # Stability
    /// See [`get_cursor_id()`](get_cursor_id) for reasons why there is a small
    /// chance this method may be adjusted in future releases.
    ///
    /// # Return Values
    /// - Ok(bool)
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(feature = "experimental")]
    pub fn cursor_id_matches<S: Into<Vec<u8>>>(&self, cursor_id: S) -> Result<bool, Error> {
        let c_cursor = CString::new(cursor_id).map_err(Error::NullError)?;
        let result = unsafe { ffi::sd_journal_test_cursor(self.ffi, c_cursor.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result > 0)
    }

    /// Determine the message cataloge entry for the current record (implements
    /// [`sd_journal_get_catalog()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#)).
    ///
    /// # Return Values
    /// - Ok(String): message catalogue
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    pub fn get_catalog(&self) -> Result<String, Error> {
        let mut data: *mut c_char = ptr::null_mut();
        let result = unsafe { ffi::sd_journal_get_catalog(self.ffi, &mut data) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let catalog = unsafe { CStr::from_ptr(data) };
        let catalog = match catalog.to_str() {
            Err(error) => {
                unsafe { libc::free(data as *mut c_void) };
                Err(Error::UTF8Error(error))?
            },
            Ok(value) => value.to_owned()
        };
        unsafe { libc::free(data as *mut c_void) };
        Ok(catalog)
    }

    /// Retrieve the data of a specific field (implements
    /// [`sd_journal_get_data()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Retrieve the data of a specific field. The fieldname must be provided in
    /// all upper case letters. See the documentation of well-known
    /// [field names](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html#).
    /// Field names may not contain 0x00 bytes (would raise a NullError). If the
    /// current entry does not contain the field, an SDError(-2) is returned.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// // loop over the journal and print the timestamp and message of each record
    /// for cursor in &journal {
    ///     let cursor = cursor.unwrap();
    ///     let message = cursor.get_data("MESSAGE")
    ///                         .unwrap_or("[no message available]".to_string());
    ///     let datetime = cursor.get_realtime().unwrap();
    ///     println!("{} - {}", datetime, message);
    /// }
    /// ```
    ///
    /// # Return values
    /// - Ok(String): value in the format FIELDNAME=FIELDVALUE
    /// - Err(Error::NullError): the requested field name contains 0-bytes
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::UnexpectedDataFormat): libsystemd is expected to return
    ///   data in the format `FIELDNAME=field value`. Before returning that
    ///   data, `FIELDNAME=` are stripped of. If that operation fails, this
    ///   error is raised.
    /// - Err(Error::StringError): if this error is raised, please report an
    ///   issue against sd_journal
    pub fn get_data<F: Into<Vec<u8>>>(&self, field: F) -> Result<String, Error> {
        let c_field = CString::new(field).map_err(Error::NullError)?;
        let mut data: *const c_void = std::ptr::null_mut();
        let mut length: size_t = 0;
        let result =
            unsafe { ffi::sd_journal_get_data(self.ffi, c_field.as_ptr(), &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let result = unsafe {
            CStr::from_ptr(data as *mut c_char).to_str()
                                               .map_err(Error::UTF8Error)?
        };

        let field = c_field.into_string().map_err(Error::StringError)?;
        let result = match result.strip_prefix(&field) {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(value) => match value.strip_prefix('=') {
                None => Err(Error::UnexpectedDataFormat)?,
                Some(value) => value
            }
        };
        Ok(result.to_string())
    }

    /// Enumerate the fields of the current record (implements
    /// [`sd_journal_enumerate_data()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// This is the libsystemd way of iterating over fields. There is also a
    /// rustified alternative method [`iter_fields()`](Journal::iter_fields).
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// # journal.next().unwrap();
    /// // loop over all fields of the current record and print FIELDNAME: field value
    /// while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
    ///     println!("{}: {}", field, value)
    /// }
    /// ```
    ///
    /// # Return values
    /// - Ok(Enumeration::Value(String, String)): field name and value
    /// - Ok(Enumeration::EoF): no more fields to enumerate
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::UnexpectedDataFormat): libsystemd is expected to return
    ///   data in the format `FIELDNAME=field value`. Field name and value are
    ///   separated at the `=`. If the format does not match, this error is
    ///   raised.
    pub fn enumerate_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result = unsafe { ffi::sd_journal_enumerate_data(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        let result = unsafe {
            CStr::from_ptr(data as *const c_char).to_str()
                                                 .map_err(Error::UTF8Error)?
                                                 .to_owned()
        };
        let (field, value) = match result.find('=') {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(index) => result.split_at(index)
        };
        let value = match value.strip_prefix('=') {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(value) => value
        };
        Ok(Enumeration::Value((field.to_owned(), value.to_owned())))
    }

    /// Enumerate the available & supported fields of the current record
    /// (implements [`sd_journal_enumerate_available_data()`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// # Return values
    /// - Ok(Enumeration::Value(String, String)): field name and value
    /// - Ok(Enumeration::EoF): no more fields to enumerate
    /// - Err(Error::SDError): sd-journal returned an error code
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::UnexpectedDataFormat): libsystemd is expected to return
    ///   data in the format `FIELDNAME=field value`. Field name and value are
    ///   separated at the `=`. If the format does not match, this error is
    ///   raised.
    #[cfg(any(feature = "246"))]
    pub fn enumerate_available_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result =
            unsafe { ffi::sd_journal_enumerate_available_data(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        let result = unsafe {
            CStr::from_ptr(data as *const c_char).to_str()
                                                 .map_err(Error::UTF8Error)?
                                                 .to_owned()
        };
        let (field, value) = match result.find('=') {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(index) => result.split_at(index)
        };
        let value = match value.strip_prefix('=') {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(value) => value
        };
        Ok(Enumeration::Value((field.to_owned(), value.to_owned())))
    }

    /// Restart enumeration of fields (implements
    /// [`sd_journal_restart_data`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    pub fn restart_fields_enumeration(&self) {
        unsafe {
            ffi::sd_journal_restart_data(self.ffi);
        }
    }

    /// Returns an iterator over the fields of the current records.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// # journal.next().unwrap();
    /// // The following 2 loops are synonyms
    /// while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
    ///     println!("{}: {}", field, value);
    /// }
    /// for field in journal.iter_fields() {
    ///     let (field, value) = field.unwrap();
    ///     println!("{}: {}", field, value);
    /// }
    /// ```
    pub fn iter_fields<'a>(&'a self) -> Fields<'a> {
        Fields { journal: self }
    }

    /// Query the journal for unique field values of a certain field (implements
    /// [`sd_journal_query_unique()`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// # libsystemd Issues
    /// `sd_journal_query_unique()` and the related functions do not always
    /// succeed to return **unique** values, i.e. a value may be returned
    /// repeatedly. An [issue](https://github.com/systemd/systemd/issues/18075)
    /// has been reported.
    ///
    /// # Return Values
    /// - Ok(())
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn query_unique_values<S: Into<Vec<u8>>>(&self, field: S) -> Result<(), Error> {
        let c_field = CString::new(field).map_err(Error::NullError)?;
        let result = unsafe { ffi::sd_journal_query_unique(self.ffi, c_field.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Enumerate all unique values for the field requested (implements
    /// [`sd_journal_enumerate_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// Return Values
    /// - Ok(Enumeration::Value(String)): value
    /// - Ok(Enumeration::EoF): no more unique values to enumerate
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn enumerate_unique_values(&self) -> Result<Enumeration<String>, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result = unsafe { ffi::sd_journal_enumerate_unique(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        let result = unsafe {
            CStr::from_ptr(data as *const c_char).to_str()
                                                 .map_err(Error::UTF8Error)?
        };
        let index = match result.find('=') {
            None => Err(Error::UnexpectedDataFormat)?,
            Some(index) => index
        };
        let (_, result) = result.split_at(index + 1);
        Ok(Enumeration::Value(result.to_owned()))
    }

    /// Enumerate available unique values for the field requested (implements
    /// [`sd_journal_enumerate_available_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// Return Values:
    /// - Ok(Enumeration::Value(String)): value
    /// - Ok(Enumeration::EoF): no more unique values to enumerate
    /// - Err(Error::UTF8Error): UTF-8 decoding error occured
    /// - Err(Error::SDError): sd-journal returned an error code
    #[cfg(any(feature = "246"))]
    pub fn enumerate_available_unique_values(&self) -> Result<Enumeration<String>, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result =
            unsafe { ffi::sd_journal_enumerate_available_unique(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(data as *const c_char).to_str()
                                                 .map_err(Error::UTF8Error)?
                                                 .to_owned()
        }))
    }

    /// Restart enumeration of unique values (implements
    /// [`sd_journal_restart_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    pub fn restart_unique_value_enumeration(&self) {
        unsafe { ffi::sd_journal_restart_unique(self.ffi) }
    }

    /// Returns an iterator over unique values of a field.
    ///
    /// # Examples
    /// ```
    /// # use sd_journal::*;
    /// # use std::path::PathBuf;
    /// # let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// # test_data.push("test-data/");
    /// # println!("looking for test data in folder {}", test_data.display());
    /// # let journal = Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    /// for value in journal.iter_unique_values("MESSAGE").unwrap() {
    ///     let value = value.unwrap();
    ///     println!("{}", value);
    /// }
    /// ```
    ///
    /// # Return Values
    /// - Ok(UniqueValues)
    /// - Err(Error::SDError): sd-journal returned an error code
    pub fn iter_unique_values<'a, S: Into<Vec<u8>>>(&'a self,
                                                    field: S)
                                                    -> Result<UniqueValues<'a>, Error> {
        self.query_unique_values(field)?;
        Ok(UniqueValues { journal: &self })
    }
}

impl<'a> Cursor<'a> {
    /// see [Journal::get_realtime](Journal::get_realtime)
    pub fn get_realtime(&self) -> Result<NaiveDateTime, Error> {
        self.journal.get_realtime()
    }

    /// see [Journal::get_monotonic](Journal::get_monotonic)
    pub fn get_monotonic(&self) -> Result<(Duration, sd_id128::ID128), Error> {
        self.journal.get_monotonic()
    }

    /// see [Journal::get_cursor_id](Journal::get_cursor_id)
    pub fn get_id(&self) -> Result<String, Error> {
        self.journal.get_cursor_id()
    }

    /// see [Journal::cursor_id_matches](Journal::cursor_id_matches)
    pub fn id_matches<S: Into<Vec<u8>>>(&self, cursor_id: S) -> Result<bool, Error> {
        self.journal.cursor_id_matches(cursor_id)
    }

    /// see [Journal::get_catalog](Journal::get_catalog)
    pub fn get_catalog(&self) -> Result<String, Error> {
        self.journal.get_catalog()
    }

    /// see [Journal::get_data](Journal::get_data)
    pub fn get_data<F: Into<Vec<u8>>>(&self, field: F) -> Result<String, Error> {
        self.journal.get_data(field)
    }

    /// see [Journal::enumerate_fields](Journal::enumerate_fields)
    pub fn enumerate_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        self.journal.enumerate_fields()
    }

    /// see [Journal::enumerate_available_fields](Journal::
    /// enumerate_available_fields)
    pub fn enumerate_available_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        self.journal.enumerate_available_fields()
    }

    /// see [Journal::restart_fields_enumeration](Journal::
    /// restart_fields_enumeration)
    pub fn restart_fields_enumeration(&self) {
        self.journal.restart_fields_enumeration()
    }

    /// see [Journal::iter_fields](Journal::iter_fields)
    pub fn iter_fields(&self) -> Fields<'a> {
        self.journal.iter_fields()
    }
}
