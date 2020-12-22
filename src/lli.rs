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
use super::*;
use sd_id128::ID128;
use std::ffi::CStr;
/// A low level wrapper for sd-journal as offered by libsystemd based on FFI
/// bindings offered in crate [sd-sys](https://gitlab.com/systemd.rs/sd-sys).
///
/// Journal is a fully implemented , low level (bare metal) wrapper for
/// submitting and querying log entries from the systemd journal. It is
/// recommended to use the high level wrapper offered in the root of the crate.
/// This low level wrapper is rather to be used for very special requirements
/// that are not offered by the high level wrapper. This low level wrapper may
/// be accessed from the high level wrapper.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Journal {
    ffi: *mut ffi::sd_journal
}

impl Journal {
    /// Print a simple message with a chosen priority to the journal (implements
    /// [`sd_journal_print`](<https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>)).
    ///
    /// Submits a simple, plain text log entry to the system wide journal, e.g.
    /// ```
    /// # use std::ffi::CString;
    /// use sd_journal::*;
    /// lli::Journal::print(Level::Info, &CString::new("Hello World!").unwrap());
    /// ```
    ///
    /// Parameters:
    /// - Level: The priority value is one as defined in syslog.h
    /// - Message: log message
    ///
    /// Return Values:
    /// - Ok(): success
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn print(level: Level, message: &CStr) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_print(level as c_int, message.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Send a raw log record to the journal (implements
    /// [`sd_journal_sendv`](<https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>))
    ///
    /// This method may be used to submit structured log entries to the system
    /// journal. It takes a slice or a vector of strings (String or &str). The
    /// strings passed should be of the format "VARIABLE=value". A call like
    /// `lli::Journal::sendv(&["PRIORITY=6", "MESSAGE=Hello World!"])` is a
    /// synonym for `lli::Journal::print(Level::Info, "Hello World!");`
    ///
    /// The variable name must be in uppercase and consist only of characters,
    /// numbers and underscores, and may not begin with an underscore. (All
    /// assignments that do not follow this syntax will silently be ignored.)
    /// The value can be of any size and format, i.e. the data encoding may
    /// differ from UTF8 and may contain binary coding. A number of
    /// well-known fields are defined, see [here](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html#)
    /// for details, but additional application defined fields may be used. A
    /// variable may be assigned more than one value per entry.
    ///
    /// Examples
    /// ```
    /// use sd_journal::*;
    /// # use std::ffi::CString;
    /// lli::Journal::sendv(&["PRIORITY=6", "MESSAGE=Hello World!"]).unwrap();
    /// lli::Journal::sendv(&["PRIORITY=6".to_string(), "MESSAGE=Hello World!".to_string()]).unwrap();
    /// lli::Journal::sendv(&Vec::from(["PRIORITY=6", "MESSAGE=Hello World!"])).unwrap();
    /// lli::Journal::sendv(&Vec::from(["PRIORITY=6".to_string(),
    ///                                 "MESSAGE=Hello World!".to_string(),
    ///                                 "CODE_FUNC=log_structured_raw_entry()".to_string(),
    ///                                 "CODE_LINE=34".to_string(),
    ///                                 "CODE_FILE=tests/lib.rs".to_string()])).unwrap();
    /// ```
    ///
    /// Parameters:
    /// - data: any slice of byte slices, e.g. &[&str], Vec<String>, ...
    ///
    /// Return Values:
    /// - Ok(): success
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn sendv<T: AsRef<[u8]>>(data: &[T]) -> Result<(), Error> {
        let iovec: Vec<iovec> = data.iter()
                                    .map(|field| iovec { iov_base: field.as_ref().as_ptr()
                                                                   as *mut c_void,
                                                         iov_len:  field.as_ref().len() })
                                    .collect();
        let result = unsafe { ffi::sd_journal_sendv(iovec.as_ptr(), iovec.len() as c_int) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Open a journal for read access (implements
    /// [`sd_journal_open`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Opens the log journal for reading. It will find all journal files
    /// and interleave them automatically when reading.
    ///
    /// Examples
    /// ```
    /// use sd_journal::*;
    /// # use std::ffi::CString;
    /// lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    /// lli::Journal::open(FileFlags::LocalOnly, UserFlags::CurrentUserOnly).unwrap();
    ///  ```
    ///
    /// Parameters:
    /// - FileFlags: which journal files to load: no restriction vs. local files
    ///   only vs. runtime only
    /// - UserFlags: which journal files to load: no restriction vs. system
    ///   journal only vs. current user journal only
    ///
    /// Return values:
    /// - Ok(Journal): initialized journal
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
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
    /// [`sd_journal_open_namespace`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Opens the log journal for reading on a selected
    /// [namespace](https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html#Journal%20Namespaces)
    /// only. It will find all journal files and interleave them automatically
    /// when reading. This method does not support the
    /// `SD_JOURNAL_ALL_NAMESPACES` flag. If you want to open all namespaces,
    /// see `open_all_namespaces`.
    ///
    /// Parameters:
    /// - Namespace: a &str to the namespace to open
    /// - NamespaceFlags: whether to include the default namespace or only
    ///   access to defined namespace
    /// - FileFlags: which journal files to load: no restriction vs. local files
    ///   only vs. runtime only
    /// - UserFlags: which journal files to load: no restriction vs. system
    ///   journal only vs. current user journal only
    ///
    /// Return values:
    /// - Ok(Journal):  initialized journal
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn open_namespace(namespace: &CStr,
                          namespace_flags: NamespaceFlags,
                          file_flags: FileFlags,
                          user_flags: UserFlags)
                          -> Result<Journal, Error>
    {
        let mut pointer = ptr::null_mut() as *mut ffi::sd_journal;
        let flags = file_flags as c_int | user_flags as c_int | namespace_flags as c_int;
        let result =
            unsafe { ffi::sd_journal_open_namespace(&mut pointer, namespace.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Open the journal for read access including all available namespaces
    /// (implements
    /// [`sd_journal_open_namespace`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)
    /// with flag SD_JOURNAL_ALL_NAMESPACES set).
    ///
    /// Opens the log journal for reading for all
    /// [namespaces](https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html#Journal%20Namespaces).
    /// It will find all journal files automatically and interleave
    /// them automatically when reading.
    ///
    /// Parameters:
    /// - FileFlags: which journal files to load: no restriction vs. local files
    ///   only vs. runtime only
    /// - UserFlags: which journal files to load: no restriction vs. system
    ///   journal only vs. current user journal only
    ///
    /// Return values:
    /// - Ok(Journal):  initialized journal
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn open_all_namespaces(file_flags: FileFlags,
                               user_flags: UserFlags)
                               -> Result<Journal, Error>
    {
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
    /// [`sd_journal_open_directory`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Open the journal located at a specific path: takes an *absolute*
    /// directory path as argument. All journal files in this directory
    /// will be opened and interleaved automatically.
    ///
    /// Parameters:
    /// - Path: absolute directory path
    /// - PathFlags: whether the path contains the full path or points to root
    ///   of an OS. In the latter journal files are searched for below the usual
    ///   /var/log/journal and /run/log/journal relative to the specified path,
    ///   instead of directly beneath it
    /// - UserFlags: which journal files to load: no restriction vs. system
    ///   journal only vs. current user journal only
    ///
    /// Return values:
    /// - Ok(Journal): initialized journal
    /// - Err(UTF8Error): the path contains non-UTF8 characters which are not
    ///   supported
    /// - Err(NullError): the path contains a 0-byte
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn open_directory(path: &CStr,
                          path_flags: PathFlags,
                          user_flags: UserFlags)
                          -> Result<Journal, Error>
    {
        let mut pointer = ptr::null_mut() as *mut ffi::sd_journal;
        let flags = path_flags as c_int | user_flags as c_int;
        let result = unsafe { ffi::sd_journal_open_directory(&mut pointer, path.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Open the journal stored in a list of files (implements
    /// [`sd_journal_open_files`](https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#)).
    ///
    /// Parameters:
    /// - files: absolute directory path to files in a &[&CStr]
    ///
    /// Return values:
    /// - Ok(Journal): the journal
    /// - Err(UTF8Error): the path contains non-UTF8 characters which are not
    ///   supported
    /// - Err(NullError): the path contains a 0-byte
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn open_files<T: AsRef<CStr>>(files: &[T]) -> Result<Journal, Error> {
        let files_vec: Vec<*const c_char> =
            files.iter().map(|file| file.as_ref().as_ptr()).collect();
        let mut pointer = std::ptr::null_mut() as *mut ffi::sd_journal;
        let flags: c_int = 0;
        let result = unsafe { ffi::sd_journal_open_files(&mut pointer, files_vec.as_ptr(), flags) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let journal = Journal { ffi: pointer };
        Ok(journal)
    }

    /// Advance the read pointer of the journal by one entry (implements
    /// [`sd_journal_next`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// Return values:
    /// - Done: full success
    /// - EoF: no movement was executed, since the cursor is already placed at
    ///   EoF.
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
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

    /// Set back the read pointer of the journal by one entry (implements
    /// [`sd_journal_previous`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// Return values:
    /// - Done: full success
    /// - Limited(actual): the movement was executed but limited by the EoF of
    ///   the journal. The actual movement is given in the parameter.
    /// - EoF: no movement was executed, since the cursor is already placed at
    ///   EoF. This is an equivalent to Limited(0).
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
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

    /// Advance the read pointer of the journal by multiple entries (implements
    /// [`sd_journal_next_skip`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// Parameters:
    /// skip: how many records to skip (0 or positive number)
    ///
    /// Return values:
    /// -Done: full success
    /// - Limited(actual): the movement was executed but limited by the EoF of
    ///   the journal. The actual movement is given in the parameter.
    /// - EoF: no movement was executed, since the cursor is already placed at
    ///   EoF. This is an equivalent to Limited(0).
    /// - Error(SDError): sd-journal returned an error code
    /// - Error(RangeError): skip must be 0 or a positive number
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
    /// [`sd_journal_previous_skip`](https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#)).
    ///
    /// Parameters:
    /// - skip: how many records to skip (0 or positive number)
    ///
    /// Return values:
    /// - Done: full success
    /// - Limited(actual): the movement was executed but limited by the EoF of
    ///   the journal. The actual movement is given in the parameter.
    /// - EoF: no movement was executed, since the cursor is already placed at
    ///   EoF. This is an equivalent to Limited(0).
    /// - Error(SDError): sd-journal returned an error code
    /// - Error(RangeError): skip must be 0 or a positive number
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

    /// Retrieves the realtime timestamp of the current record (implements
    /// [`sd_journal_get_realtime_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#)).
    ///
    /// Return Values:
    /// - Ok(u64): Microseconds since UNIX-EPOCH
    /// - Error(SDError): sd-journal returned an error code
    pub fn get_realtime_usec(&self) -> Result<u64, Error> {
        let mut usec: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_realtime_usec(self.ffi, &mut usec) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(usec)
    }

    /// Retrieves the monotonic timestamp of the current record altogether with
    /// it's boot id (implements [`sd_journal_get_monotonic_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#)).
    ///
    /// Return Values:
    /// - Ok(u64, ID128): monotonic timestamp in microseconds since boot and
    ///   boot id
    /// - Error(SDError): sd-journal returned an error code
    pub fn get_monotonic_usec(&self) -> Result<(u64, sd_id128::ID128), Error> {
        let mut usec: u64 = 0;
        let mut boot_id = ID128::default().into_ffi();
        let result =
            unsafe { ffi::sd_journal_get_monotonic_usec(self.ffi, &mut usec, &mut boot_id) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok((usec, ID128::from_ffi(boot_id)))
    }

    /// Adds a match to filter journal entries (implements
    /// [`sd_journal_add_match`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#)).
    ///
    /// Adds a filter on a field that will be applied on cursor movement
    /// thereafter. The filter must follow the format "FIELDNAME=fieldvalue".
    ///
    /// Parameter:
    /// - &[c_char]: Filter to apply
    ///
    /// Return Values:
    /// - Ok(()): done
    /// - Error(SDError): sd-journal returned an error code
    pub fn add_match(&self, filter: &[c_uchar]) -> Result<(), Error> {
        let result = unsafe {
            ffi::sd_journal_add_match(self.ffi, filter.as_ptr() as *const c_void, filter.len())
        };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Adds a disjuntion marker to match definitions (implements
    /// [`sd_jounal_add_disjunction`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#).)
    ///
    /// Return Value:
    /// - Ok(()): done
    /// - Error(SDError): sd-journal returned an error code
    pub fn add_disjunction(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_add_disjunction(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Adds a conjuntion marker to match definitions (implements
    /// [`sd_jounal_add_conjunction`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#).)
    ///
    /// Return Value:
    /// - Ok(()): done
    /// - Error(SDError): sd-journal returned an error code
    pub fn add_conjunction(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_add_conjunction(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Flushes the match definition (implements
    /// [`sd_journal_flush_matches`](https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#))
    pub fn flush_matches(&self) {
        unsafe { ffi::sd_journal_flush_matches(self.ffi) }
    }

    /// Seek to the head of the journal (implements
    /// [`sd_journal_seek_head`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seek to the beginning of the journal, i.e. to the position *before* the
    /// oldest available entry.
    /// Return values:
    /// - Ok(()): success
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
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
    /// available entry.
    ///
    /// Return values:
    /// - Ok(()): success
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn seek_tail(&self) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_seek_tail(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Seek to a monotonic timestamp of a certain boot id (implements
    /// [`sd_journal_seek_monotonic_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seek to a position with the specified monotonic timestamp, i.e.
    /// `clockMonotonic'. Since monotonic time restarts on every reboot a
    /// boot ID needs to be specified as well
    ///
    /// Parameters:
    /// - bootID: ID128 of a boot
    /// - clockMonotonic: 1.000.000 clock ticks per second since start of boot
    ///
    /// Return values:
    /// - Ok(())
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn seek_monotonic_usec(&self, boot_id: ID128, clock_monotonic: u64) -> Result<(), Error> {
        let ffi = boot_id.into_ffi();
        let result = unsafe { ffi::sd_journal_seek_monotonic_usec(self.ffi, ffi, clock_monotonic) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Seek to realtime timestamp (implements
    /// [`sd_journal_seek_realtime_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seeks to a position with the specified realtime (wallclock) timestamp,
    /// i.e. 'clockRealtime'. Note that the realtime clock is not necessarily
    /// monotonic. If a realtime timestamp is ambiguous, it is not defined which
    /// position is sought to.
    ///
    /// Parameters:
    /// - clockRealtime: 1.000.000 clock ticks per second since Jan 1st 1970
    ///
    /// Return values:
    /// - Ok(())
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn seek_realtime_usec(&self, clock_realtime: u64) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_seek_realtime_usec(self.ffi, clock_realtime) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Seek to cursor (implements
    /// [`sd_journal_seek_cursor`](https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#)).
    ///
    /// Seeks the journal to the position of the cursor provided.
    ///
    /// Parameters:
    /// - [Cursor](crate::Cursor)
    ///
    /// Return Values:
    /// - Ok(())
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn seek_cursor(&self, cursor: &Cursor) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_seek_cursor(self.ffi, cursor.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Enumerate fields of the journal (implements
    /// [`sd_journal_enumerate_fields`](https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#)).
    ///
    /// Return Values:
    /// - Ok([Enumeration::EoF](crate::Enumeration)): no more fields
    /// - Ok([Value(String)](crate::Enumeration)): field name
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn enumerate_fields(&self) -> Result<Enumeration, Error> {
        let mut field: *const c_char = ptr::null();
        let result = unsafe { ffi::sd_journal_enumerate_fields(self.ffi, &mut field) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(field).to_owned()
        }))
    }

    /// Restart field enumeration (implements
    /// [`sd_journal_restart_fields`](https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#)).
    pub fn restart_fields(&self) {
        unsafe { ffi::sd_journal_restart_fields(self.ffi) }
    }

    /// Retrieve a text representation of the cursor (implements
    /// [`sd_journal_get_cursor`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#)).
    ///
    /// Return Values:
    /// - Ok([Cursor](crate::Cursor))
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_cursor(&self) -> Result<Cursor, Error> {
        let mut cursor: *mut c_char = ptr::null_mut();
        let result = unsafe { ffi::sd_journal_get_cursor(self.ffi, &mut cursor) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(Cursor { ffi: cursor })
    }

    /// Checks whether the current journal position matches a cursor (implements
    /// [`sd_journal_get_cursor`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#)).
    ///
    /// Return Values:
    /// - Ok([CursorCheck](crate::CursorCheck))
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn test_cursor(&self, cursor: &Cursor) -> Result<CursorCheck, Error> {
        let result = unsafe { ffi::sd_journal_test_cursor(self.ffi, cursor.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(CursorCheck::DoesNotMatch);
        }
        Ok(CursorCheck::Matches)
    }

    /// Determines the timestamps of the first and last entry in journal
    /// (implements [`sd_journal_get_cutoff_realtime_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_realtime_usec.html#)).
    ///
    /// Return Values:
    /// - Ok((u64, u64)): from - to timestamps of the journal
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_cutoff_realtime_usec(&self) -> Result<(u64, u64), Error> {
        let mut from: u64 = 0;
        let mut to: u64 = 0;
        let result =
            unsafe { ffi::sd_journal_get_cutoff_realtime_usec(self.ffi, &mut from, &mut to) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok((from, to))
    }

    /// Determines the timestamps of the first and last entry in journal
    /// for a specific boot id (implements
    /// [`sd_journal_get_cutoff_realtime_usec`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_monotonic_usec.html#)).
    ///
    /// Parameter:
    /// - [ID128](sd_id128::ID128): boot id
    ///
    /// Return Values:
    /// - Ok((u64, u64)): from - to timestamps of the journal
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_cutoff_monotonic_usec(&self, boot_id: ID128) -> Result<(u64, u64), Error> {
        let mut from: u64 = 0;
        let mut to: u64 = 0;
        let result = unsafe {
            ffi::sd_journal_get_cutoff_monotonic_usec(self.ffi,
                                                      boot_id.into_ffi(),
                                                      &mut from,
                                                      &mut to)
        };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok((from, to))
    }

    /// Determines the disk space used by the journal (implements
    /// [`sd_journal_get_usage`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_usage.html#)).
    ///
    /// Return Values:
    /// - Ok(u64): space required in Bytes
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_usage(&self) -> Result<u64, Error> {
        let mut usage: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_usage(self.ffi, &mut usage) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(usage)
    }

    /// Determine the message cataloge entry for the current record (implements
    /// [`sd_journal_get_catalog`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#)).
    ///
    /// Return Values:
    /// - Ok(CString): message catalogue
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    ///
    /// TODO: setup a new struct holding the reference to the memory
    /// implementing DROP so we do not need to copy the data (the struct may be
    /// used on other methods as well)
    pub fn get_catalog(&self) -> Result<CString, Error> {
        let mut data: *mut c_char = ptr::null_mut();
        let result = unsafe { ffi::sd_journal_get_catalog(self.ffi, &mut data) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let string = unsafe { CStr::from_ptr(data).to_owned() };
        unsafe { libc::free(data as *mut c_void) };
        Ok(string)
    }

    /// Determine the message cataloge entry for a message id (implements
    /// [`sd_journal_get_catalog_for_message_id`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#)).
    ///
    /// Parameter:
    /// - [ID128](sd_id128::ID128): message id
    ///
    /// Return Values:
    /// - Ok(String): message catalogue
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    /// TODO: setup a new struct holding the reference to the memory
    /// implementing DROP so we do not need to copy the data (the struct may be
    /// used on other methods as well)
    pub fn get_catalog_for_message_id(id: ID128) -> Result<CString, Error> {
        let mut data: *mut c_char = ptr::null_mut();
        let result =
            unsafe { ffi::sd_journal_get_catalog_for_message_id(id.into_ffi(), &mut data) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        let string = unsafe { CStr::from_ptr(data).to_owned() };
        unsafe { libc::free(data as *mut c_void) };
        return Ok(string);
    }

    /// Returns a read only file descriptor to be used in polling the journal
    /// (implements [`sd_journal_get_fd`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Return Values:
    /// - Ok(RawFd)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_fd(&self) -> Result<c_int, Error> {
        let result = unsafe { ffi::sd_journal_get_fd(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result)
    }

    /// Returns events to be used in polling the journal on the file descriptor
    /// (implements [`sd_journal_get_events`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Return Values:
    /// - Ok(c_int): events to be used in polling the file descriptor
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_events(&self) -> Result<c_int, Error> {
        let result = unsafe { ffi::sd_journal_get_fd(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result)
    }

    /// Returns the timeout to be used in polling the journal on the file
    /// descriptor (implements
    /// [`sd_journal_get_timeout`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Return Values:
    /// - Ok(u64): timeout
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_timeout(&self) -> Result<u64, Error> {
        let mut timeout: u64 = 0;
        let result = unsafe { ffi::sd_journal_get_timeout(self.ffi, &mut timeout) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(timeout)
    }

    /// Processes events after each wake-up and returns the type of events
    /// (implements [`sd_journal_process`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Return Values:
    /// - Ok([Event](crate::Event)): journal wake event
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn process(&self) -> Result<Event, Error> {
        let result = unsafe { ffi::sd_journal_get_fd(self.ffi) };
        match result {
            ffi::SD_JOURNAL_NOP => Ok(Event::NOOP),
            ffi::SD_JOURNAL_APPEND => Ok(Event::Append),
            ffi::SD_JOURNAL_INVALIDATE => Ok(Event::Invalidate),
            _ => Err(Error::SDError(result))
        }
    }

    /// Wait for changes in the journal for a maximum period defined in timeout
    /// (implements [`sd_journal_wait`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#)).
    ///
    /// Use (uint64_t) -1 for timeout to wait indefinitely.
    ///
    /// Return Values:
    /// - Ok(Event): journal wake event
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
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
    /// [`sd_journal_has_runtime_files`](https://www.freedesktop.org/software/systemd/man/sd_journal_has_runtime_files.html#)).
    ///
    /// Return Values:
    /// - Ok(bool)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn has_runtime_files(&self) -> Result<bool, Error> {
        let result = unsafe { ffi::sd_journal_has_runtime_files(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result > 0)
    }

    /// Checks whether the journal owns persistent files (implements
    /// [`sd_journal_has_persistent_files`](https://www.freedesktop.org/software/systemd/man/sd_journal_has_persistent_files.html#)).
    ///
    /// Return Values:
    /// - Ok(bool)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn has_persistent_files(&self) -> Result<bool, Error> {
        let result = unsafe { ffi::sd_journal_has_persistent_files(self.ffi) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(result > 0)
    }

    /// Retrieve the data of a specific field (implements
    /// [`sd_journal_get_data`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Retrieve the data of a specific field. The fieldname must be provided in
    /// all upper case letters. See the documentation of well-known
    /// [field names](https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html#).
    /// Field names may not contain 0x00 bytes (would raise a NullError). If the
    /// current entry does not contain the field, an SDError(-2) is returned. On
    /// success the returned value follows the format "FIELDNAME=Fieldvalue".
    ///
    /// Parameters:
    /// - field: field name to investigate
    ///
    /// Return values:
    /// - Ok(CString): value in the format FIELDNAME=FIELDVALUE
    /// - Err(NullError): the requested field name contains 0-bytes
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_data(&self, field: &str) -> Result<CString, Error> {
        let field = match CString::new(field) {
            Ok(field) => field,
            Err(error) => return Err(Error::NullError(error))
        };
        let mut data: *const c_void = std::ptr::null_mut();
        let mut length: size_t = 0;
        let result =
            unsafe { ffi::sd_journal_get_data(self.ffi, field.as_ptr(), &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(unsafe { CStr::from_ptr(data as *mut c_char).to_owned() })
    }

    /// Enumerate the fields of the current record (implements
    /// [`sd_journal_enumerate_data`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Return values:
    /// - Ok([Enumeration](crate::Enumeration)): value in the format
    ///   FIELDNAME=FIELDVALUE
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn enumerate_data(&self) -> Result<Enumeration, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result = unsafe { ffi::sd_journal_enumerate_data(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(data as *const c_char).to_owned()
        }))
    }

    /// Enumerate the available & supported fields of the current record
    /// (implements [`sd_journal_enumerate_available_data`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Return values:
    /// - Ok([Enumeration](crate::Enumeration)): value in the format
    ///   FIELDNAME=FIELDVALUE
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn enumerate_available_data(&self) -> Result<Enumeration, Error> {
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
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(data as *const c_char).to_owned()
        }))
    }

    /// Restart enumeration of fields (implements
    /// [`sd_journal_restart_data`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    pub fn restart_data(&self) {
        unsafe {
            ffi::sd_journal_restart_data(self.ffi);
        }
    }

    /// Sets the data treshold limit for certain methods returning data fields
    /// (implements [`sd_journal_set_data_threshold`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Parameter:
    /// - size_t: treshold limit; 0 for no limit
    ///
    /// Return Values:
    /// - Ok(())
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn set_data_treshold(&self, size: size_t) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_set_data_threshold(self.ffi, size) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Gets the currently applied data treshold (implements
    /// [`sd_journal_get_data_threshold`](https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#)).
    ///
    /// Return Values:
    /// - Ok(size_t)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn get_data_treshold(&self) -> Result<size_t, Error> {
        let mut size: size_t = 0;
        let result = unsafe { ffi::sd_journal_get_data_threshold(self.ffi, &mut size) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(size)
    }

    /// Query the journal for unique field values of a certain field (implements
    /// [`sd_journal_query_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// Parameters:
    /// - field: field name to query for
    ///
    /// Return Values:
    /// - Ok(())
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn query_unique(&self, field: &CStr) -> Result<(), Error> {
        let result = unsafe { ffi::sd_journal_query_unique(self.ffi, field.as_ptr()) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(())
    }

    /// Enumerate available unique values for the field requested (implements
    /// [`sd_journal_enumerate_available_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// Return Values:
    /// - Ok(Value)
    /// - Ok(EoF)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn enumerate_available_unique(&self) -> Result<Enumeration, Error> {
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
            CStr::from_ptr(data as *const c_char).to_owned()
        }))
    }

    /// Enumerate all unique values for the field requested (implements
    /// [`sd_journal_enumerate_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    ///
    /// Return Values:
    /// - Ok(Value)
    /// - Ok(EoF)
    /// - Err([SDError](crate::Error)): sd-journal returned an error code
    pub fn enumerate_unique(&self) -> Result<Enumeration, Error> {
        let mut data: *const c_void = ptr::null_mut();
        let mut length: size_t = 0;
        let result = unsafe { ffi::sd_journal_enumerate_unique(self.ffi, &mut data, &mut length) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        if result == 0 {
            return Ok(Enumeration::EoF);
        }
        Ok(Enumeration::Value(unsafe {
            CStr::from_ptr(data as *const c_char).to_owned()
        }))
    }

    /// Restart enumeration of unique values (implements
    /// [`sd_journal_restart_unique`](https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#)).
    pub fn restart_unique(&self) {
        unsafe { ffi::sd_journal_restart_unique(self.ffi) }
    }
}
