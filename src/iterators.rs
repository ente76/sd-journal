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

pub struct CursorIterator<'a> {
    pub(crate) journal: &'a Journal
}

pub struct CursorReverseIterator<'a> {
    pub(crate) journal: &'a Journal
}

pub struct Cursor<'a> {
    pub(crate) journal: &'a Journal
}

pub struct Fields<'a> {
    pub(crate) journal: &'a Journal
}

pub struct FieldNames<'a> {
    pub(crate) journal: &'a Journal
}

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

impl<'a> Cursor<'a> {
    pub fn get_realtime(&self) -> Result<NaiveDateTime, Error> {
        self.journal.get_realtime()
    }

    pub fn get_monotonic(&self) -> Result<(Duration, sd_id128::ID128), Error> {
        self.journal.get_monotonic()
    }

    pub fn get_id(&self) -> Result<String, Error> {
        self.journal.get_cursor_id()
    }

    pub fn id_matches<S: Into<Vec<u8>>>(&self, cursor_id: S) -> Result<bool, Error> {
        self.journal.cursor_id_matches(cursor_id)
    }

    pub fn get_catalog(&self) -> Result<String, Error> {
        self.journal.get_catalog()
    }

    pub fn get_data<F: Into<Vec<u8>>>(&self, field: F) -> Result<String, Error> {
        self.journal.get_data(field)
    }

    pub fn enumerate_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        self.journal.enumerate_fields()
    }

    pub fn enumerate_available_fields(&self) -> Result<Enumeration<(String, String)>, Error> {
        self.journal.enumerate_available_fields()
    }

    pub fn restart_fields_enumeration(&self) {
        self.journal.restart_fields_enumeration()
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
