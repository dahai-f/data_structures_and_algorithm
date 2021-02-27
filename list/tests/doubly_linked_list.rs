#![feature(test)]

extern crate test;

use test::Bencher;

use rand::Rng;

use list::doubly_linked_list::BetterTransactionLog;

const LIST_ITEMS: usize = 15_000;

#[bench]
fn bench_linked_list_find(b: &mut Bencher) {
    let mut log = BetterTransactionLog::new_empty();
    let items: Vec<String> = (0..LIST_ITEMS).map(|i| {
        format!("INSERT INTO my table VALUES ({})", i).to_string()
    }).collect();

    for item in items.iter() {
        log.append(item.clone());
    }

    let mut rng = rand::thread_rng();

    b.iter(|| {
        let i = rng.gen_range(0..LIST_ITEMS);
        log.iter().find(|item| item == &&items[i]).expect("NOT FOUND");
    });
}

#[test]
fn transaction_log_append() {
    let mut transaction_log = BetterTransactionLog::new_empty();
    assert_eq!(transaction_log.length(), 0);
    transaction_log.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    transaction_log.append("INSERT INTO my table VALUES (2,3,4)".to_owned());
    transaction_log.append("INSERT INTO my table VALUES (3,4,5)".to_owned());
    assert_eq!(transaction_log.length(), 3);
    assert_eq!(
        transaction_log.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        transaction_log.pop(),
        Some("INSERT INTO my table VALUES (2,3,4)".to_owned())
    );
    assert_eq!(
        transaction_log.pop(),
        Some("INSERT INTO my table VALUES (3,4,5)".to_owned())
    );
    assert_eq!(transaction_log.pop(), None);
}

#[test]
fn better_transaction_log_pop() {
    let mut list = BetterTransactionLog::new_empty();
    assert_eq!(list.pop(), None);
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (1,2,4)".to_owned());
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,4)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(list.pop(), None);
}

#[test]
fn better_transaction_log_iterator() {
    let mut list = BetterTransactionLog::new_empty();
    assert_eq!(list.pop(), None);
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (2,3,4)".to_owned());
    list.append("INSERT INTO my table VALUES (3,4,5)".to_owned());
    let mut iter = list.iter();
    assert_eq!(
        iter.next(),
        Some(&"INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        iter.next(),
        Some(&"INSERT INTO my table VALUES (2,3,4)".to_owned())
    );
    assert_eq!(
        iter.next(),
        Some(&"INSERT INTO my table VALUES (3,4,5)".to_owned())
    );

    let mut iter = list.back_iter();
    assert_eq!(
        iter.next_back(),
        Some(&"INSERT INTO my table VALUES (3,4,5)".to_owned())
    );
    assert_eq!(
        iter.next_back(),
        Some(&"INSERT INTO my table VALUES (2,3,4)".to_owned())
    );
    assert_eq!(
        iter.next_back(),
        Some(&"INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
}

#[test]
fn better_transaction_log_clone_and_pop() {
    let mut list = BetterTransactionLog::new_empty();
    assert_eq!(list.pop(), None);
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (2,3,4)".to_owned());
    list.append("INSERT INTO my table VALUES (3,4,5)".to_owned());

    let mut iter = list.back_iter();
    assert_eq!(
        iter.next_back(),
        Some(&"INSERT INTO my table VALUES (3,4,5)".to_owned())
    );
    assert_eq!(
        iter.next_back(),
        Some(&"INSERT INTO my table VALUES (2,3,4)".to_owned())
    );

    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (2,3,4)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (3,4,5)".to_owned())
    );
}