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

fn compression_map(file: &String) -> Result<(u32, HashMap<char, u32>, HashMap<char, String>), ()> {
    let mut map: HashMap<char, u32> = HashMap::new();
    let n: u32;

    match frequency(file.to_string(), &mut map) {
            Ok(t) => {
                n = t as u32;
                println!("O arquivo possui {t} caracteres.");
            },
            Err(_) => {panic!("Arquivo {file} não existe.");}
    }

    let mut vector_node = VectorNode::VectorNode::new();
    vector_node.build_vector(&mut map);
    let node: Huffman::Huffman =  vector_node.build_tree();

    Ok((n, map, VectorNode::VectorNode::build_code(node)))
}

fn compression(file: String, file_bin: String) -> io::Result<()> {
    let mut str_code: String = String::new();

    let (n, map_freq, map_code) = compression_map(&file).unwrap();
    
    let fs = File::open(file)?;
    let mut buf = Reader::new(&fs);
    let contet = buf.into_iter();

    let mut c: char;

    for r in contet {
        c = r.expect("frequencia: Erro na leitura do arquivo de entrada");

        str_code += map_code.get(&c).unwrap();
    }

    let vec_bytes = divide_string(&str_code, 7);

    write(file_bin, n, &map_freq, vec_bytes)?;

    Ok(())
}

fn complete_string(s: &mut String){
    let size = s.len();
    if size < 8{
        for _ in 0..(8 - size){
            s.push('0');
        }
    }
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
        complete_string(&mut aux_s);
        v.push(aux_s);
    }

    return v;
}

fn write(arq: String, n: u32, map_freq: &HashMap<char, u32>, vec_bytes: Vec<String>) -> io::Result<()> {

    println!();
    println!("LEM MAP: {}", map_freq.len());
    println!("N: {}", n);
    println!();

    let mut fs = File::create_new(arq)?;
    let mut buffer = Cursor::new(Vec::new());

    buffer.write_u16::<BigEndian>(map_freq.len() as u16)?;
    buffer.write_u32::<BigEndian>(n)?;

    for i in map_freq {
        buffer.write_u32::<BigEndian>(*i.0 as u32)?;
        buffer.write_u32::<BigEndian>(*i.1)?;
    }

    fs.write_all(buffer.get_mut())?;

    for b in vec_bytes{
        match u8::from_str_radix(&b, 2) {
            Ok(num) => {
                buffer.write_u8(num)?;
            },
            Err(e) => {panic!("Não foi possível escrever o byte no arquivo, Error: {}", e)}
        }
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
    fn compression_test() -> io::Result <()>{
        let mut map: HashMap<char, u32> = HashMap::new();

        let file = "test.txt";
        let file_bin = "test_bin.bin";

        match frequency(file.to_string(), &mut map) {
            Ok(t) => {
                println!("O arquivo possui {t} caracteres.");
            },
            Err(_) => {println!("Arquivo {file} não existe.");}
            
        }

        compression(file.to_string(), file_bin.to_string())?;

     
        let num_char_test: u32 = 23;

        // Le arquivo binário 
        let mut fs = File::open(file_bin.to_string())?;
        let mut v: Vec<u8> = Vec::new();
        fs.read_to_end(&mut v)?;
        let mut buffer = Cursor::new(v);

        /*
            Verifica se foi possui o mesmo número de caracteres distintos
        */
        let num_char_dist_test: u16 = map.len() as u16;
        let num_char_dist: u16 = buffer.read_u16::<BigEndian>().unwrap();

        println!("NUM CHAR DIST TEST: {}, NUM CHAR DIST: {}", num_char_dist_test, num_char_dist);
        assert_eq!(num_char_dist_test, num_char_dist);

        /*
            Verifica se foi possui o mesmo número de caracteres
        */
        let num_char_test: u32 = 23;
        let mum_char: u32 = buffer.read_u32::<BigEndian>().unwrap();

        println!("NUM CHAR TEST: {}, NUM CHAR: {}", num_char_test, mum_char);
        assert_eq!(num_char_test, mum_char);

        /*
            Verifica se o mapa de frequência lido no arquivo binário é igual ao 
            mapa de frequência criado pelo mapa criado para o arquivo text
        */
        let mut c:char;
        let mut f:u32;

        for _ in 0..num_char_dist{
            c = char::from_u32(buffer.read_u32::<BigEndian>().unwrap()).unwrap();
            f = buffer.read_u32::<BigEndian>().unwrap();
            println!("{c} - {f}");

            assert_eq!(map.get(&c), Some(&f))
        }

        Ok(())
    }

    #[test]
    fn divide_string_test(){
        let s: String = "0101010".to_string();

        let v_s = divide_string(&s, 1);

        assert_eq!("01".to_string(), v_s[0]);
        assert_eq!("01".to_string(), v_s[1]);
        assert_eq!("01".to_string(), v_s[2]);
        assert_eq!("00000000".to_string(), v_s[3]);
    }

}

