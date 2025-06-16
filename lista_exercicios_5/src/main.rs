mod VectorNode;
mod Huffman;

use std::{collections::HashMap};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io;
use utf8_read::{Reader, StreamPosition};

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

    // println!("NODE TEST: {:?}", node);

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

fn write(file: String, n: u32, map_freq: &HashMap<char, u32>, vec_bytes: Vec<String>) -> io::Result<()> {
    let mut fs = File::create_new(file)?;
    let mut buffer = Cursor::new(Vec::new());

    buffer.write_u16::<BigEndian>(map_freq.len() as u16)?;
    buffer.write_u32::<BigEndian>(n)?;

    for i in map_freq {
        buffer.write_u32::<BigEndian>(*i.0 as u32)?;
        buffer.write_u32::<BigEndian>(*i.1)?;
    }

    println!("LEN: {}", vec_bytes.len());
    for b in vec_bytes{
        buffer.write_u8(convert_string_to_u8(b))?;
    }

    fs.write_all(buffer.get_mut())?;

    Ok(())
}

fn read(file: String) -> io::Result<(u32, Huffman::Huffman, Vec<u8>)> {
    let mut map_freq: HashMap<char, u32> = HashMap::new();
    let mut v_u8: Vec<u8> = Vec::new();

    let mut fs = File::open(file)?;
    let mut v: Vec<u8> = Vec::new();
    fs.read_to_end(&mut v)?;
    let mut buffer = Cursor::new(v);

    let num_char_dist: u16 = buffer.read_u16::<BigEndian>().unwrap();
    let mum_char: u32 = buffer.read_u32::<BigEndian>().unwrap();

    let mut c:char;
    let mut f:u32;

    for _ in 0..num_char_dist{
        c = char::from_u32(buffer.read_u32::<BigEndian>().unwrap()).unwrap();
        f = buffer.read_u32::<BigEndian>().unwrap();

        map_freq.insert(c, f);
    }

    let mut vector_node = VectorNode::VectorNode::new();
    vector_node.build_vector(&mut map_freq);
    let node: Huffman::Huffman =  vector_node.build_tree();

    loop{
        if let Ok(b) =  buffer.read_u8(){
            v_u8.push(b);  
        }else{
            break;
        }
    }

    println!("LEN: {}", v_u8.len());

    Ok((mum_char, node, v_u8 ))
}

fn add_bit(b: u8, c: char, i: usize) -> u8{
    if c == '1'{
        return b | (1 << i);
    }

    return b;
}

fn bit_mask(b: u8, i: usize) -> char{
    if i <= 7 {
        let m: u8 = 0 | (1 << i);

        if b & m == 0 {
            return '0';
        }else {
            return '1';
        }
    }else{
        panic!("O Indice i de ser de 0 até 7");
    }
}

fn convert_string_to_u8(s: String) -> u8{
    let mut b: u8 = 0b0000_0000;

    for (i, c) in s.chars().rev().enumerate(){
        b = add_bit(b, c, i);
    }

    return b;
}

fn convert_u8_to_string(b: u8) -> String{
    let mut s: String = String::new();

    for i in (0..8).rev(){
        s.push(bit_mask(b, i));
    }

    return s;
}


fn build_str_code(v_u8: &Vec<u8>) -> String{
    let mut str_code: String = String::new();

    println!("VEC U8: {:?}", v_u8);
    for b in v_u8{
        str_code += &convert_u8_to_string(*b);
        println!("STR CODE: {}", &convert_u8_to_string(*b));
    }


    return str_code;
}

fn write_txt(file_txt: String, text: String) -> io::Result<()>{
    let mut file = File::create(file_txt)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn decompressor(file: String, file_txt: String) -> io::Result<()>{
    let (mum_char, mut node, v_u8) = read(file)?;
    let mut str_code: String = build_str_code(&v_u8);

    let text: String = VectorNode::VectorNode::traverse_code_num_char(&mut node, &mut str_code, mum_char);
    println!("STR CODE: {}", str_code);
    println!("TEXT: {}", text);

    write_txt(file_txt, text)?;

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
        let num_char_test: u32 = 21;
        let mum_char: u32 = buffer.read_u32::<BigEndian>().unwrap();

        println!("NUM CHAR TEST: {}, NUM CHAR: {}", num_char_test, mum_char);
        assert_eq!(num_char_test, mum_char);

        /*
            Verifica se o mapa de frequência lido no arquivo binário é igual ao 
            mapa de frequência criado pelo mapa criado para o arquivo text
        */
        let mut map_test: HashMap<char, u32> = HashMap::new();
        let mut c:char;
        let mut f:u32;

        for _ in 0..num_char_dist{
            c = char::from_u32(buffer.read_u32::<BigEndian>().unwrap()).unwrap();
            f = buffer.read_u32::<BigEndian>().unwrap();
            println!("{c} - {f}");

            map_test.insert(c, f);
        }

        println!("MAP: {:?}", map);
        println!("MAP TEST: {:?}", map_test);
        assert_eq!(map, map_test);


        /*
            Verifica quantos u8 foram escritas no arquivo binário
         */
        let mut v_u8: Vec<u8> = Vec::new();
        let qtd_u8_test: u32 = 7;
        let mut cont_qtd_u8: u32 = 0;

        loop{
            if let Ok(b) =  buffer.read_u8(){
                v_u8.push(b);  
                cont_qtd_u8 += 1;
            }else{
                break;
            }
        }

        println!("QTD U8 TEST: {},  CONT QTD U8: {}", qtd_u8_test, cont_qtd_u8);
        assert_eq!(qtd_u8_test, cont_qtd_u8);

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

    #[test]
    fn read_test() -> io::Result <()>{
        let file_bin = "test_bin.bin";

        let num_char_test: u32 = 23;
        let (mum_char, _, _) = read(file_bin.to_string())?;

        assert_eq!(num_char_test, mum_char);

        Ok(())
    }

    #[test]
    fn add_bit_test(){
        assert_eq!(0b0000_0001 as u8, add_bit(0b0000_0000, '1', 0));
        assert_eq!(0b0000_0010 as u8, add_bit(0b0000_0000, '1', 1));
        assert_eq!(0b0000_0000 as u8, add_bit(0b0000_0000, '0', 1));
        assert_eq!(0b1000_0000 as u8, add_bit(0b0000_0000, '1', 7));
    }

    #[test]
    fn convert_string_to_u8_test(){
        let s = "00001111".to_string();

        assert_eq!(0b0000_1111, convert_string_to_u8(s));
    }
    
    #[test]
    fn bit_mask_test() {
        let b: u8 = 0b0101_0101;

        assert_eq!(bit_mask(b, 0), '1');
        assert_eq!(bit_mask(b, 1), '0');
        assert_eq!(bit_mask(b, 2), '1');
        assert_eq!(bit_mask(b, 3), '0');

        assert_eq!(bit_mask(b, 4), '1');
        assert_eq!(bit_mask(b, 5), '0');
        assert_eq!(bit_mask(b, 6), '1');
        assert_eq!(bit_mask(b, 7), '0');
    }

    #[test]
    fn convert_u8_to_string_test() {
        let b: u8 = 0b0101_0101;

        assert_eq!(convert_u8_to_string(b), "01010101".to_string());
    }

    #[test]
    fn decompressor_test(){
        let file_bin = "test_bin.bin";
        let file_txt = "text_txt_test.txt";

        decompressor(file_bin.to_string(), file_txt.to_string()).unwrap();
    }
}

