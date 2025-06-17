use std::collections::HashMap;

use crate::Huffman::{Huffman};

#[derive(Debug, Clone)]
pub struct VectorNode {
    vec: Vec<Huffman>,
    min_index: Option<usize>
}

impl VectorNode {
    pub fn new() -> Self {
        VectorNode{ vec: Vec::new(), min_index: None }
    }

    pub fn build_vector(&mut self, map: &mut HashMap<char, u32>) {

      for (c, f) in map {
        let node: Huffman = Huffman::new_leaf(*c, *f);
        self.push(node);
      }
    }

    pub fn push(&mut self, mut node: Huffman) {
        self.vec.push(node);
        self.select_min();

      // if self.min_index.is_none(){
      //   self.min_index = Some(0);
      //   self.vec.push(node);
      // }else{
      //   if node.get_freq() < self.vec[self.min_index.unwrap()].get_freq(){
      //     self.min_index = Some(self.min_index.unwrap() + 1);
      //     println!("MIN INDEX: {}", self.min_index.unwrap());
      //     self.vec.push(node);
      //   }else{
      //     self.vec.push(node);
      //   }
      // }
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

    pub fn traverse_code(node: &mut Huffman, code: &mut String) -> String{

      if code.is_empty() {return  String::new();}

      let mut res: String = String::new();

      let mut c: char;
      let mut i: usize = 0;

      while i < code.len() {
         (c, i) = node.traverse(code, i).unwrap();
         res.push(c);
      }

      return res;
    }

    pub fn traverse_code_num_char(node: &mut Huffman, code: &mut String, num_char: u32) -> String{

      if code.is_empty() {return  String::new();}

      let mut res: String = String::new();
      let mut cont: u32 = 0;

      let mut c: char;
      let mut i: usize = 0;

      while cont < num_char {
         (c, i) = node.traverse(code, i).unwrap();
         res.push(c);
         cont += 1;
      }

      return res;
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
  fn build_vector_test(){

  }

  #[test]
  fn push_vector_node_test(){
     let mut map: HashMap<char, u32> = HashMap::new();

     let mut vector_node: VectorNode = VectorNode::new();

     map.insert('a', 1);
     map.insert('b', 1);
     map.insert('c', 1);
     map.insert('d', 1);

     vector_node.build_vector(&mut map);

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

  #[test]
  fn build_traverse_code_test(){
    let mut vector_node: VectorNode = VectorNode::new();

    vector_node.push(Huffman::new_leaf('a', 1));
    vector_node.push(Huffman::new_leaf('b', 2));
    vector_node.push(Huffman::new_leaf('c', 3));
    vector_node.push(Huffman::new_leaf('d', 4));

    let mut node: Huffman = vector_node.build_tree();

    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "".to_string());

    assert_eq!(String::new(), unpacked);

    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "0".to_string());

    assert_eq!("d".to_string(), unpacked);

    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "10".to_string());

    assert_eq!("c".to_string(), unpacked);

    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "110".to_string());

    assert_eq!("a".to_string(), unpacked);

    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "111".to_string());

    assert_eq!("b".to_string(), unpacked);

    println!("");
    let unpacked: String = VectorNode::traverse_code(&mut node, &mut "1101111111010100000".to_string());

    assert_eq!("abbcccdddd".to_string(), unpacked);

  }

  #[test]
  fn traverse_code_num_char_test(){
    let mut vector_node: VectorNode = VectorNode::new();

    vector_node.push(Huffman::new_leaf('a', 1));
    vector_node.push(Huffman::new_leaf('b', 2));
    vector_node.push(Huffman::new_leaf('c', 3));
    vector_node.push(Huffman::new_leaf('d', 4));

    let mut node: Huffman = vector_node.build_tree();

    let unpacked: String = VectorNode::traverse_code_num_char(&mut node, &mut "110111111101010000001111111111111111111111111".to_string(), 10);
    assert_eq!("abbcccdddd".to_string(), unpacked);
  }

}