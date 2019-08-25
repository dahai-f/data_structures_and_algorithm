use std::fmt::Debug;

use rand::{Rng, RngCore, thread_rng};

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