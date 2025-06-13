use std::collections::HashMap;

use crate::Huffman::{Huffman};

#[derive(Debug, Clone)]
struct VectorNode {
    vec: Vec<Huffman>,
    min_index: Option<usize>
}

impl VectorNode {
    pub fn new() -> Self {
        VectorNode{ vec: Vec::new(), min_index: None }
    }

    pub fn push(&mut self, mut node: Huffman) {
      if self.min_index.is_none(){
        self.min_index = Some(0);
        self.vec.push(node);
      }else{
        if node.get_freq() < self.vec[self.min_index.unwrap()].get_freq(){
          self.min_index = Some(self.min_index.unwrap() + 1);
          self.vec.push(node);
        }else{
          self.vec.push(node);
        }
      }
    }

    pub fn select_min(&mut self){
      let mut min: Option<usize> = None;

      for i in 0..self.vec.len(){
          if min.is_some(){

            if self.vec[i].get_freq() < self.vec[min.unwrap()].get_freq(){
              min = Some(i);
            }
          }else{
              min = Some(i)
          }
      }

      self.min_index = min;
    }

    pub fn min(&mut self) -> Option<Huffman>{
       if self.min_index.is_none(){
        None
       }else{
        let i: usize = self.min_index.unwrap();
        Some(self.vec.remove(i))
       }
    }

    pub fn build_tree(&mut self) -> Huffman {
      while self.vec.len() > 1 {
          let mut node_a: Huffman = self.min().unwrap();

          self.select_min();

          let mut node_b: Huffman = self.min().unwrap();

          self.select_min();

          let mut node: Huffman = Huffman::new_node(node_a, node_b);

          self.push(node);
      }

      if self.vec.len() == 1 {
        // let mut map: HashMap<char, String> = HashMap::new();

        // node.pre_order_code(&mut map);

        return self.min().unwrap();

      }else{
        panic!("Error: there should be only one node");
      }
    }

    pub fn build_code(mut node: Huffman) -> HashMap<char, String>{
        let mut map: HashMap<char, String> = HashMap::new();

        node.pre_order_code(&mut map);

        return map;
    }

}

#[cfg(test)]
mod tests {
  use std::iter::Enumerate;

use super::*;

  #[test]
  fn new_vector_node_test(){
    let vector_node = VectorNode::new();

    assert_eq!(vector_node.vec.len(), 0);
    assert_eq!(vector_node.min_index, None);
  }

  #[test]
  fn push_vector_node_test(){
     let mut vector_node: VectorNode = VectorNode::new();

     vector_node.push(Huffman::new_leaf('a', 1));
     vector_node.push(Huffman::new_leaf('b', 2));
     vector_node.push(Huffman::new_leaf('c', 3));
     vector_node.push(Huffman::new_leaf('d', 4));

     assert_eq!(vector_node.vec.len(), 4);
  }

  #[test]
  fn min_vector_node_test(){
    let mut vector_node = VectorNode::new();

    vector_node.push(Huffman::new_leaf('a', 1));
    vector_node.push(Huffman::new_leaf('b', 2));
    vector_node.push(Huffman::new_leaf('c', 3));
    vector_node.push(Huffman::new_leaf('d', 4));

    let mut min_node: Huffman = vector_node.min().unwrap();
    vector_node.select_min();

    assert_eq!(vector_node.vec.len(), 3);
    assert_eq!(min_node.get_freq(), 1);

    let mut min_node: Huffman = vector_node.min().unwrap();
    vector_node.select_min();

    assert_eq!(vector_node.vec.len(), 2);
    assert_eq!(min_node.get_freq(), 2);

    let mut min_node: Huffman = vector_node.min().unwrap();
    vector_node.select_min();

    assert_eq!(vector_node.vec.len(), 1);
    assert_eq!(min_node.get_freq(), 3);

    let mut min_node: Huffman = vector_node.min().unwrap();
    vector_node.select_min();

    assert_eq!(vector_node.vec.len(), 0);
    assert_eq!(min_node.get_freq(), 4);
  }

  #[test]
  fn build_tree_test(){
     let mut vector_node: VectorNode = VectorNode::new();

     vector_node.push(Huffman::new_leaf('a', 1));
     vector_node.push(Huffman::new_leaf('b', 2));
     vector_node.push(Huffman::new_leaf('c', 3));
     vector_node.push(Huffman::new_leaf('d', 4));

     let mut node: Huffman = vector_node.build_tree();

     println!("NODE: {:?}", node);

     assert_eq!(vector_node.vec.len(), 0);

     println!("FREQ: {}", node.get_freq());
     assert_eq!(node.get_freq(), 10);
  }

  #[test]
  fn buid_code_test(){
     let mut vector_node: VectorNode = VectorNode::new();

     vector_node.push(Huffman::new_leaf('a', 1));
     vector_node.push(Huffman::new_leaf('b', 2));
     vector_node.push(Huffman::new_leaf('c', 3));
     vector_node.push(Huffman::new_leaf('d', 4));

     let mut node: Huffman = vector_node.build_tree();

     let map = VectorNode::build_code(node);

      assert_eq!(map.get(&'d').unwrap(), "0");
      assert_eq!(map.get(&'c').unwrap(), "10");
      assert_eq!(map.get(&'a').unwrap(), "110");
      assert_eq!(map.get(&'b').unwrap(), "111");

  }

}