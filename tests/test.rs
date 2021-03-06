extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

#[test]
fn mysql_basic_connection() {
    let manager = ConnectionManager::<MysqlConnection>::new("mysql://localhost/");

    let pool = Arc::new(r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .unwrap());

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn mysql_is_valid() {
    let manager = ConnectionManager::<MysqlConnection>::new("mysql://localhost/");

    let pool = r2d2::Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .build(manager)
        .unwrap();

    pool.get().unwrap();
}
