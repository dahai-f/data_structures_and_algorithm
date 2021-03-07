use std::collections::HashMap;

struct Node<T> {
    next: HashMap<char, Node<T>>,
    value: Option<T>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            next: Default::default(),
            value: None,
        }
    }
}

pub struct Trie<T> {
    root: HashMap<char, Node<T>>,
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            root: Default::default(),
        }
    }
}

impl<T> Trie<T> {
    pub fn add(&mut self, path: &str, value: T) -> Option<T> {
        let mut chars = path.chars();
        if let Some(c) = chars.next() {
            let mut node = self.root.entry(c).or_insert_with(Node::default);
            for c in chars {
                node = node.next.entry(c).or_insert_with(Node::default);
            }
            let mut value = Some(value);
            std::mem::swap(&mut value, &mut node.value);
            value
        } else {
            None
        }
    }

    pub fn find(&self, path: &str) -> Option<&T> {
        let mut path = path.chars();
        if let Some(c) = path.next() {
            let mut node = self.root.get(&c);
            for c in path {
                match node {
                    None => {
                        return None;
                    }
                    Some(n) => {
                        node = n.next.get(&c);
                    }
                }
            }
            node.and_then(|node| node.value.as_ref())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn add_and_find() {
        let mut trie = Trie::default();
        assert_eq!(trie.add("a b c d e f g", "a b c d e f g"), None);
        assert_eq!(
            trie.add("a b c d e f g", "a b c d e f g"),
            Some("a b c d e f g")
        );
        assert_eq!(trie.find("a b c d e f g"), Some(&"a b c d e f g"));
        assert_eq!(trie.find(""), None);
        assert_eq!(trie.find("dfd"), None);
        assert_eq!(trie.add("", ""), None);
        assert_eq!(trie.find(""), None);
    }
}
