//! Estrutura que representa o documento CNS.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, random_decimal_vector, random_element_from_vector,
    to_decimal,
};

const CNS_SIZE: usize = 15;

/// Marcador que representa o documento CNS, implementando o trait
/// [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_cns::Cns;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Cns>(), ("CNS", 15));
/// assert!(Cns::validate("144082627260004"));
/// ```
pub struct Cns;

impl Document for Cns {
    const NAME: &'static str = "CNS";
    const SIZE: usize = CNS_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != CNS_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != CNS_SIZE || is_first_digit_invalid(&digits[0]) {
            return false;
        }

        validate_checksum(&digits)
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == CNS_SIZE
            && get_digits(doc, to_decimal).len() == CNS_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 3 || digits.len() != CNS_SIZE {
            return false;
        }

        symbols[0] == (3, ' ')
            && symbols[1] == (8, ' ')
            && symbols[2] == (13, ' ')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as CNS!");
        }

        let masked_doc: String = format!(
            "{} {} {} {}",
            &doc[0..3],
            &doc[3..7],
            &doc[7..11],
            &doc[11..15],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let first_digit: u16 =
            random_element_from_vector(&valid_first_digits());

        let cns: Vec<u16> = {
            if [1, 2].contains(&first_digit) {
                generate_first_case(first_digit)
            } else {
                generate_second_case(first_digit)
            }
        };

        cns.iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid CNS!")
    }
}

fn valid_first_digits() -> Vec<u16> {
    vec![1, 2, 7, 8, 9]
}

fn is_first_digit_valid(digit: &u16) -> bool {
    valid_first_digits().contains(digit)
}

fn is_first_digit_invalid(digit: &u16) -> bool {
    !is_first_digit_valid(digit)
}

fn validate_checksum(doc_slice: &[u16]) -> bool {
    if [1, 2].contains(&doc_slice[0]) {
        let check_digits: Vec<u16> =
            generate_last_four_digits(&doc_slice[..11]);

        doc_slice[11..] == check_digits
    } else {
        let checksum: u16 = cns_sum(doc_slice);

        checksum.is_multiple_of(11)
    }
}

fn cns_sum(doc_slice: &[u16]) -> u16 {
    doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * (15 - i as u16))
        .sum()
}

fn generate_last_four_digits(doc_slice: &[u16]) -> Vec<u16> {
    let mut checksum: u16 = cns_sum(doc_slice);

    let mut check_digit: u16 = 11 - (checksum % 11);

    if check_digit == 11 {
        check_digit = 0;
    }

    let check_digits: Vec<u16> = {
        if check_digit == 10 {
            checksum += 2;
            check_digit = 11 - (checksum % 11);
            vec![0, 0, 1, check_digit]
        } else {
            vec![0, 0, 0, check_digit]
        }
    };

    check_digits
}

fn generate_first_case(first_digit: u16) -> Vec<u16> {
    let mut cns: Vec<u16> = vec![first_digit];
    cns.extend_from_slice(&random_decimal_vector(10));
    cns.extend_from_slice(&generate_last_four_digits(&cns));

    cns
}

fn generate_second_case(first_digit: u16) -> Vec<u16> {
    let mut cns: Vec<u16> = vec![first_digit];
    cns.extend_from_slice(&random_decimal_vector(14));

    let checksum: u16 = cns_sum(&cns);
    let rest: u16 = checksum % 11;
    if rest == 0 {
        return cns;
    }

    let diff: u16 = 11 - rest;

    let mut val: usize = diff as usize;
    let mut idx: usize = 15 - val;

    loop {
        if val == 0 {
            if validate_checksum(&cns) {
                return cns;
            } else {
                let checksum: u16 = cns_sum(&cns);

                let diff: u16 = 15 - (checksum % 11);

                val = diff as usize;
                idx = 15 - diff as usize;
                continue;
            }
        }

        if 15 - idx > val {
            idx += 1;
            continue;
        }

        if cns[idx] != 9 {
            cns[idx] += 1;
            val -= 15 - idx;
        } else {
            cns[idx] -= 1;
            val += 15 - idx;
            idx -= 1;
        }
    }
}
