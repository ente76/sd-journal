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

use libc::c_int;
use sd_sys::journal as ffi;
use std::{ffi::{IntoStringError, NulError},
          fmt,
          str::Utf8Error};

/// Errors reported by Journal
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SDError(i32),
    UTF8Error(Utf8Error),
    NullError(NulError),
    RangeError,
    StringError(IntoStringError),
    TimeStampOutOfRange,
    UnexpectedDataFormat
}

/// Log Level of a log entry according to syslog.h as used in the journal.
///
/// Two convinience methods for the levels exist:
/// - as_raw_str(): returns a raw static &str to be used in log_raw_record()
/// - as_value_str(): returns a static &str to be used in log_record()
#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    Emergency = ffi::LOG_EMERG as isize,
    Alert     = ffi::LOG_ALERT as isize,
    Critical  = ffi::LOG_CRIT as isize,
    Error     = ffi::LOG_ERR as isize,
    Warning   = ffi::LOG_WARNING as isize,
    Notice    = ffi::LOG_NOTICE as isize,
    Info      = ffi::LOG_INFO as isize,
    Debug     = ffi::LOG_DEBUG as isize
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Level {
    /// Return the raw level &'static str to be used in log_raw_message(), e.g.
    /// Level::Critical.as_raw_str() returns "PRIORITY=2".
    pub fn as_raw_str(&self) -> &str {
        match self {
            Level::Emergency => "PRIORITY=0",
            Level::Alert => "PRIORITY=1",
            Level::Critical => "PRIORITY=2",
            Level::Error => "PRIORITY=3",
            Level::Warning => "PRIORITY=4",
            Level::Notice => "PRIORITY=5",
            Level::Info => "PRIORITY=6",
            Level::Debug => "PRIORITY=7"
        }
    }

    /// Returns the levels value as &'static str to be used in log_record(),
    /// e.g. Level::Critical.as_value_str() returns "2".
    pub const fn as_value_str(&self) -> &str {
        match self {
            Level::Emergency => "0",
            Level::Alert => "1",
            Level::Critical => "2",
            Level::Error => "3",
            Level::Warning => "4",
            Level::Notice => "5",
            Level::Info => "6",
            Level::Debug => "7"
        }
    }
}

/// Return value for certain cursor movement operations like next(), next_skip()
/// and alike.
///
/// Return Values:
/// - `Done`: full success
/// - `Limited(actual)`: the movement was executed but limited by the EoF of the
///   journal. The actual movement is given in the parameter.
/// - `EoF`: no movement was executed since the cursor is already placed at EoF.
///   This is an equivalent to Limited(0).
#[derive(Debug, PartialEq, Eq)]
pub enum CursorMovement {
    Done,
    Limited(c_int),
    EoF
}

/// Return value for enumerations over fields, e.g. enumerate_data
#[derive(Debug, PartialEq, Eq)]
pub enum Enumeration<T> {
    Value(T),
    EoF
}

/// File related options for opening a journal
#[derive(Debug, PartialEq, Eq)]
pub enum FileFlags {
    RuntimeOnly      = ffi::SD_JOURNAL_RUNTIME_ONLY as isize,
    LocalOnly        = ffi::SD_JOURNAL_LOCAL_ONLY as isize,
    LocalRuntimeOnly = (ffi::SD_JOURNAL_RUNTIME_ONLY | ffi::SD_JOURNAL_LOCAL_ONLY) as isize,
    AllFiles         = 0
}

/// User related options for opening a journal
#[derive(Debug, PartialEq, Eq)]
pub enum UserFlags {
    SystemOnly               = ffi::SD_JOURNAL_SYSTEM as isize,
    CurrentUserOnly          = ffi::SD_JOURNAL_CURRENT_USER as isize,
    CurrentUserAndSystemOnly = (ffi::SD_JOURNAL_SYSTEM | ffi::SD_JOURNAL_CURRENT_USER) as isize,
    AllUsers                 = 0
}

/// Namespace related options for opening a journal
pub enum NamespaceFlags {
    SelectedNamespaceOnly    = 0,
    DefaultNamespaceIncluded = ffi::SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE as isize
}

/// Path related options for opening a journal
pub enum PathFlags {
    FullPath     = 0,
    PathToOSRoot = ffi::SD_JOURNAL_OS_ROOT as isize
}

/// Journal event types
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    NOOP,
    Append,
    Invalidate
}
