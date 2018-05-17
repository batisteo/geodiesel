#[cfg(feature = "postgres")]
extern crate url;

use support::{database, project};

#[test]
fn reset_drops_the_database() {
    let p = project("reset_drops_the_database")
        .folder("migrations")
        .build();
    let db = database(&p.database_url()).create();
    db.execute("CREATE TABLE posts ( id INTEGER )");

    assert!(db.table_exists("posts"));

    let result = p.command("database").arg("reset").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
    assert!(!db.table_exists("posts"));
}