mod VectorNode;
mod Huffman;

use std::{collections::HashMap};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io;
use utf8_read::Reader;

fn main() {
    println!("Hello, world!");
}

fn frequency(file: String, map_freq: &mut HashMap<char, u32>) -> io::Result<usize> {
    let fs = File::open(file)?;
    let mut buf = Reader::new(&fs);
    let content = buf.into_iter();

    let mut num: usize = 0;
    let mut c: char;

    for r in content {
        c = r.expect("frequencia: Erro na leitura do arquivo de entrada");
        num += 1;
        match map_freq.get(&c) {
            None => {map_freq.insert(c, 1);},
            Some(v) => {map_freq.insert(c, v + 1);}
        }
    }

    Ok(num)
}

fn compression_map(file: &String) -> Result<(HashMap<char, u32>, HashMap<char, String>), ()> {
    let mut map: HashMap<char, u32> = HashMap::new();

    match frequency(file.to_string(), &mut map) {
            Ok(t) => {
                println!("O arquivo possui {t} caracteres.");
            },
            Err(_) => {println!("Arquivo {file} não existe.");}
    }

    let mut vector_node = VectorNode::VectorNode::new();
    vector_node.build_vector(&mut map);
    let node: Huffman::Huffman =  vector_node.build_tree();

    Ok((map, VectorNode::VectorNode::build_code(node)))
}

fn compression(file: String) -> io::Result<()> {
    let mut str_code: String = String::new();

    let (map_freq, map_code) = compression_map(&file).unwrap();
    
    let fs = File::open(file)?;
    let mut buf = Reader::new(&fs);
    let contet = buf.into_iter();

    let mut c: char;

    for r in contet {
        c = r.expect("frequencia: Erro na leitura do arquivo de entrada");

        str_code += map_code.get(&c).unwrap();
    }

    println!("MAP FREQ: {:?}", map_freq);
    println!("MAP FREQ: {:?}", map_code);
    println!("STR CODE: {}", str_code);

    let vec_bytes = divide_string(&str_code, 7);

    

    Ok(())
}

fn divide_string(s: &String, size: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut aux_s: String = String::new();

    let mut i: usize = 0;

    for c in s.chars(){
        if i == size {
            aux_s.push(c);
            v.push(aux_s);
            aux_s = String::new();
            i = 0;
        }else{
            aux_s.push(c);
            i += 1;
        }
    }

    if !(aux_s.len() == 0) {
        v.push(aux_s);
    }

    return v;
}

fn write(arq: String, map_freq: &HashMap<char, u32>) -> io::Result<()> {
    let mut fs = File::create_new(arq)?;
    let mut buffer = Cursor::new(Vec::new());

    buffer.write_u16::<BigEndian>(map_freq.len() as u16)?;

    for i in map_freq {
        buffer.write_u32::<BigEndian>(*i.0 as u32)?;
        buffer.write_u32::<BigEndian>(*i.1)?;
    }

    fs.write_all(buffer.get_mut())?;
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn frequency_test(){
        let mut map: HashMap<char, u32> = HashMap::new();

        let file = "test.txt";

        match frequency(file.to_string(), &mut map) {
            Ok(t) => {
                println!("O arquivo possui {t} caracteres.");
            },
            Err(_) => {println!("Arquivo {file} não existe.");}
            
        }

        assert_eq!(*map.get(&'a').unwrap(), 1);
        assert_eq!(*map.get(&'b').unwrap(), 2);
        assert_eq!(*map.get(&'c').unwrap(), 3);
        assert_eq!(*map.get(&'d').unwrap(), 4);
        assert_eq!(*map.get(&'e').unwrap(), 5);
        assert_eq!(*map.get(&'f').unwrap(), 6);

    }

    #[test]
    fn compression_test(){
        let mut map: HashMap<char, u32> = HashMap::new();

        let file = "test.txt";

        match frequency(file.to_string(), &mut map) {
            Ok(t) => {
                println!("O arquivo possui {t} caracteres.");
            },
            Err(_) => {println!("Arquivo {file} não existe.");}
            
        }

        compression(file.to_string());
    }

    #[test]
    fn divide_string_test(){
        let s: String = "0101010".to_string();

        let v_s = divide_string(&s, 1);

        assert_eq!("01".to_string(), v_s[0]);
        assert_eq!("01".to_string(), v_s[1]);
        assert_eq!("01".to_string(), v_s[2]);
        assert_eq!("0".to_string(), v_s[3]);
    }
}

