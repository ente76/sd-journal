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

use sd_id128::ID128;
use sd_journal::*;
use std::ffi::CString;

#[test]
fn print() {
    // send "Hello World!" to Journal
    lli::Journal::print(Level::Info, &CString::new("Hello World!").unwrap()).unwrap();
}

#[test]
fn sendv() {
    // send various different "Hello World!" to Journal
    lli::Journal::sendv(&["PRIORITY=6", "MESSAGE=Hello World!"]).unwrap();
    lli::Journal::sendv(&["PRIORITY=6".to_string(), "MESSAGE=Hello World!".to_string()]).unwrap();
    lli::Journal::sendv(&Vec::from(["PRIORITY=6", "MESSAGE=Hello World!"])).unwrap();
    lli::Journal::sendv(&Vec::from(["PRIORITY=6".to_string(),
                                    "MESSAGE=Hello World!".to_string(),
                                    "CODE_FUNC=log_structured_raw_entry()".to_string(),
                                    "CODE_LINE=34".to_string(),
                                    "CODE_FILE=tests/lib.rs".to_string()])).unwrap();
}

#[test]
fn open() {
    // Open the journal using various flags
    lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    lli::Journal::open(FileFlags::LocalOnly, UserFlags::CurrentUserOnly).unwrap();
}

#[test]
fn open_namespace() {
    // Open the journal for a namespace including the default namespace
    lli::Journal::open_namespace(&CString::new("namespace").unwrap(),
                                 NamespaceFlags::DefaultNamespaceIncluded,
                                 FileFlags::LocalOnly,
                                 UserFlags::AllUsers).unwrap();
}

#[test]
fn open_all_namespaces() {
    // open the journal for all namespaces
    lli::Journal::open_all_namespaces(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
}

#[test]
fn open_directory() {
    // open the standard journal by opening the directory relatively to root -->
    // system wide default journal will be opened
    lli::Journal::open_directory(&CString::new("/").unwrap(),
                                 PathFlags::PathToOSRoot,
                                 UserFlags::AllUsers).unwrap();
    // fail on a non existing folder
    lli::Journal::open_directory(&CString::new("/...").unwrap(),
                                 PathFlags::FullPath,
                                 UserFlags::AllUsers).unwrap_err();
}

#[test]
fn open_files() {
    // open the curreńt system.journal file in the default location for journals
    // /var/log/journal/<MACHINE-ID>/system.journal
    let id = sd_id128::ID128::machine_id().unwrap()
                                          .to_string_sd()
                                          .unwrap();
    let mut folder = "/var/log/journal/".to_string();
    folder.push_str(&id);
    folder.push_str("/system.journal");
    lli::Journal::open_files(&[&CString::new(folder).unwrap()]).unwrap();
    lli::Journal::open_files(&[&CString::new("/").unwrap()]).unwrap_err();
}

#[test]
fn next() {
    // do a next()
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    // TODO: seek_tail & do a next() ==> should result in EoF, unfortunately
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn previous() {
    // do a previous()
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.previous().unwrap();
    // TODO: seek_head & do a previous() ==> should result in EoF, unfortunately
    // there is a [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn next_skip() {
    // do a next_skip(10) and result in a "done"
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_head().unwrap();
    assert_eq!(journal.next_skip(10).unwrap(), CursorMovement::Done);
    // TODO: seek_tail & previous_skip(5) & do a next_skip(10) ==> should result
    // in Limited(5) ,unfortunately there is a
    // [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn previous_skip() {
    // do a previous(10) and result in "done"
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    assert_eq!(journal.previous_skip(10).unwrap(), CursorMovement::Done);
    // TODO: seek_tail & previous_skip(5) & do a next_skip(10) ==> should result
    // in Limited(5) ,unfortunately there is a
    // [defect in libsystemd](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn get_realtime_usec() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    // get realtime_usec on a non positioned journal ==> error
    journal.get_realtime_usec().unwrap_err();
    // get realtime_usec on a postioned journal ==> success
    journal.next().unwrap();
    journal.get_realtime_usec().unwrap();
}

#[test]
fn get_monotonic_usec() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    // get monotonic_usec on a non positioned journal ==> error
    journal.get_monotonic_usec().unwrap_err();
    // get monotonic_usec on a positioned journal ==> success
    journal.next().unwrap();
    journal.get_monotonic_usec().unwrap();
}

// TODO: add more meaningful test
#[test]
fn add_match() {
    // add a match for "MESSAGE=Hello World!" should succeed due to the other tests
    // executed before
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_match(b"MESSAGE=Hello World!").unwrap();
    assert_eq!(journal.next().unwrap(), CursorMovement::Done);
}

// TODO: add more meaningful test
#[test]
fn add_disjunction() {
    // add a disjunction marker
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_disjunction().unwrap();
    journal.next().unwrap();
}

// TODO: add more meaningful test
#[test]
fn add_conjunction() {
    // add a conjunction marker
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.add_conjunction().unwrap();
    journal.next().unwrap();
}

// TODO: add more meaningful test
#[test]
fn flush_matches() {
    // flush matches without previously defining any
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.flush_matches();
}

#[test]
fn seek_head() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_head().unwrap();
    // TODO: add a test that previous() should result in EoF ... see
    // [defect](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn seek_tail() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    // TODO: add a test that next() should result in EoF ... see
    // [defect](https://github.com/systemd/systemd/issues/17662)
}

#[test]
fn seek_monotonic_usec() {
    // get monotonic cutoff of current boot id --> seek to start +5 and do a
    // previous() then get the monotonic time ==> should be equal to start
    let bid_today = ID128::boot_id().unwrap();
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let (mono_start, _) = journal.get_cutoff_monotonic_usec(bid_today.clone())
                                 .unwrap();
    journal.seek_monotonic_usec(bid_today.clone(), mono_start + 5)
           .unwrap();
    journal.previous().unwrap();
    let (mono_journal, bid_journal) = journal.get_monotonic_usec().unwrap();
    assert_eq!(mono_journal, mono_start);
    assert_eq!(bid_journal, bid_today);
}

#[test]
fn seek_realtime_usec() {
    // get current realtime now
    // seek_realtime(now) + previous ==> last_entry
    // clock of last_entry should be < now
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
    journal.seek_realtime_usec(now).unwrap();
    journal.previous().unwrap();
    let clock_last_entry = journal.get_realtime_usec().unwrap();
    assert!(clock_last_entry <= now);
    // seek_tail + previous
    // clock of last_entry should match clock of tail
    journal.seek_tail().unwrap();
    journal.previous().unwrap();
    let clock_tail = journal.get_realtime_usec().unwrap();
    assert_eq!(clock_last_entry, clock_tail);
    // get realtime_cutoff
    // clock of last_entry should match end of realtime_cutoff
    let (start, end) = journal.get_cutoff_realtime_usec().unwrap();
    assert_eq!(clock_last_entry, end);
    // seek to 5 microseconds before start of journal + next()
    // clock of first entry should match start of cutoff_realtime
    let bstart = start - 5;
    journal.seek_realtime_usec(bstart).unwrap();
    journal.next().unwrap();
    assert_eq!(start, journal.get_realtime_usec().unwrap());
    // seek to 5 microseconds past end of journal + previous()
    // clock of entry should match the end of cutoff_realtime
    let aend = end + 5;
    journal.seek_realtime_usec(aend).unwrap();
    journal.previous().unwrap();
    assert_eq!(end, journal.get_realtime_usec().unwrap());
}

#[test]
fn seek_cursor() {
    // go to 10 items before end --> get cursor
    // go to head
    // seek_cursor(cursor)
    // assert that get_cursor gives the same cursor again
    // next() --> assert get_cursor gives another cursor this time
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.seek_tail().unwrap();
    journal.previous_skip(10).unwrap();
    let cursor = journal.get_cursor().unwrap();
    println!("initial cursor: {:?}", cursor);
    journal.seek_head().unwrap();
    journal.next().unwrap();
    journal.seek_cursor(&cursor).unwrap();
    journal.previous().unwrap();
    let same_cursor = journal.get_cursor().unwrap();
    println!("same cursor: {:?}", same_cursor);
    assert_eq!(cursor, same_cursor);
    journal.next().unwrap();
    let other_cursor = journal.get_cursor().unwrap();
    println!("other cursor: {:?}", other_cursor);
    assert_ne!(cursor, other_cursor);
}

#[test]
fn enumerate_fields() {
    // loop once through all fields an print them assuming no error is raised ever
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    loop {
        match journal.enumerate_fields().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => println!("{}", string.to_string_lossy().to_owned())
        }
    }
}

#[test]
fn restart_fields() {
    // enumerate fields until "MESSAGE" is found
    // restart
    // enumerate again until "MESSAGE" is found
    // any error or "MESSAGE" is not found lead to an error
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    loop {
        match journal.enumerate_fields().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => {
                if string.to_string_lossy() == "MESSAGE" {
                    println!("Field 'MESSAGE' found for the first time. Let's restart \
                              enumeration.");
                    journal.restart_fields();
                    break;
                }
            },
        }
    }
    loop {
        match journal.enumerate_fields().unwrap() {
            Enumeration::EoF => break,
            Enumeration::Value(string) => {
                if string.to_string_lossy() == "MESSAGE" {
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
fn get_cursor() {
    // get cursor and print it
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    println!("{:?}", journal.get_cursor().unwrap());
}

#[test]
fn test_cursor() {
    // get cursor -> test_cursor matches on same position
    // next() -> test_cursor does not match anymore
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    let cursor = journal.get_cursor().unwrap();
    assert_eq!(journal.test_cursor(&cursor).unwrap(), CursorCheck::Matches);
    journal.next().unwrap();
    assert_eq!(journal.test_cursor(&cursor).unwrap(),
               CursorCheck::DoesNotMatch);
}

#[test]
fn get_cutoff_realtime() {
    // get realtime cutoff --> from < to
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let (from, to) = journal.get_cutoff_realtime_usec().unwrap();
    assert!(from < to);
}

#[test]
fn get_cutoff_monotonic() {
    // get current boot id
    // get monotonic cutoff --> from < to
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    let (from, to) = journal.get_cutoff_monotonic_usec(sd_id128::ID128::boot_id().unwrap())
                            .unwrap();
    assert!(from < to);
}

#[test]
fn get_catalog() {
    // if there is a catalogue entry, there must be a field "MESSAGE_ID" filled
    // iterate over records until you find "MESSAGE_ID"
    // get catalog for message
    // go to next without "MESSAGE_ID" --> get_catalogue returns an error
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while journal.get_data("MESSAGE_ID").is_err() {
        match journal.next().unwrap() {
            CursorMovement::EoF => return,
            _ => ()
        }
    }
    let c = journal.get_catalog().unwrap();
    println!("{}", c.to_string_lossy());
    while journal.get_data("MESSAGE_ID").is_ok() {
        match journal.next().unwrap() {
            CursorMovement::EoF => return,
            _ => ()
        }
    }
    journal.get_catalog().unwrap_err();
}

#[test]
fn get_catalog_for_message_id() {
    // if there is a catalogue entry, there must be a field "MESSAGE_ID" filled
    // iterate over records until you find "MESSAGE_ID"
    // get catalog for message
    // go to next without "MESSAGE_ID" --> get_catalogue returns an error
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    while journal.get_data("MESSAGE_ID").is_err() {
        match journal.next().unwrap() {
            CursorMovement::EoF => return,
            _ => ()
        }
    }
    let mid = journal.get_data("MESSAGE_ID").unwrap();
    let sid = mid.to_string_lossy().to_string().split_off(11);
    let id = ID128::from_str(sid.as_ref()).unwrap();
    let c = lli::Journal::get_catalog_for_message_id(id).unwrap();
    println!("{}", c.to_string_lossy().to_string());
    let _ = lli::Journal::get_catalog_for_message_id(ID128::random_id().unwrap()).unwrap_err();
}

#[test]
fn get_fd() {
    // TODO: do a more meaningful test: open the fd for reading
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_fd().unwrap();
}

#[test]
fn get_events() {
    // TODO: do a more meaningful test: open the fd for reading
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_events().unwrap();
}

#[test]
fn get_timeout() {
    // TODO: do a more meaningful test: poll the fd and measure the timeout
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.get_timeout().unwrap();
}

#[test]
fn process() {
    // TODO: do a test at all... calling process without a wait seems to return
    // random numbers
    // let journal = lli::Journal::open(FileFlags::AllFiles,
    // UserFlags::AllUsers).unwrap(); journal.process().unwrap();
}

#[test]
fn wait() {
    // TODO: do a more meaningful test: the journal always returns INVALIDATE???
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.wait(10).unwrap();
}

#[test]
fn has_runtime_files() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.has_runtime_files().unwrap();
}

#[test]
fn has_persistent_files() {
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.has_persistent_files().unwrap();
}

#[test]
fn get_data() {
    // get data for field "MESSAGE" and check the result actually contains
    // "MESSAGE="
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    journal.get_data("MESSAGE")
           .unwrap()
           .to_string_lossy()
           .contains("MESSAGE=");
}

#[test]
fn enumerate_data() {
    // loop through all fields of a record and print them
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    loop {
        match journal.enumerate_data().unwrap() {
            Enumeration::Value(result) => println!("{}", result.to_string_lossy()),
            Enumeration::EoF => break
        }
    }
}

#[test]
fn enumerate_available_data() {
    // loop through all fields of a record and print them
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    loop {
        match journal.enumerate_available_data().unwrap() {
            Enumeration::Value(result) => println!("{}", result.to_string_lossy()),
            Enumeration::EoF => break
        }
    }
}

#[test]
fn restart_data() {
    // loop through all fields of a record => if the field contains MESSAGE= restart
    // loop a second time through all fields: if field MESSAGE= shows up again ==>
    // success
    // test fails on:
    // - no field MESSAGE ever found (first & second loop exit without match)
    // - restart_data() fails, i.e. second loop does not find "MESSAGE" once more
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.next().unwrap();
    loop {
        match journal.enumerate_available_data().unwrap() {
            Enumeration::Value(result) => {
                println!("{}", result.to_string_lossy());
                if result.to_string_lossy().to_string().contains("MESSAGE=") {
                    journal.restart_data();
                    break;
                }
            },
            Enumeration::EoF => break
        }
    }
    loop {
        match journal.enumerate_available_data().unwrap() {
            Enumeration::Value(result) => {
                if result.to_string_lossy().to_string().contains("MESSAGE=") {
                    return;
                }
            },
            Enumeration::EoF => break
        }
    }
    panic!("Test failed: Field MESSAGE either does not exist or restart_data() did not succeed.");
}

#[test]
fn set_treshold() {
    // set the treshold without error
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.set_data_treshold(5).unwrap();
}

#[test]
fn get_treshold() {
    // get the old treshold
    // set the treshold to 5; assert the new value is 5
    // assert the new value of 5 differs from the old value (very unlikely to match)
    // set the treshold to 0 (unlimited)
    // assert the value is not 0
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
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
fn query_unique() {
    // run a query for a field without raising an error
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique(&CString::new("MESSAGE").unwrap())
           .unwrap();
}

#[test]
fn enumerate_unique() {
    // query MESSAGE field 3 times and assert each result differs
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique(&CString::new("MESSAGE").unwrap())
           .unwrap();
    let first = journal.enumerate_unique().unwrap();
    println!("first: {:?}", first);
    let second = journal.enumerate_unique().unwrap();
    println!("second: {:?}", second);
    let third = journal.enumerate_unique().unwrap();
    println!("third: {:?}", third);
    assert_ne!(first, second);
    assert_ne!(first, third);
}

#[test]
fn enumerate_available_unique() {
    // query MESSAGE field 3 times and assert each result differs
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique(&CString::new("MESSAGE").unwrap())
           .unwrap();
    let first = journal.enumerate_available_unique().unwrap();
    println!("first: {:?}", first);
    let second = journal.enumerate_available_unique().unwrap();
    println!("second: {:?}", second);
    let third = journal.enumerate_available_unique().unwrap();
    println!("third: {:?}", third);
    assert_ne!(first, second);
    assert_ne!(first, third);
}

#[test]
fn restart_unique() {
    // query MESSAGE field 3 times but restart after the second time
    // assert 1 and 2 differ
    // assert 1 and 3 match
    let journal = lli::Journal::open(FileFlags::AllFiles, UserFlags::AllUsers).unwrap();
    journal.query_unique(&CString::new("MESSAGE").unwrap())
           .unwrap();
    let first = journal.enumerate_available_unique().unwrap();
    println!("first: {:?}", first);
    let second = journal.enumerate_available_unique().unwrap();
    println!("second: {:?}", second);
    journal.restart_unique();
    let third = journal.enumerate_available_unique().unwrap();
    println!("third: {:?}", third);
    assert_ne!(first, second);
    assert_eq!(first, third);
}
