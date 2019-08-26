use std::fmt::Debug;

use rand::{Rng, RngCore, thread_rng};

use crate::skip_list;
use crate::skip_list::BestTransactionLog;

use super::test::{Bencher, iter};

const LIST_ITEMS: u64 = 15_000;

#[bench]
fn bench_skip_list_find(b: &mut Bencher)
{
    let mut log = BestTransactionLog::new_empty(20);
    for i in 0..LIST_ITEMS {
        log.append(i, format!("INSERT DATA {}", i));
    }

    let mut rng = thread_rng();
    b.iter(|| {
        log.find(rng.gen_range(0, LIST_ITEMS)).expect("NOT FOUND");
    });
}

#[test]
fn test_skip_list() {
    let mut log = BestTransactionLog::new_empty(20);

    log.append(0, format!("INSERT DATA {}", 0));

    assert_eq!(log.length(), 1);

//    for x in log.iter(0) {
//        assert_eq!(x.1, format!("INSERT DATA {}", 0))
//    }

    assert_eq!(log.find(0).expect("NOT FOUND"), format!("INSERT DATA {}", 0));
}

#[test]
fn test_skip_list_find() {
    let mut log = BestTransactionLog::new_empty(20);
    for i in 0..LIST_ITEMS {
        log.append(i, format!("INSERT DATA {}", i));
    }

    let mut rng = thread_rng();
    for _ in 0..LIST_ITEMS {
        let i = rng.gen_range(0, LIST_ITEMS);
        assert_eq!(log.find(i).expect("NOT FOUND"), format!("INSERT DATA {}", i));
    }
}

#[test]
fn skip_list_append() {
    let mut list = skip_list::BestTransactionLog::new_empty(3);
    list.append(1, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(2, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(3, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(4, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(5, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(6, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    list.append(7, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
    assert_eq!(list.length(), 7);
}

#[test]
fn skip_list_find() {
    let mut list = skip_list::BestTransactionLog::new_empty(3);
    list.append(1, "INSERT INTO mytable VALUES (1)".to_owned());
    list.append(2, "INSERT INTO mytable VALUES (2)".to_owned());
    list.append(3, "INSERT INTO mytable VALUES (3)".to_owned());
    list.append(4, "INSERT INTO mytable VALUES (4)".to_owned());
    list.append(5, "INSERT INTO mytable VALUES (5)".to_owned());
    list.append(6, "INSERT INTO mytable VALUES (6)".to_owned());
    list.append(7, "INSERT INTO mytable VALUES (7)".to_owned());
    assert_eq!(list.length(), 7);
    assert_eq!(
        list.find(7),
        Some("INSERT INTO mytable VALUES (7)".to_owned())
    );
    assert_eq!(
        list.find(6),
        Some("INSERT INTO mytable VALUES (6)".to_owned())
    );
    assert_eq!(
        list.find(5),
        Some("INSERT INTO mytable VALUES (5)".to_owned())
    );
    assert_eq!(
        list.find(4),
        Some("INSERT INTO mytable VALUES (4)".to_owned())
    );
    assert_eq!(
        list.find(3),
        Some("INSERT INTO mytable VALUES (3)".to_owned())
    );
    assert_eq!(
        list.find(2),
        Some("INSERT INTO mytable VALUES (2)".to_owned())
    );
    assert_eq!(
        list.find(1),
        Some("INSERT INTO mytable VALUES (1)".to_owned())
    );
}