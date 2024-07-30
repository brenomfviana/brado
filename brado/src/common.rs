//! Funções comuns utilizadas na validação de documentos.

use rand::Rng;
use std::collections::HashSet;

const RADIX: u32 = 10;

/// Verifica se o vetor de dígitos possui um único numeral.
/// Se possuir, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplo
///
/// ```
/// use brado::common::is_repeated;
///
/// let result = is_repeated(&vec![1, 1, 1]); // true
/// assert!(result);
///
/// let result = is_repeated(&vec![1, 2, 1]); // false
/// assert!(!result);
/// ```
pub fn is_repeated(digits: &[u16]) -> bool {
    let a_set: HashSet<u16> = HashSet::from_iter(digits.iter().cloned());
    a_set.len() == 1
}

/// Recebe o índice da posição (`usize`) e um caractere (`char`)
/// e retorna a conversão do caractere em um dígito decimal
/// (Option<`u16`>).
///
/// ## Exemplo
///
/// ```
/// use brado::common::to_decimal;
///
/// let result = to_decimal(0, '1');
/// assert_eq!(result, Some(1));
/// ```
pub fn to_decimal(
    _i: usize,
    c: char,
) -> Option<u16> {
    c.to_digit(RADIX).map(|c| c as u16)
}

/// Extrai e retorna o vetor dígitos de uma string (`&str`)
/// a partir da função de conversão passada.
///
/// ## Exemplo
///
/// ```
/// use brado::common::{get_digits, to_decimal};
///
/// let result = get_digits("111", &to_decimal);
/// assert_eq!(result, vec![1, 1, 1]);
///
/// let result = get_digits("121", &to_decimal);
/// assert_eq!(result, vec![1, 2, 1]);
/// ```
pub fn get_digits<F>(
    doc: &str,
    convert: &F,
) -> Vec<u16>
where
    F: Fn(usize, char) -> Option<u16>,
{
    doc.chars()
        .enumerate()
        .filter_map(|(i, c)| convert(i, c))
        .collect()
}

/// Extrai e retorna o vetor de símbolos de uma string (`&str`)
/// a partir da função de conversão passada.
/// O vetor resultante é um vetor de tuplas de dois elementos: o
/// primeiro representa a posição do símbolo na string e o segundo
/// o próprio símbolo.
///
/// ## Exemplo
///
/// ```
/// use brado::common::{get_symbols, to_decimal};
///
/// let result = get_symbols("1.1-1", &to_decimal);
/// assert_eq!(result, vec![(1, '.'), (3, '-')]);
/// ```
pub fn get_symbols<F>(
    doc: &str,
    convert: F,
) -> Vec<(usize, char)>
where
    F: Fn(usize, char) -> Option<u16>,
{
    doc.chars()
        .enumerate()
        .filter_map(|(i, c)| match convert(i, c) {
            Some(_) => None,
            None => Some((i, c)),
        })
        .collect()
}

/// Desmascara uma string (`&str`), ou seja, remove os símbolos,
/// a partir da função de conversão passada e retorna a string
/// resultante.
///
/// ## Exemplo
///
/// ```
/// use brado::common::{unmask, to_decimal};
///
/// let result = unmask("1.1-1", &to_decimal);
/// assert_eq!(result, String::from("111"));
/// ```
pub fn unmask<F>(
    doc: &str,
    convert: F,
) -> String
where
    F: Fn(usize, char) -> Option<u16>,
{
    doc.chars()
        .enumerate()
        .filter_map(|(i, c)| convert(i, c).map(|n| n.to_string()))
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um vetor de dígitos aleatórios com o tamanho `size`.
///
/// ## Exemplo
///
/// ```
/// use brado::common::random_digit_vector;
///
/// let result = random_digit_vector(10);
/// assert_eq!(result.len(), 10);
/// ```
pub fn random_digit_vector(size: usize) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut digits: Vec<u16> = vec![];
    for _ in 0..size {
        digits.push(rng.gen_range(0..10));
    }
    digits
}

/// Seleciona aleatoriamente um elemento de um vetor de digítos.
///
/// ## Exemplo
///
/// ```
/// use brado::common::random_digit_from_vector;
///
/// let options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let result = random_digit_from_vector(&options);
/// assert_eq!(options.contains(&result), true);
/// ```
pub fn random_digit_from_vector(options: &[u16]) -> u16 {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..options.len());
    options[idx]
}
