#[test]
/// sfgg
fn readme() {
    use sd_journal::*;
    use std::path::PathBuf;

    // load local test data
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test-data/");
    println!("looking for test data in folder {}", test_data.display());
    let journal =
        Journal::open_directory(&test_data, PathFlags::FullPath, UserFlags::AllUsers).unwrap();

    // loop over journal records
    while let Ok(CursorMovement::Done) = journal.next() {
        // do something on each cursor, e.g. loop over all fields and print their field
        // name and value
        while let Ok(Enumeration::Value((field, value))) = journal.enumerate_fields() {
            println!("{}: {}", field, value)
        }
    }
    journal.seek_head().unwrap();
    // loop over journal records the rustified way
    for cursor in journal.iter().filter_map(Result::ok) {
        let cursor: Cursor = cursor;
        for (field, value) in cursor.into_iter().filter_map(Result::ok) {
            println!("{}: {}", field, value)
        }
    }
}
