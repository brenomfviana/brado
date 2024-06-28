#[cfg(test)]
mod cpf_tests {
    use brado;

    #[test]
    fn cpf_validate_1_valid_cpfs() {
        let valid_cpfs = ["63929247011", "639.292.470-11"];
        for valid_cpf in valid_cpfs {
            assert_eq!(brado::cpf::validate(valid_cpf), true);
        }
    }

    #[test]
    fn cpf_validate_2_invalid_cpfs() {
        let invalid_cpfs = [
            "6392924701",
            "63929247010",
            "639292470110",
            "639.292.470-1",
            "639.292.470-10",
            "639.292.470-110",
            "63.929.247-011",
            "6392.924.701-1",
            "639:292.470-11",
            "639.292:470-11",
            "639.292.470:11",
            "AAAAAAAAAAA",
            "AAA.AAA.AAA-AA",
        ];
        for invalid_cpf in invalid_cpfs {
            assert_eq!(brado::cpf::validate(invalid_cpf), false);
        }
    }

    #[test]
    fn cpf_validate_3_invalid_cpfs_repeated_numbers() {
        for i in 1..=9 {
            let cpf_rep_num = (vec![i.to_string(); 11]).join("");
            assert_eq!(brado::cpf::validate(&cpf_rep_num), false);
        }
    }

    #[test]
    fn cpf_is_bare_1_valid_bare() {
        let documents = [
            "63929247011", // Valid CPF
            "63929247010", // Invalid CPF
        ];
        for document in documents {
            assert_eq!(brado::cpf::is_bare(document), true);
        }
    }

    #[test]
    fn cpf_is_bare_2_invalid_bare() {
        let documents = [
            "639.292.470-11", // Valid CPF
            "639.292.470-10", // Invalid CPF
            "639.29247011",   // Invalid CPF
            "639292.47011",   // Invalid CPF
            "639292470-11",   // Invalid CPF
            "6392924701",     // Invalid CPF
            "639292470110",   // Invalid CPF
        ];
        for document in documents {
            assert_eq!(brado::cpf::is_bare(document), false);
        }
    }

    #[test]
    fn cpf_is_masked_1_valid_masked() {
        let documents = [
            "639.292.470-11", // Valid CPF
            "639.292.470-10", // Invalid CPF
        ];
        for document in documents {
            assert_eq!(brado::cpf::is_masked(document), true);
        }
    }

    #[test]
    fn cpf_is_masked_2_invalid_masked() {
        let documents = [
            "63929247011",     // Valid CPF
            "63929247010",     // Invalid CPF
            "639.29247011",    // Invalid CPF
            "639292.47011",    // Invalid CPF
            "639292470-11",    // Invalid CPF
            "6392924701",      // Invalid CPF
            "639292470110",    // Invalid CPF
            "639.292.470-1",   // Invalid CPF
            "639.292.470-110", // Invalid CPF
            "63.929.247-011",  // Invalid CPF
            "6392.924.701-1",  // Invalid CPF
            "639:292.470-11",  // Invalid CPF
            "639.292:470-11",  // Invalid CPF
            "639.292.470:11",  // Invalid CPF
        ];
        for document in documents {
            assert_eq!(brado::cpf::is_masked(document), false);
        }
    }

    #[test]
    fn cpf_mask_1_valid_mask() {
        let valid_cpf: &str = "63929247011";
        assert_eq!(
            brado::cpf::mask(valid_cpf),
            Ok(String::from("639.292.470-11"))
        );
        let invalid_cpf: &str = "63929247010";
        assert_eq!(
            brado::cpf::mask(invalid_cpf),
            Ok(String::from("639.292.470-10"))
        );
    }

    #[test]
    fn cpf_mask_2_invalid_mask() {
        let documents = [
            "639.292.470-11", // Valid CPF
            "639.292.470-10", // Invalid CPF
            "6392924701",     // Invalid CPF
            "639292470110",   // Invalid CPF
        ];
        for document in documents {
            let result = brado::cpf::mask(document);
            assert_eq!(
                result,
                Err("The given string cannot be masked as CPF!"),
            );
        }
    }

    #[test]
    fn cpf_generate_1() {
        for _ in 0..1000 {
            let cpf = brado::cpf::generate();
            assert_eq!(brado::cpf::validate(&cpf), true);
            assert_eq!(brado::cpf::is_bare(&cpf), true);
        }
    }

    #[test]
    fn cpf_generate_masked_1() {
        for _ in 0..1000 {
            let cpf = brado::cpf::generate_masked();
            assert_eq!(brado::cpf::validate(&cpf), true);
            assert_eq!(brado::cpf::is_masked(&cpf), true);
        }
    }
}
