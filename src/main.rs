use diesel::prelude::*;
use std::collections::HashMap;
use std::fs::copy;

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

    let list = std::fs::read_dir("./previews")
        .expect("error")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("error");

    for from in list {
        let from = from.file_name().unwrap().to_str().unwrap();
        match map.get(from) {
            Some(target) => {
                let from = "./previews/".to_owned() + from + "/preview_music.ogg";
                let to = target.to_string() + "/preview_music.ogg";
                println!("{}を{}としてコピーを試みます.", from, to);
                copy(from, to).expect("コピーに失敗しました.");
                println!("コピーしました.");
            }
            _ => {
                println!("{} is not found in DB.", from);
            }
        }
    }
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
