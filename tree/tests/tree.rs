use tree::binary::*;

struct IoTDevice {
    id: u64,
    _address: String,
}

#[test]
fn find() {}

#[test]
fn walk() {
    let mut set = BinarySearchTree::default();
    set.add(
        4,
        IoTDevice {
            id: 4,
            _address: "".to_string(),
        },
    );
    set.add(
        7,
        IoTDevice {
            id: 7,
            _address: "".to_string(),
        },
    );
    set.add(
        8,
        IoTDevice {
            id: 8,
            _address: "".to_string(),
        },
    );
    set.add(
        2,
        IoTDevice {
            id: 2,
            _address: "".to_string(),
        },
    );
    set.add(
        1,
        IoTDevice {
            id: 1,
            _address: "".to_string(),
        },
    );
    set.add(
        3,
        IoTDevice {
            id: 3,
            _address: "".to_string(),
        },
    );
    set.add(
        6,
        IoTDevice {
            id: 6,
            _address: "".to_string(),
        },
    );
    set.add(
        9,
        IoTDevice {
            id: 9,
            _address: "".to_string(),
        },
    );
    set.add(
        5,
        IoTDevice {
            id: 5,
            _address: "".to_string(),
        },
    );

    let mut items = Vec::default();
    set.walk(|(_item, dev)| items.push(dev.id));
    assert_eq!(items, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
