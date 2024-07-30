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
    let digits_clone = digits.iter().cloned();
    let unique_digits: HashSet<u16> = HashSet::from_iter(digits_clone);
    unique_digits.len() == 1
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
/// let result = to_decimal('1');
/// assert_eq!(result, Some(1));
/// ```
pub fn to_decimal(c: char) -> Option<u16> {
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
/// let result = get_digits("111", to_decimal);
/// assert_eq!(result, vec![1, 1, 1]);
///
/// let result = get_digits("121", to_decimal);
/// assert_eq!(result, vec![1, 2, 1]);
/// ```
pub fn get_digits<F>(
    doc: &str,
    convert: F,
) -> Vec<u16>
where
    F: Fn(char) -> Option<u16>,
{
    doc.chars().filter_map(convert).collect()
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
/// let result = get_symbols("1.1-1", to_decimal);
/// assert_eq!(result, vec![(1, '.'), (3, '-')]);
/// ```
pub fn get_symbols<F>(
    doc: &str,
    convert: F,
) -> Vec<(usize, char)>
where
    F: Fn(char) -> Option<u16>,
{
    doc.chars()
        .enumerate()
        .filter_map(|(i, c)| match convert(c) {
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
/// let result = unmask("1.1-1", to_decimal);
/// assert_eq!(result, String::from("111"));
/// ```
pub fn unmask<F>(
    doc: &str,
    convert: F,
) -> String
where
    F: Fn(char) -> Option<u16>,
{
    doc.chars()
        .filter_map(|c| convert(c).map(|n| n.to_string()))
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um vetor de números decimais aleatórios
/// com o tamanho `size`.
///
/// ## Exemplo
///
/// ```
/// use brado::common::random_decimal_vector;
///
/// let result = random_decimal_vector(10);
/// assert_eq!(result.len(), 10);
/// ```
pub fn random_decimal_vector(size: usize) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut digits: Vec<u16> = vec![];
    for _ in 0..size {
        digits.push(rng.gen_range(0..10));
    }
    digits
}

/// Seleciona aleatoriamente um elemento de um vetor.
///
/// ## Exemplo
///
/// ```
/// use brado::common::random_element_from_vector;
///
/// let options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let result = random_element_from_vector(&options);
/// assert_eq!(options.contains(&result), true);
/// ```
pub fn random_element_from_vector<T>(options: &[T]) -> T
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..options.len());
    options[idx].clone()
}

/// Gera e retorna uma string com o tamanho `size` composto por
/// elementos aleatórios do alfabeto `alphabet`.
///
/// ## Exemplo
///
/// ```
/// use brado::common::random_string_from_alphabet;
///
/// let result = random_string_from_alphabet(10, &[1,2,3]);
/// assert_eq!(result.len(), 10);
/// ```
pub fn random_string_from_alphabet<T>(
    size: usize,
    alphabet: &[T],
) -> String
where
    T: Clone + ToString,
{
    let mut vector: Vec<T> = vec![];
    for _ in 0..size {
        let element = random_element_from_vector(alphabet);
        vector.push(element);
    }
    vector
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}
