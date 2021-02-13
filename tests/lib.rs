use chrono::Duration;
use sd_id128::*;
use sd_journal::*;
use std::{ffi::CString,
          path::{Path, PathBuf}};

// testing on sd-journal
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

#[test]
fn log_message() {
    // send various different "Hello World!" to Journal
    // the following lines are all synonyms
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    Journal::log_message(Level::Info, String::from("Hello World!").as_str()).unwrap();
    Journal::log_message(Level::Info, String::from("Hello World!")).unwrap();
    Journal::log_message(Level::Info, CString::new("Hello World!").unwrap()).unwrap();
}

#[test]
fn log_raw_record() {
    // send various different "Hello World!" to Journal
    // the first two lines are synonyms
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    Journal::log_raw_record(&["PRIORITY=6", "MESSAGE=Hello World!"]).unwrap();
    // data: &Vec<String>
    Journal::log_raw_record(&vec![format!("PRIORITY={}", Level::Info),
                                  "MESSAGE=Hello World!".to_string(),
                                  format!("CODE_LINE={}", line!()),
                                  format!("CODE_FILE={}", file!()),
                                  "CUSTOM_FIELD=42".to_string()]).unwrap();
    // data: &[&str]
    Journal::log_raw_record(&["MESSAGE=Hello World!",
                              &format!("PRIORITY={}", Level::Info),
                              &format!("CODE_FILE={}", file!()),
                              &format!("CODE_LINE={}", line!()),
                              "CUSTOM_FIELD=42"]).unwrap();
}

#[test]
fn get_catalog_for_message_id() {
    // find the very first message with a catalog entry and print it.
    // if no such message is found: successful anyway
    // TODO: find a better and more meaningful test
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    loop {
        match journal.next() {
            Ok(CursorMovement::EoF) => break,
            Ok(_) => (),
            Err(_) => break
        }
        let id = match journal.get_data("MESSAGE_ID") {
            Err(_) => continue,
            Ok(value) => value
        };
        println!("Message ID: {}", id);
        let id128 = sd_id128::ID128::from_str(&id).unwrap();
        let catalog = match Journal::get_catalog_for_message_id(id128) {
            Err(_) => continue,
            Ok(v) => v
        };
        println!("{}", catalog);
        break;
    }
}

#[test]
fn open() {
    // Open the local system journal using various flags
    Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    Journal::open(FileFlags::LocalOnly, UserFlags::CurrentUserOnly).unwrap();
    Journal::open(FileFlags::RuntimeOnly, UserFlags::CurrentUserAndSystemOnly).unwrap();
    Journal::open(FileFlags::LocalOnly, UserFlags::SystemOnly).unwrap();
    Journal::open(FileFlags::LocalRuntimeOnly,
                  UserFlags::CurrentUserAndSystemOnly).unwrap();
}

#[test]
#[cfg(any(feature = "245", feature = "246"))]
fn open_namespace() {
    // Open the journal for a namespace including the default namespace
    Journal::open_namespace("namespace",
                            NamespaceFlags::DefaultNamespaceIncluded,
                            FileFlags::LocalOnly,
                            UserFlags::AllUsers).unwrap();
    // open a non-existent namespace and make sure, it is empty
    let journal = Journal::open_namespace("akjghöowighjökvndsövlljsk",
                                          NamespaceFlags::SelectedNamespaceOnly,
                                          FileFlags::AllFiles,
                                          UserFlags::AllUsers).unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::EoF);
}

#[test]
#[cfg(any(feature = "245", feature = "246"))]
fn open_all_namespaces() {
    // open the journal for all namespaces
    let journal = Journal::open_all_namespaces(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::Done);
}

#[test]
fn open_directory() {
    // open the system journal by pointing to root with path flags set to
    // PathToOSRoot
    Journal::open_directory("/", PathFlags::PathToOSRoot, UserFlags::AllUsers).unwrap();
    Journal::open_directory(Path::new("/"), PathFlags::PathToOSRoot, UserFlags::AllUsers).unwrap();
    Journal::open_directory(PathBuf::from("/"),
                            PathFlags::PathToOSRoot,
                            UserFlags::AllUsers).unwrap();
    // open test data included in a project located in a folder "test-data" in the
    // project root
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    Journal::open_directory(test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    // fail on a non existing folder
    Journal::open_directory("/...", PathFlags::FullPath, UserFlags::AllUsers).unwrap_err();
}

#[test]
fn open_files() {
    // open the curreńt system.journal file in the default location for journals
    // /var/log/journal/<MACHINE-ID>/system.journal
    let machine_id = sd_id128::ID128::machine_id().unwrap()
                                                  .to_string_sd()
                                                  .unwrap();
    let mut sdjournal_path = PathBuf::from("/var/log/journal/");
    sdjournal_path.push(&machine_id);
    sdjournal_path.push("system.journal");
    println!("looking for sd-journal in {}", sdjournal_path.display());
    Journal::open_files([sdjournal_path]).unwrap();
    // open test data included in a project located in a folder "test-data" in the
    // project root
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/test.journal");
    println!("looking for test data in {}", test_data.display());
    Journal::open_files([test_data]).unwrap();
    // fail on non-existing file
    Journal::open_files(vec!["/abcdefghijk.xyz"]).unwrap_err();
}

#[test]
fn next() {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    // loop over a journal & print it's messages
    while let Ok(CursorMovement::Done) = journal.next() {
        // do something on each cursor, e.g. print the MESSAGE
        println!("{}", journal.get_data("MESSAGE").unwrap());
    }
    // do a next() after seek_tail() to hit an EoF
    journal.seek_tail().unwrap();
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
    // which requires you to do a previous() after seek_tail() in order to get
    // to the expected position
    journal.previous().unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::EoF);
}

#[test]
fn iter() {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    // loop over a journal & print it's messages
    for cursor in journal.iter() {
        match cursor {
            Err(_) => break,
            Ok(cursor) => println!("{}", cursor.get_data("MESSAGE").unwrap())
        }
    }
    // ...
    journal.seek_head().unwrap();
    let cursor = journal.iter_reverse().next().unwrap().unwrap();
    // the following two lines are actually return the same value
    let m1 = cursor.get_data("MESSAGE").unwrap();
    let m2 = journal.get_data("MESSAGE").unwrap();
    assert_eq!(m1, m2);
}

#[test]
fn previous() {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    // loop over a journal & print it's messages
    while let Ok(CursorMovement::Done) = journal.previous() {
        // do something on each cursor, e.g. print the MESSAGE
        println!("{}", journal.get_data("MESSAGE").unwrap());
    }

    // do a previous() after seek_head() to hit an EoF
    journal.seek_head().unwrap();
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
    // which requires you to do a next() after seek_head() in order to get
    // the expected EoF
    journal.next().unwrap();
    assert_eq!(journal.previous().unwrap(), CursorMovement::EoF);
}

#[test]
fn iter_reverse() {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    // loop over a journal & print it's messages
    for cursor in journal.iter_reverse() {
        match cursor {
            Err(_) => break,
            Ok(cursor) => println!("{}", cursor.get_data("MESSAGE").unwrap())
        }
    }
    // ...
    journal.seek_tail().unwrap();
    let cursor = journal.iter_reverse().next().unwrap().unwrap();
    // the following two lines are actually return the same value
    let m1 = cursor.get_data("MESSAGE").unwrap();
    let m2 = journal.get_data("MESSAGE").unwrap();
    assert_eq!(m1, m2);
}

#[test]
fn next_skip() {
    // do a next_skip(10) and result in a Limited(5)
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
    // which requires you to do a previous() after seek_tail() in order to get
    // to the expected position
    journal.previous().unwrap();
    journal.previous_skip(5).unwrap();
    assert_eq!(journal.next_skip(10).unwrap(), CursorMovement::Limited(5));
}

#[test]
fn previous_skip() {
    // do a previous_skip(10) and result in a Limited(5)
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_head().unwrap();
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
    // which requires you to do a previous() after seek_tail() in order to get
    // to the expected position
    journal.next().unwrap();
    journal.next_skip(5).unwrap();
    assert_eq!(journal.previous_skip(10).unwrap(),
               CursorMovement::Limited(5));
}

#[test]
fn seek_head() {
    // seek_head --> next() --> previous() --> EoF
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_head().unwrap();
    // seek_head() should be followed by a next() before any previous() --> issues
    journal.next().unwrap();
    // previous() should hit EoF
    assert_eq!(journal.previous(), Ok(CursorMovement::EoF));
}

#[test]
fn seek_tail() {
    // seek_tail --> previous() --> next() --> EoF
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    // seek_head() should be followed by a previous() before any next() --> issues
    journal.previous().unwrap();
    // next() should hit EoF
    assert_eq!(journal.next(), Ok(CursorMovement::EoF));
}

#[test]
fn seek_monotonic() {
    // get monotonic cutoff of current boot id --> seek to start +5 and do a
    // previous() then get the monotonic time ==> should be equal to start
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let boot_id = ID128::boot_id().unwrap();
    let (from, _) = journal.get_monotonic_cutoff(boot_id.clone()).unwrap();
    journal.seek_monotonic(boot_id.clone(), from).unwrap();
    journal.previous().unwrap();
    let (mono_journal, bid_journal) = journal.get_monotonic().unwrap();
    assert_eq!(mono_journal, from);
    assert_eq!(bid_journal, boot_id);
}

#[test]
fn seek_realtime() {
    // get current realtime now
    // seek_realtime(now) + previous ==> last_entry
    // clock of last_entry should be < now
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let now = chrono::offset::Local::now().naive_local();
    journal.seek_realtime(now).unwrap();
    journal.previous().unwrap();
    let clock_last_entry = journal.get_realtime().unwrap();
    assert!(clock_last_entry <= now);
    // seek_tail + previous
    // clock of last_entry should match clock of tail
    journal.seek_tail().unwrap();
    journal.previous().unwrap();
    let clock_tail = journal.get_realtime().unwrap();
    assert_eq!(clock_last_entry, clock_tail);
    // get realtime_cutoff
    // clock of last_entry should match end of realtime_cutoff
    let (start, end) = journal.get_realtime_cutoff().unwrap();
    assert_eq!(clock_last_entry, end);
    // seek to 5 microseconds before start of journal + next()
    // clock of first entry should match start of cutoff_realtime
    let bstart = start.checked_sub_signed(chrono::Duration::seconds(6))
                      .unwrap();
    journal.seek_realtime(bstart).unwrap();
    journal.next().unwrap();
    assert_eq!(start, journal.get_realtime().unwrap());
    // seek to 5 microseconds past end of journal + previous()
    // clock of entry should match the end of cutoff_realtime
    let aend = end.checked_add_signed(Duration::seconds(10)).unwrap();
    journal.seek_realtime(aend).unwrap();
    journal.previous().unwrap();
    assert_eq!(end, journal.get_realtime().unwrap());
}

#[test]
fn seek_cursor_id() {
    // go to 10 items before end --> get cursor
    // go to head
    // seek_cursor(cursor)
    // assert that get_cursor gives the same cursor again
    // next() --> assert get_cursor gives another cursor this time
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    journal.previous_skip(10).unwrap();
    let cursor = journal.get_cursor_id().unwrap();
    println!("initial cursor: {:?}", cursor);
    journal.seek_head().unwrap();
    journal.next().unwrap();
    journal.seek_cursor_id(cursor.clone()).unwrap();
    journal.previous().unwrap();
    let same_cursor = journal.get_cursor_id().unwrap();
    println!("same cursor: {:?}", same_cursor);
    assert_eq!(cursor, same_cursor);
    journal.next().unwrap();
    let other_cursor = journal.get_cursor_id().unwrap();
    println!("other cursor: {:?}", other_cursor);
    assert_ne!(cursor, other_cursor);
}

#[test]
fn add_match() {
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    // add a match for "MESSAGE=Hello World!" should succeed while a match for
    // "MESSAGE=Hello Woooooorld!" should not return any matches
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_match("MESSAGE=Hello World!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::Done);
    while let Ok(CursorMovement::Done) = journal.next() {
        // do something on the journal entries
    }
    journal.flush_matches();
    journal.add_match("MESSAGE=Hello Woooooorld!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::EoF);
    // add some more matches to assure type compatibility
    journal.add_match("MESSAGE=Hello Woooooorld!").unwrap();
    journal.add_match(&"MESSAGE=Hello Woooooorld!").unwrap();
    journal.add_match("MESSAGE=Hello Woooooorld!".to_string())
           .unwrap();
}

#[test]
fn add_disjunction() {
    // add a match for "MESSAGE=Hello Woooooooooorld!" OR "MESSAGE=Hello World!"
    // should find data (i.e. next() return Done)
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_match(b"MESSAGE=Hello World!").unwrap();
    journal.add_disjunction().unwrap();
    journal.add_match(b"MESSAGE=Hello Woooooooooorld!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::Done);
}

#[test]
fn add_conjunction() {
    // add a match for "MESSAGE=Hello Woooooooooorld!" AND "MESSAGE=Hello World!"
    // should not find any data (i.e. next() returns EoF)
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_match(b"MESSAGE=Hello World!").unwrap();
    journal.add_conjunction().unwrap();
    journal.add_match(b"MESSAGE=Hello Woooooooooorld!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::EoF);
}

#[test]
fn flush_matches() {
    Journal::log_message(Level::Info, "Hello World!").unwrap();
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_match(b"MESSAGE=Hello Woooooorld!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::EoF);
    journal.flush_matches();
    assert_eq!(journal.next().unwrap(), CursorMovement::Done);
}

#[test]
fn get_realtime_cutoff() {
    // get realtime cutoff --> from < to
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let (from, to) = journal.get_realtime_cutoff().unwrap();
    println!("{} - {}", from, to);
    assert!(from < to);
}

#[test]
fn get_monotonic_cutoff() {
    // get current boot id
    // get monotonic cutoff --> from < to
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let (from, to) = journal.get_monotonic_cutoff(sd_id128::ID128::boot_id().unwrap())
                            .unwrap();
    println!("{} - {}", from, to);
    assert!(from < to);
}

#[test]
fn set_treshold() {
    // set the treshold without error
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.set_data_treshold(5).unwrap();
}

#[test]
fn get_treshold() {
    // get the old treshold
    // set the treshold to 5; assert the new value is 5
    // assert the new value of 5 differs from the old value (very unlikely to match)
    // set the treshold to 0 (unlimited)
    // assert the value is not 0
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let old = journal.get_data_treshold().unwrap();
    println!("old value: {}", old);
    journal.set_data_treshold(5).unwrap();
    let new = journal.get_data_treshold().unwrap();
    println!("new value: {}", new);
    assert!(old != new);
    assert_eq!(new, 5);
    journal.set_data_treshold(0).unwrap();
    let ulimited = journal.get_data_treshold().unwrap();
    println!("ulimited value: {}", ulimited);
    assert_eq!(ulimited, 0);
}

#[test]
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
fn enumerate_field_names() {
    // loop once through all fields an print them assuming no error is raised ever
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    loop {
        match journal.enumerate_field_names().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => println!("{}", string)
        }
    }
}

#[test]
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
fn restart_fields() {
    // enumerate fields until "MESSAGE" is found
    // restart
    // enumerate again until "MESSAGE" is found
    // any error or "MESSAGE" is not found lead to an error
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    loop {
        match journal.enumerate_field_names().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => {
                if string == "MESSAGE" {
                    println!("Field 'MESSAGE' found for the first time. Let's restart \
                              enumeration.");
                    journal.restart_field_name_enumeration();
                    break;
                }
            },
        }
    }
    loop {
        match journal.enumerate_field_names().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => {
                if string == "MESSAGE" {
                    println!("Field 'MESSAGE' found for the second time. Restart seems to work \
                              fine.");
                    return;
                }
            },
        }
    }
    panic!("Test failed: Field Message was either not existing at all or restart did not work.");
}

#[test]
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
fn iter_field_names() {
    // loop once through all fields an print them assuming no error is raised ever
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    for fieldname in journal.iter_field_names() {
        println!("{}", fieldname.unwrap());
    }
}

#[test]
fn get_fd() {
    // TODO: do a more meaningful test: open the fd for reading
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_fd().unwrap();
}

#[test]
fn get_events() {
    // TODO: do a more meaningful test: open the fd for reading
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_events().unwrap();
}

#[test]
fn get_timeout() {
    // TODO: do a more meaningful test: poll the fd and measure the timeout
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_timeout().unwrap();
}

#[test]
fn process() {
    // TODO: do a test at all... calling process without a wait seems to return
    // random numbers
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.process().unwrap();
}

#[test]
fn wait() {
    // TODO: do a more meaningful test: the journal always returns INVALIDATE???
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    journal.wait(10).unwrap();
}

#[test]
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
fn has_runtime_files() {
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.has_runtime_files().unwrap();
}

#[test]
#[cfg(any(feature = "246", feature = "245", feature = "229"))]
fn has_persistent_files() {
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.has_persistent_files().unwrap();
}

#[test]
fn get_usage() {
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    println!("usage in bytes: {}", journal.get_usage().unwrap());
    assert!(journal.get_usage().unwrap() > 0);
}

#[test]
fn get_realtime() {
    // get realtime_usec on a postioned journal at head and tail
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    println!("realtime at journal head: {}",
             journal.get_realtime().unwrap());
    journal.seek_tail().unwrap();
    journal.previous().unwrap();
    println!("realtime at journal tail: {}",
             journal.get_realtime().unwrap());
}

#[test]
fn get_monotonic() {
    // get realtime_usec on a postioned journal at head and tail
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    println!("monotonic at journal head: ({}, {})",
             journal.get_monotonic().unwrap().0,
             journal.get_monotonic().unwrap().1);
    journal.seek_tail().unwrap();
    journal.previous().unwrap();
    println!("monotonic at journal tail: ({}, {})",
             journal.get_monotonic().unwrap().0,
             journal.get_monotonic().unwrap().1);
}

#[test]
fn get_cursor_id() {
    // get cursor and print it
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    println!("{:?}", journal.get_cursor_id().unwrap());
}

#[test]
fn cursor_id_matches() {
    // get cursor -> test_cursor matches on same position
    // next() -> test_cursor does not match anymore
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    let cursor = journal.get_cursor_id().unwrap();
    assert_eq!(journal.cursor_id_matches(cursor.clone()).unwrap(), true);
    journal.next().unwrap();
    assert_eq!(journal.cursor_id_matches(cursor.clone()).unwrap(), false);
}

#[test]
fn get_catalog() {
    // if there is a catalogue entry, there must be a field "MESSAGE_ID" filled
    // iterate over records until you find "MESSAGE_ID"
    // get catalog for message
    // go to next without "MESSAGE_ID" --> get_catalogue returns an error
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while journal.get_data("MESSAGE_ID").is_err() {
        match journal.next().unwrap() {
            CursorMovement::EoF => return,
            _ => ()
        }
    }
    let c = journal.get_catalog().unwrap();
    println!("{}", c);
    while journal.get_data("MESSAGE_ID").is_ok() {
        match journal.next().unwrap() {
            CursorMovement::EoF => return,
            _ => ()
        }
    }
    journal.get_catalog().unwrap_err();
}

#[test]
fn get_data() {
    // get data for field "MESSAGE" and check the result actually contains
    // "MESSAGE="
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();
    for cursor in &journal {
        let cursor = cursor.unwrap();
        let message = cursor.get_data("MESSAGE")
                            .unwrap_or("[no message available]".to_string());
        let datetime = cursor.get_realtime().unwrap();
        println!("{} - {}", datetime, message);
    }
}

#[test]
fn enumerate_fields() {
    // loop through all fields of a record and print them
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
        println!("{}: {}", field, value)
    }
}

#[test]
#[cfg(any(feature = "246"))]
fn enumerate_available_fields() {
    // loop through all fields of a record and print them
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while let Ok(Enumeration::Value((field, value))) = journal.enumerate_available_fields() {
        println!("{}: {}", field, value)
    }
}

#[test]
fn restart_fields_enumeration() {
    // loop through all fields of a record => if the field contains MESSAGE=restart
    // loop a second time through all fields: if field MESSAGE= shows up again ==>
    // success
    // test fails on:
    // - no field MESSAGE ever found (first & second loop exit without match)
    // - restart_data() fails, i.e. second loop does not find "MESSAGE" once more
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
        println!("{}: {}", field, value);
        if field == "MESSAGE" {
            journal.restart_fields_enumeration();
            break;
        }
    }
    while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
        println!("{}: {}", field, value);
        if field == "MESSAGE" {
            return;
        }
    }
    panic!("Test failed: Field MESSAGE either does not exist or restart_data() did not succeed.");
}

#[test]
fn iter_fields() {
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    // The following 2 loops are synonyms
    while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
        println!("{}: {}", field, value);
    }
    for field in journal.iter_fields() {
        let (field, value) = field.unwrap();
        println!("{}: {}", field, value);
    }
}

#[test]
fn query_unique_values() {
    // run a query for a field without raising an error
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique_values("MESSAGE").unwrap();
}

#[test]
fn enumerate_unique_values() {
    // query MESSAGE field 3 times and assert each result differs
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique_values("MESSAGE").unwrap();
    let mut results = Vec::new();
    while true {
        let value = journal.enumerate_unique_values().unwrap();
        if value == Enumeration::EoF {
            println!("reached EoF");
            break;
        }
        if results.iter().any(|v| v == &value) {
            println!("found duplicate: {:?}", value);
            assert!(false);
        }
        results.push(value);
    }
    // let first = journal.enumerate_unique_values().unwrap();
    // println!("first: {:?}", first);
    // let second = journal.enumerate_unique_values().unwrap();
    // println!("second: {:?}", second);
    // assert!(first != second);
}

#[test]
#[cfg(any(feature = "246"))]
fn enumerate_available_unique_values() {
    // query MESSAGE field 3 times and assert each result differs
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique_values("MESSAGE").unwrap();
    let first = journal.enumerate_unique_values().unwrap();
    println!("first: {:?}", first);
    let second = journal.enumerate_unique_values().unwrap();
    println!("second: {:?}", second);
    assert_eq!(first, Enumeration::Value("Hello World!".to_string()));
    assert_eq!(second, Enumeration::EoF);
}

#[test]
fn restart_unique_value_enumeration() {
    // query _PID field 3 times but restart after the second time
    // assert 1 and 2 differ
    // assert 1 and 3 match
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique_values("_PID").unwrap();
    let first = journal.enumerate_unique_values().unwrap();
    println!("first: {:?}", first);
    let second = journal.enumerate_unique_values().unwrap();
    println!("second: {:?}", second);
    journal.restart_unique_value_enumeration();
    let third = journal.enumerate_unique_values().unwrap();
    println!("third: {:?}", third);
    assert_ne!(first, second);
    assert_eq!(first, third);
}

#[test]
fn iter_unique_values() {
    let journal = Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    for value in journal.iter_unique_values("MESSAGE").unwrap() {
        let value = value.unwrap();
        println!("{}", value);
    }
}
