use diesel::prelude::*;
use std::collections::HashMap;

#[macro_use]
extern crate diesel;

fn main() {
    let connection =
        SqliteConnection::establish("./songdata.db").expect("データベースに接続できませんでした");

    use crate::folder::dsl::*;
    let record: Vec<Record> = folder
        .load(&connection)
        .expect("クエリが実行できませんでした");

    let mut map = HashMap::new();
    for row in record {
        map.insert(row.title, row.path);
    }

    dbg!(map.len());
}

#[derive(Queryable)]
struct Record {
    title: String,
    path: String,
}

table! {
    folder(path) {
        title -> Text,
        path -> Text,
    }
}
