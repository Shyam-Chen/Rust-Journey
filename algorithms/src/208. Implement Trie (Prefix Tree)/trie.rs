use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }

        node.is_end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        for c in word.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return false,
            }
        }

        node.is_end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;

        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
#[path = "./trie_test.rs"]
mod tests;
