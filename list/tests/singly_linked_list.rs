use list::singly_linked_list;
use list::singly_linked_list::TransactionLog;

#[test]
fn transaction_log_append() {
    let mut transaction_log = TransactionLog::new_empty();
    assert_eq!(transaction_log.length(), 0);
    assert_eq!(transaction_log.pop(), None);

    transaction_log.append("INSERT INTO my table VALUES (1,2,3)".to_string());
    transaction_log.append("INSERT INTO my table VALUES (2,3,4)".to_string());
    transaction_log.append("INSERT INTO my table VALUES (3,4,5)".to_string());
    assert_eq!(transaction_log.length(), 3);

    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (1,2,3)".to_string()));
    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (2,3,4)".to_string()));
    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (3,4,5)".to_string()));
    assert_eq!(transaction_log.pop(), None);

    transaction_log.append("INSERT INTO my table VALUES (4,5,6)".to_string());
    transaction_log.append("INSERT INTO my table VALUES (2,3,4)".to_string());
    transaction_log.append("INSERT INTO my table VALUES (3,4,5)".to_string());
    assert_eq!(transaction_log.length(), 3);

    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (4,5,6)".to_string()));
    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (2,3,4)".to_string()));
    assert_eq!(transaction_log.pop(), Some("INSERT INTO my table VALUES (3,4,5)".to_string()));
    assert_eq!(transaction_log.pop(), None);
}

#[test]
fn transaction_log_pop() {
    let mut list = singly_linked_list::TransactionLog::new_empty();
    assert_eq!(list.pop(), None);
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(
        list.pop(),
        Some("INSERT INTO my table VALUES (1,2,3)".to_owned())
    );
    assert_eq!(list.pop(), None);
}

#[test]
fn transaction_log_clone_and_pop() {
    let mut list = TransactionLog::new_empty();
    assert_eq!(list.pop(), None);
    list.append("INSERT INTO my table VALUES (1,2,3)".to_owned());
    list.append("INSERT INTO my table VALUES (2,3,4)".to_owned());
    list.append("INSERT INTO my table VALUES (3,4,5)".to_owned());

    let mut _log_clone = list.clone();

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