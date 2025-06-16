//! Módulo com a estrutura de dados que constroem a árvore binária para o Códifo Huffman
//! 
//! As seguintes funcionalidades estão disponíveis no módulo:
//! -> Criação do nó folha que contém o caracter e sua frequência
//! -> Criação dos nós intermediários que constroem a árvore 
//! -> Realiza a travessia da árvore binária construindo o código de Huffman 

use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub enum Huffman {
        Leaf {c: char, freq: u32},
        Node {freq: u32, left: Box<Huffman>, right: Box<Huffman>}
    }
    
    impl Huffman {
        pub fn new_leaf(c: char, freq: u32) -> Self {
            Huffman::Leaf{c, freq}
        }

        
        pub fn new_node(node_a: Huffman, node_b: Huffman) -> Self {
            let freq_a: u32 = match node_a {
                Self::Leaf { c: _, freq } => freq,
                Self::Node { freq, left: _, right: _ } => freq
            };

            let freq_b: u32 = match node_b {
                Self::Leaf { c: _, freq } => freq,
                Self::Node { freq, left: _, right: _ } => freq
            };
            
            Huffman::Node { 
                freq: freq_a + freq_b, 
                left: Box::new(node_a), 
                right: Box::new(node_b)
            }
        }

        pub fn get_freq(&mut self) -> u32{
            match self {
                Self::Leaf { c: _, freq } => *freq,
                Self::Node { freq, left: _, right: _ } => *freq
            }
        }
        
        pub fn traverse(&mut self, code: &mut String, i: usize) -> Option<(char, usize)>{
            match self {
                Huffman::Leaf { c, freq: _ } => {
                   return Some((*c, i));
                },
                Huffman::Node { freq: _, left, right } => {
                    if code.char_indices().nth(i) == Some((i, '0')){

                        return left.traverse(code, i + 1);

                    }else if code.char_indices().nth(i) == Some((i, '1')) {
                    
                        return  right.traverse(code, i + 1);
                    } else {
                        None
                    }
                }
            }
        }

        fn pre_order(&mut self, code: &mut String, map: &mut HashMap<char, String>) {
            match self {
                Huffman::Leaf { c, freq: _ } => {
                  map.insert(*c, code.clone());
                },

                Huffman::Node { freq: _, left, right } => {
                    code.push('0');
                    left.pre_order(code, map);

                    code.pop();
                    
                    code.push('1');
                    right.pre_order(code, map);

                    code.pop();
                }
            }
        }

        pub fn pre_order_code(&mut self, map: &mut HashMap<char, String>) {
            let mut code: String = String::from("");

            self.pre_order(&mut code, map);
        }

        pub fn compression_string(&mut self, code: String, map: &mut HashMap<char, String>) -> Option<String> {

            if code.is_empty() {return None;}

            let mut res: String = String::new();

            for c in code.chars(){  
                res += map.get(&c).unwrap();
            }

            return Some(res);
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn new_leaf_test(){
            let leaf: Huffman = Huffman::new_leaf('a', 32);

            match  leaf {
                Huffman::Leaf { c, freq } => {
                    assert_eq!(c, 'a');
                    assert_eq!(freq, 32);
                }
                _ => panic!("O Nó tem que ser uma folha")
            }
        }

        #[test]
        fn new_node_with_children_leaves_test(){
            let leaf_a: Huffman = Huffman::new_leaf('a', 1);
            let leaf_b: Huffman = Huffman::new_leaf('b', 1);

            let node: Huffman = Huffman::new_node(leaf_a, leaf_b);

            match node {
                Huffman::Node { freq, left, right } => {
                    assert_eq!(freq, 2);

                    match  *left {
                        Huffman::Leaf { c, freq } => {
                            assert_eq!(c, 'a');
                            assert_eq!(freq, 1);
                        }
                         _ => panic!("O Nó tem que ser uma folha")
                    }

                    match  *right {
                        Huffman::Leaf { c, freq } => {
                            assert_eq!(c, 'b');
                            assert_eq!(freq, 1);
                        }
                         _ => panic!("O Nó tem que ser uma folha")
                    }
            }
                _ => panic!("O Nó tem que ser um node")
            }
        }

        #[test]
        pub fn new_node_with_children_node_test(){
            let leaf_a: Huffman = Huffman::new_leaf('a', 1);
            let leaf_b: Huffman = Huffman::new_leaf('b', 1);
            let leaf_c: Huffman = Huffman::new_leaf('c', 1);

            let node_d: Huffman = Huffman::new_node(leaf_a, leaf_b);
            let node_e: Huffman = Huffman::new_node(node_d, leaf_c);


            match  node_e {
                Huffman::Node { freq, left, right } => {
                    match *left {
                        Huffman::Node { freq, left, right } => {
                            assert_eq!(freq, 2);

                            match *left {
                                Huffman::Leaf { c, freq } => {
                                    assert_eq!(c, 'a');
                                    assert_eq!(freq, 1);
                                },
                                _ => panic!("O Nó tem que ser uma folha")
                            }


                            match *right {
                                Huffman::Leaf { c, freq } => {
                                    assert_eq!(c, 'b');
                                    assert_eq!(freq, 1);
                                },
                                _ => panic!("O Nó tem que ser uma folha")
                            }


                        },
                        _ => panic!("O Nó tem que ser um node")
                    }

                    match  *right {
                        Huffman::Leaf { c, freq } => {
                            assert_eq!(c, 'c');
                            assert_eq!(freq, 1);

                        },
                        _ => panic!("O Nó tem que ser uma folha")
                    }

                    assert_eq!(freq, 3);

                },
                _ => panic!("O Nó tem que ser um node")
            }      
        }

        #[test]
        pub fn traverse_test(){
            let mut code_00: String = String::from("00");
            let mut code_01: String = String::from("01");
            let mut code_10: String = String::from("10");
            let mut code_11: String = String::from("11");


            let leaf_a: Huffman = Huffman::new_leaf('a', 1);
            let leaf_b: Huffman = Huffman::new_leaf('b', 2);
            let leaf_c: Huffman = Huffman::new_leaf('c', 3);
            let leaf_d: Huffman = Huffman::new_leaf('d', 4);

            let node_e: Huffman = Huffman::new_node(leaf_a, leaf_b);
            let node_f: Huffman = Huffman::new_node(leaf_c, leaf_d);


            let mut node_g: Huffman = Huffman::new_node(node_e, node_f);

            println!("Code_00: {} = a", code_00);

            assert_eq!(node_g.traverse(&mut code_00, 0), Some(('a', 2)));

            println!("Code_00: {} = b", code_01);

            assert_eq!(node_g.traverse(&mut code_01, 0), Some(('b', 2)));

            println!("Code_10: {} = c", code_10);

            assert_eq!(node_g.traverse(&mut code_10, 0), Some(('c', 2)));

            println!("Code_11: {} = d", code_11);

            assert_eq!(node_g.traverse(&mut code_11, 0), Some(('d', 2)));

        }
        
        #[test]
        pub fn pre_order_test(){
            let mut map: HashMap<char, String> = HashMap::new();

            let leaf_a: Huffman = Huffman::new_leaf('a', 1);
            let leaf_b: Huffman = Huffman::new_leaf('b', 2);
            let leaf_c: Huffman = Huffman::new_leaf('c', 3);
            let leaf_d: Huffman = Huffman::new_leaf('d', 4);

            let node_e: Huffman = Huffman::new_node(leaf_a, leaf_b);
            let node_f: Huffman = Huffman::new_node(leaf_c, leaf_d);

            let mut node_g: Huffman = Huffman::new_node(node_e, node_f);

            node_g.pre_order_code(&mut map);

            assert_eq!(map.get(&'a').unwrap(), "00");
            assert_eq!(map.get(&'b').unwrap(), "01");
            assert_eq!(map.get(&'c').unwrap(), "10");
            assert_eq!(map.get(&'d').unwrap(), "11");
        }

        #[test]
        pub fn get_freq_test(){
            let mut node: Huffman = Huffman::new_leaf('a', 1);

            assert_eq!(node.get_freq(), 1)
        }

        #[test]
        pub fn compression_string_test(){
            let mut map: HashMap<char, String> = HashMap::new();

            let leaf_a: Huffman = Huffman::new_leaf('a', 1);
            let leaf_b: Huffman = Huffman::new_leaf('b', 2);
            let leaf_c: Huffman = Huffman::new_leaf('c', 3);
            let leaf_d: Huffman = Huffman::new_leaf('d', 4);

            let node_e: Huffman = Huffman::new_node(leaf_a, leaf_b);
            let node_f: Huffman = Huffman::new_node(leaf_c, leaf_d);

            let mut node_g: Huffman = Huffman::new_node(node_e, node_f);

            node_g.pre_order_code(&mut map);

            let res: String = node_g.compression_string("abc".to_string(), &mut map).unwrap();

            assert_eq!(res, "000110"); 

        }
    }
