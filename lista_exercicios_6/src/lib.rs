/*
Exercício 6

Implementar um programa multithread que calcule os número primos entre 2 e n (u128::MAX), 
tendo como limite para n o maior valor armazenado em um dado u128.
*/

fn gen_natural_numbers(n: u128) -> Vec<u128>{
    let len_v: u128 = ( (n.isqrt() as u128 - 2u128) / 2u128) / 128u128;

    vec![0; len_v as usize + 1]
}

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

fn get_value_in_bit_mask(v: &Vec<u128>, i: usize, j: usize) -> Option<u128> {
    /*
        i posição do u128 no vector
        j posição do bit no u128

        Retorna o número u128 presente no u128 na posição j
        estando na posição i do vector
     */

    let first_value_in_u128: u128 = 126u128
}


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
    fn test_gen_natural_numbers_with_n_10_b_len_vec_is_391(){
        /*
            Verifica se n = 10 Bilhões o vetor deve ter 391 elementos
        */

        let n: u128 = 10_000_000_000;
        let len_v: usize = gen_natural_numbers(n).len();

        assert_eq!(len_v, 391);
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
        
}
