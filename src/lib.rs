use std::vec::Vec;

pub struct Node {
  is_end: bool,
  leaf: [u16;256],
}

impl Node {
  pub fn new() -> Node {
    Node {
      is_end: false,
      leaf: [0;256],
    }
  }
}

pub struct Trie {
  nodes: Vec<Node>,
}

impl Trie {
  pub fn new() -> Trie {
    Trie {
      nodes: vec![Node::new()],
    }
  }
  pub fn add(&mut self, target: &str) {
    let bytes = target.as_bytes();
    let mut nodes_len = self.nodes.len();
    let nodes: &mut Vec<Node> = &mut self.nodes;
    let mut node: &mut Node = &mut nodes[0];
    for i in 0..bytes.len() {
      if node.is_end {
        return
      }
      let mut n = node.leaf[bytes[i] as usize];
      if n == 0 {
        n = nodes_len as u16;
        node.leaf[bytes[i] as usize] = n;
        nodes.push(Node::new());
        nodes_len += 1;
      }
      node = &mut nodes[n as usize];
    }
    node.is_end = true
  }
  fn matches(&self, bytes: &[u8], from: usize) -> bool {
    let mut node = &self.nodes[0];
    for i in from..bytes.len() {
      if node.is_end {
        return true
      }
      let n = node.leaf[bytes[i] as usize];
      if n == 0 {
        return false
      }
      node = &self.nodes[n as usize];
    }
    node.is_end
  }
  pub fn starts(&self, target: &str) -> bool {
    return self.matches(target.as_bytes(), 0);
  }
  pub fn contains(&self, target: &str) -> bool{
    let bytes = target.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
      if self.matches(bytes, i) {
        return true
      }
      let b = bytes[i];
      if b&0b11110000 == 0b11110000 {
        i += 4
      } else if b&0b11100000 == 0b11100000 {
        i += 3
      } else if b&0b11000000 == 0b11000000 {
        i += 2
      } else {
        i += 1
      }
    }
    return false;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_zoi() {
    let mut tr = Trie::new();
    tr.add("ぞい！");
    assert!(tr.starts("ぞい！"));
    assert!(tr.contains("今日も一日がんばるぞい！"));
    assert!(!tr.starts("zoi"));
  }

  #[test]
  fn test_zoizoi() {
    let mut tr = Trie::new();
    tr.add("ぞい！");
    tr.add("zoi");
    tr.add("java");
    tr.add("ぞい");
    assert!(tr.starts("zoi"));
    assert!(tr.starts("ぞい！"));
    assert!(!tr.starts("z"));
    assert!(!tr.starts("ganbaruzoi"));
    assert!(!tr.starts("nenetch"));
    assert!(tr.contains("ganbaruzoi!"));
    assert!(tr.contains("今日も一日がんばるぞい！"));
    assert!(tr.contains("anatatojava"));
    assert!(tr.contains("anatatojava, imasugu download"));
  }

  #[test]
  fn test_contains(){
    let mut tr = Trie::new();
    tr.add("うらにわ");
    tr.add("おおにわとり");
    tr.add("こけこっこ");
    tr.add("ok");
    assert!(!tr.contains("にわにはにわにわとりがいる"));
    assert!(tr.contains("にわにはにわおおにわとりがいる"));
    assert!(!tr.contains("コケコッコー"));
    assert!(!tr.contains("POKEMON"));
  }

  #[test]
  fn test_empty() {
    let tr = Trie::new();
    assert!(!tr.starts(""));
    assert!(!tr.starts("zoi"));
  }
}
