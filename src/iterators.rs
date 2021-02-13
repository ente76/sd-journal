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

/// Iterator over entries in the journal
pub struct CursorIterator<'a> {
    pub(crate) journal: &'a Journal
}

/// Iterator over entries in the journal in reverse order
pub struct CursorReverseIterator<'a> {
    pub(crate) journal: &'a Journal
}

/// Iterator over the fields of a journal entry record
pub struct Fields<'a> {
    pub(crate) journal: &'a Journal
}

/// Iterator over the field names of the journal
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
pub struct FieldNames<'a> {
    pub(crate) journal: &'a Journal
}

/// Iterator over unique values assigned to a field in the journal
pub struct UniqueValues<'a> {
    pub(crate) journal: &'a Journal
}

impl<'a> Iterator for CursorIterator<'a> {
    type Item = Result<Cursor<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.journal.next() {
            Ok(CursorMovement::EoF) => None,
            Ok(CursorMovement::Done) => Some(Ok(Cursor { journal: self.journal })),
            Ok(CursorMovement::Limited(_)) => Some(Ok(Cursor { journal: self.journal })),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Iterator for CursorReverseIterator<'a> {
    type Item = Result<Cursor<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.journal.previous() {
            Ok(CursorMovement::EoF) => None,
            Ok(CursorMovement::Done) => Some(Ok(Cursor { journal: self.journal })),
            Ok(CursorMovement::Limited(_)) => Some(Ok(Cursor { journal: self.journal })),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> IntoIterator for &'a Journal {
    type IntoIter = CursorIterator<'a>;
    type Item = Result<Cursor<'a>, Error>;

    fn into_iter(self) -> Self::IntoIter {
        CursorIterator { journal: self }
    }
}

impl<'a> IntoIterator for Cursor<'a> {
    type IntoIter = Fields<'a>;
    type Item = Result<(String, String), Error>;

    fn into_iter(self) -> Self::IntoIter {
        Fields { journal: self.journal }
    }
}

impl<'a> Iterator for Fields<'a> {
    type Item = Result<(String, String), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.journal.enumerate_fields() {
            Ok(Enumeration::EoF) => None,
            Ok(Enumeration::Value(v)) => Some(Ok(v)),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a> Iterator for UniqueValues<'a> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.journal.enumerate_unique_values() {
            Ok(Enumeration::EoF) => None,
            Ok(Enumeration::Value(value)) => Some(Ok(value)),
            Err(e) => Some(Err(e))
        }
    }
}

#[cfg(any(feature = "246", feature = "245", feature = "229"))]
impl<'a> Iterator for FieldNames<'a> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.journal.enumerate_field_names() {
            Ok(Enumeration::EoF) => None,
            Ok(Enumeration::Value(v)) => Some(Ok(v)),
            Err(e) => Some(Err(e))
        }
    }
}
