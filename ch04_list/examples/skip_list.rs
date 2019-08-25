extern crate ch04_list;

use rand::{Rng, thread_rng};

use ch04_list::skip_list::BestTransactionLog;

fn main() {
    let mut log = BestTransactionLog::new_empty(20);
    const LIST_ITEMS: u64 = 10;
    for i in 0..LIST_ITEMS {
        log.append(i, format!("INSERT DATA {}", i));
    }

    print!("{:?}", log);

    let mut rng = thread_rng();
    for _ in 0..LIST_ITEMS {
        let i = rng.gen_range(0, LIST_ITEMS);
        println!("{}", i);
        assert_eq!(log.find(i).expect("NOT FOUND"), format!("INSERT DATA {}", i));
    }
}