/*
Exercício 6

Implementar um programa multithread que calcule os número primos entre 2 e n (u128::MAX), 
tendo como limite para n o maior valor armazenado em um dado u128.
*/

use std::sync::{Arc, RwLock};

// UNI THREAD

fn bit_is_marked(v: &Vec<u128>, i: usize, j: usize) -> bool{
    /*
    i posição do u128 no vector
    j posição do bit no u128
    
    Se o bit estiver em 0 não está marcado podendo ser um possível número primo,
    Se estiver em 1 o bit está marcado e não um número primo
    */
    
    if j <= 127 {
        let mask: u128 = 0 | (1 << j);
        
        if v[i] & mask == 0 {
            return false;
        }else{
            return true;
        }
    }else{
        panic!("O indice j deve ser de 0 até 127")
    }
}

fn add_bit(v: &mut Vec<u128>, i: usize, j: usize) {
    if j <= 127 {
        v[i] = v[i] | (1u128 << j);
    }else{
        panic!("O indice j deve ser de 0 até 127")
    }
    
}

fn value_of_bit(i: usize, j: usize) -> u128 {
    if (127 * i + j) == 0 { 2 } else {(2 * (127 * i + j) + 1) as u128}
}

fn get_value_in_bit_mask(v: &Vec<u128>, i: usize, j: usize) -> Option<u128> {
    /*
    i posição do u128 no vector
    j posição do bit no u128
    
    Retorna o número u128 presente no u128 na posição j
    estando na posição i do vector
    */
    
    if !bit_is_marked(v, i, j) {Some(value_of_bit(i, j))} else {None}
}

fn next_first_position(a: u128) -> (usize, usize){
    let i: u128 = (a - 1) / 254;

    let j: u128 = (a - 254 * i - 1) / 2;

    (i as usize, j as usize)
}

fn gen_natural_numbers(n: u128) -> Vec<u128>{
    let len_v: usize = if n <= 253 {
        1
    } else {
        ((n.abs_diff(255)) / 254 
        + if (n.abs_diff(255)) % 254 > 0 {1} else {0}
        + 1) as usize
    };

    let mut v: Vec<u128> = vec![0; len_v];

    for j in 0..127 {
        if value_of_bit(len_v - 1, j) > n {
            add_bit(&mut v, len_v -1, j);
        }
    }


    return v;
}

fn search_next_first_position(v: &mut Vec<u128>, p: u128) -> Option<(usize, usize)> {
    let mut new_p = p;
    loop {
        new_p += 2;
        let (k, w) = next_first_position(new_p);
        
        if k > v.len() - 1{
            return None;
        }

        if let Some(_) = get_value_in_bit_mask(&v, k, w){
            return Some((k, w));
        }

    }
}

fn eratosthenes(n: u128) -> Option<Vec<u128>> {
    if n < 2 {
        return None;
    }else if n == 2{
        return Some(vec![2]);
    }else{    
        let mut v: Vec<u128> = gen_natural_numbers(n);
        let stopping: u128 = n.isqrt();

        let mut res: Vec<u128> = vec![2];

        let mut k: usize = 0;
        let mut w: usize = 1;

        let mut c: i32 = 0;

        while value_of_bit(k, w) <= stopping {
            let p: u128 = value_of_bit(k, w);
            res.push(p);
            
            let (a, mut b) = next_first_position(p.pow(2));

            for i in a..v.len(){
                if p != u128::MAX{
                    for j in b..127{
                        
                    if let Some(n) = get_value_in_bit_mask(&v, i, j){
                        if n != p {
                            if n % p == 0 {
                                    add_bit(&mut v, i, j);
                                }
                            }
                    }
                    }

                    b = 0;
                }
            }

            if let Some((k_aux, w_aux)) = search_next_first_position(&mut v, p){
                k = k_aux;
                w = w_aux;
            }

        }

        for i in k..v.len(){
            for j in w..127{
                if let Some(n) = get_value_in_bit_mask(&v, i, j){
                    res.push(n);
                }
            }
            w = 0;
        }
    
        return Some(res);
    }
}

// MULTI THREAD

fn num_charges_per_thread(n: u128, t: u128) -> (u128, u128){
    let q: u128 = n.isqrt() - 1;

    let i: u128 = (1 - 1) / 254;
    let j: u128 = (q - 254 * i - 1).div_ceil(2);

    (((127 * i)  + j ) / t, ((127 * i)  + j ) % t)
}

fn patches_charges_per_threads(n: u128, t: u128) -> Option<Vec<(u128, u128)>>{
    if t == 0 {return  None;}

    let mut res: Vec<(u128, u128)> = Vec::new();

    let len_patches: (u128, u128) = num_charges_per_thread(n, t);
    let root: u128 = n.isqrt();

    if len_patches.0 == 0{
        return None;
    }else{
        let mut init: u128 = 3; // Primeiro número a ser avaliado

        for _ in 0..t{
            let end = init 
                + if len_patches.0 % 2 == 0 { 
                    len_patches.0
                } else {
                    len_patches.0 + 1
                };

            res.push(
                (
                    init, 
                    if end <= root {end} else {0}
                )
            );

            init = end + 2;
        }

        return Some(res);
    }
}

fn get_value_in_bit_mask_multithread(v: &Arc<RwLock<Vec<u128>>>, i: usize, j: usize) -> Option<u128>{

    let vec = v.read().unwrap();

    if !bit_is_marked(&vec, i, j) {Some(value_of_bit(i, j))} else {None}
}

fn bit_is_marked_multithread(v: &Arc<RwLock<Vec<u128>>>, i: usize, j: usize) -> bool{
    /*
    i posição do u128 no vector
    j posição do bit no u128
    
    Se o bit estiver em 0 não está marcado podendo ser um possível número primo,
    Se estiver em 1 o bit está marcado e não um número primo
    */
    
    if j <= 127 {
        let mask: u128 = 0 | (1 << j);
        
        let vec = v.read().unwrap();

        if vec[i] & mask == 0 {
            return false;
        }else{
            return true;
        }
    }else{
        panic!("O indice j deve ser de 0 até 127")
    }
}

fn add_bit_multithread(v: &Arc<RwLock<Vec<u128>>>, i: usize, j: usize) {
    if j <= 127 {
        let mut vec = v.write().unwrap();
        vec[i] = vec[i] | (1u128 << j);
    }else{
        panic!("O indice j deve ser de 0 até 127")
    }

}

fn search_next_first_position_multithread(v: &Arc<RwLock<Vec<u128>>>, p: u128) -> Option<(usize, usize)> {
    let mut new_p = p;

    let vec = v.read().unwrap();
    
    loop {
        new_p += 2;
        let (k, w) = next_first_position(new_p);
        
        if k > vec.len() - 1{
            return None;
        }
        
        let vec_clone = Arc::clone(&v);
        if let Some(_) = get_value_in_bit_mask_multithread(&vec_clone, k, w){
            return Some((k, w));
        }

    }
}


// fn eratosthenes_multithread(n: u128, t: u128) -> Option<Vec<u128>> {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_natural_numbers_with_n_253_len_vec_is_1(){
        /*
            Verifica se n = 253 o vetor deve ter apenas um elemento
        */

        let n: u128 = 253;
        let len_v: usize = gen_natural_numbers(n).len();

        assert_eq!(len_v, 1);
    }

    #[test]
    fn test_checks_that_no_bits_have_been_marked_with_n_253(){
        /*
        Quando n = 253 não deve haver nenhum bit marcado
         */
        
        let mut cont: i32 = 0;

        let n: u128 = 253;
        let v: Vec<u128> = gen_natural_numbers(n);

        for j in 0..127 {
            if value_of_bit(v.len() - 1, j) > n{
                cont += 1;
            }
        }

        assert_eq!(cont, 0);
    }

    #[test]
    fn test_if_80_bits_have_been_marked_with_n_10_000(){
        /*
        Quando n = 10_000 81 bits do último u128 devem estar marcados 
        após o bit 46, sendo todos maiores do que n
         */

         let mut cont_1: i32 = 0;
         let mut cont_2: i32 = 0;

        let n: u128 = 10_000;
        let v: Vec<u128> = gen_natural_numbers(n);

        for j in 0..127 {
            if value_of_bit(v.len() - 1, j) > n{
                cont_2 += 1;
            }else {
                cont_1 += 1;
            }
        }

        assert_eq!(cont_1, 47);
        assert_eq!(cont_2, 80);
    }

    #[test]
    fn test_gen_natural_numbers_with_n_10_000_len_vec_is_39(){
        /*
            Verifica se n = 10 Bilhões o vetor deve ter 391 elementos
        */

        let n: u128 = 10_000;
        let len_v: usize = gen_natural_numbers(n).len();

        assert_eq!(len_v, 40);
    }

    #[test]
    fn test_bit_is_marked(){
        let v_1: Vec<u128> = vec![1u128];
        let v_2: Vec<u128> = vec![1u128, 1u128];

        assert!(bit_is_marked(&v_1, 0, 0));
        assert!(bit_is_marked(&v_2, 1, 0));
    }

    #[test]
    fn test_add_bit(){
        let mut v_1: Vec<u128> = vec![1u128, 0u128];
        let mut v_2: Vec<u128> = vec![0; 10_000];

        add_bit(&mut v_1, 1, 0);
        add_bit(&mut v_2, 9_999, 127 );

        assert!(bit_is_marked(&v_1, 1, 0));
        assert!(bit_is_marked(&v_2, 9_999, 127));
    }

    #[test]
    fn test_value_of_bit() {
        /*
            Verfica se a função encontra todos os números ímpares, e
            o número dois da sequência.

            Sequência:

            f(i, j) = {
                Se 127*i + j = 0 então 2 // primeiro número primo
                Se 127*i + j > 0 então 2(127*i + j) + 1 // todos os números ímpares começando por 3
            }
         */


        let mut i: usize = 0;
        let mut j: usize = 0;

        assert_eq!(value_of_bit(i, j), 2);

        j = 127;
        assert_eq!(value_of_bit(i, j), 255);

        i = 39;
        j = 46;

        assert_eq!(value_of_bit(i, j), 9_999)


    }

    #[test]
    fn test_get_value_in_bit_mask(){
        /*
        Verifica se a função get_value_in_bit_mask(), retorna o valor
        que deve devolver.

        Caso o bit esteja marcado (1) a função retorna None
        */
        
        let n: u128 = 10_000;
        let mut v: Vec<u128> = gen_natural_numbers(n);

        // Bits não marcados

        assert_eq!(get_value_in_bit_mask(&v, 0, 0), Some(2));

        assert_eq!(get_value_in_bit_mask(&v, 39, 46), Some(9_999));

        assert_eq!(get_value_in_bit_mask(&v, 10, 6), Some(2_553));

        assert_eq!(get_value_in_bit_mask(&v, 15, 78), Some(3_967));

        // Bits marcados 

        add_bit(&mut v, 0, 0);
        add_bit(&mut v, 39, 46);
        add_bit(&mut v, 10, 6);
        add_bit(&mut v, 15, 78);

        assert!(bit_is_marked(&v, 0, 0));
        assert!(bit_is_marked(&v, 39, 46));
        assert!(bit_is_marked(&v, 10, 6));
        assert!(bit_is_marked(&v, 15, 78));
    }
        
    #[test]
    fn test_next_first_position(){
        let a_1: u128 = 9;
        let (i_1, j_1) = next_first_position(a_1);

        let a_2: u128 = 1027;
        let (i_2, j_2) = next_first_position(a_2);

        assert_eq!(value_of_bit(i_1, j_1), a_1);
        assert_eq!(value_of_bit(i_2, j_2), a_2);
    }   

    #[test]
    fn test_eratosthenes(){
       assert_eq!(eratosthenes(0), None);
       assert_eq!(eratosthenes(1), None);
       assert_eq!(eratosthenes(2), Some(vec![2]));
       assert_eq!(eratosthenes(3), Some(vec![2, 3]));
       assert_eq!(eratosthenes(5), Some(vec![2, 3, 5]));
       assert_eq!(eratosthenes(7), Some(vec![2, 3, 5, 7]));
       assert_eq!(eratosthenes(11), Some(vec![2, 3, 5, 7, 11]));
       assert_eq!(eratosthenes(15), Some(vec![2, 3, 5, 7, 11, 13]));
       assert_eq!(eratosthenes(100), Some(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]))
    }

    #[test]
    fn test_eratosthenes_great_numbers(){
        assert_eq!(eratosthenes(347), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347
        ]));


        assert_eq!(eratosthenes(348), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347
        ]));


        assert_eq!(eratosthenes(1223), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223
        ]));


        assert_eq!(eratosthenes(1228), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223
        ]));


        assert_eq!(eratosthenes(2740), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731
        ]));


        assert_eq!(eratosthenes(2740), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731
        ]));


         assert_eq!(eratosthenes(4391), Some(vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731,  2741, 2749, 2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897, 2903, 2909, 2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049, 3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169, 3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259, 3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407, 3413, 3433, 3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539, 3541, 3547, 3557, 3559, 3571, 3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637, 3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727, 3733, 3739, 3761, 3767, 3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847, 3851, 3853, 3863, 3877, 3881, 3889, 3907, 3911, 3917, 3919, 3923, 3929, 3931, 3943, 3947, 3967, 3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027, 4049, 4051, 4057, 4073, 4079, 4091, 4093, 4099, 4111, 4127, 4129, 4133, 4139, 4153, 4157, 4159, 4177, 4201, 4211, 4217, 4219, 4229, 4231, 4241, 4243, 4253, 4259, 4261, 4271, 4273, 4283, 4289, 4297, 4327, 4337, 4339, 4349, 4357, 4363, 4373, 4391
        ]));
    }

    #[test]
    fn test_eratosthenes_very_great_numbers(){
        assert_eq!(eratosthenes(7_919).unwrap().len(), 1_000);
    }

    #[test]
    fn test_num_charges_per_thread(){
        assert_eq!(num_charges_per_thread(255, 3), (2, 1));
        assert_eq!(num_charges_per_thread(7_919, 4), (10, 3));
    }

    #[test]
    fn test_patches_charges_per_threads(){
        let n: u128 = 225;
        let t: u128 = 3;

        assert_eq!(patches_charges_per_threads(n, 0), None);
        assert_eq!(patches_charges_per_threads(n, t), Some(vec![(3, 7), (9, 13), (15, 0)]))
    }

    #[test]
    fn test_get_value_in_bit_mask_multithread(){
        let shared_vector = Arc::new(RwLock::new(gen_natural_numbers(256)));

        assert_eq!(get_value_in_bit_mask_multithread(&shared_vector, 0, 0), Some(2));
    }

    #[test]
    fn test_add_bit_multithread(){
        let shared_vector_1 = Arc::new(RwLock::new(vec![1u128, 0u128]));
        let shared_vector_2 = Arc::new(RwLock::new(vec![0; 10_000]));

        add_bit_multithread(&shared_vector_1, 1, 0);
        add_bit_multithread(&shared_vector_2, 9_999, 127);

        assert!(bit_is_marked_multithread(&shared_vector_1, 1, 0));
        assert!(bit_is_marked_multithread(&shared_vector_2, 9_999, 127));
    }

    #[test]
    fn test_search_next_first_position_multithread(){
        let shared_vector = Arc::new(RwLock::new(gen_natural_numbers(256)));

        assert_eq!(search_next_first_position_multithread(&shared_vector, 3), Some((0, 2)));
    }
}
