#[cfg(test)]
mod cnpj_tests {
    use brado;

    #[test]
    fn cnpj_validate_1_valid_cnpjs() {
        let valid_cnpjs = ["05200851000100", "05.200.851/0001-00"];
        for valid_cnpj in valid_cnpjs {
            assert_eq!(brado::cnpj::validate(valid_cnpj), true);
        }
    }

    #[test]
    fn cnpj_validate_2_invalid_cnpjs() {
        let invalid_cnpjs = [
            "0520085100010",
            "05200851000101",
            "052008510001001",
            "05.200.851/0001-0",
            "05.200.851/0001-01",
            "05.200.851/0001-001",
            "0.520.085/1000-100",
            "052.008.510/0010-0",
            "05:200.851/0001-00",
            "05.200:851/0001-00",
            "05.200.851:0001-00",
            "05.200.851/0001:00",
            "AAAAAAAAAAAAA",
            "AA.AAA.AAA/AAAA-AA",
        ];
        for invalid_cnpj in invalid_cnpjs {
            assert_eq!(brado::cnpj::validate(invalid_cnpj), false);
        }
    }

    #[test]
    fn cnpj_is_bare_1_valid_bare() {
        let documents = [
            "05200851000100", // Valid CNPJ
            "05200851000101", // Invalid CNPJ
        ];
        for document in documents {
            assert_eq!(brado::cnpj::is_bare(document), true);
        }
    }

    #[test]
    fn cnpj_is_bare_2_invalid_bare() {
        let documents = [
            "05.200.851/0001-00", // Valid CNPJ
            "05.200.851/0001-01", // Invalid CNPJ
            "05.200851000101",    // Invalid CNPJ
            "05200.851000101",    // Invalid CNPJ
            "05200851/000101",    // Invalid CNPJ
            "052008510001-01",    // Invalid CNPJ
            "0520085100010",      // Invalid CNPJ
            "052008510001001",    // Invalid CNPJ
        ];
        for document in documents {
            assert_eq!(brado::cnpj::is_bare(document), false);
        }
    }

    #[test]
    fn cnpj_is_masked_1_valid_masked() {
        let documents = [
            "05.200.851/0001-00", // Valid CNPJ
            "05.200.851/0001-01", // Invalid CNPJ
        ];
        for document in documents {
            assert_eq!(brado::cnpj::is_masked(document), true);
        }
    }

    #[test]
    fn cnpj_is_masked_2_invalid_masked() {
        let documents = [
            "05200851000100",      // Valid CNPJ
            "05200851000101",      // Invalid CNPJ
            "05.200851000101",     // Invalid CNPJ
            "05200.851000101",     // Invalid CNPJ
            "05200851/000101",     // Invalid CNPJ
            "052008510001-01",     // Invalid CNPJ
            "05.200.851/0001-0",   // Invalid CNPJ
            "05.200.851/0001-001", // Invalid CNPJ
            "0.520.085/1000-100",  // Invalid CNPJ
            "052.008.510/0010-0",  // Invalid CNPJ
            "05:200.851/0001-00",  // Invalid CNPJ
            "05.200:851/0001-00",  // Invalid CNPJ
            "05.200.851:0001-00",  // Invalid CNPJ
            "05.200.851/0001:00",  // Invalid CNPJ
        ];
        for document in documents {
            assert_eq!(brado::cnpj::is_masked(document), false);
        }
    }

    #[test]
    fn cnpj_mask_1_valid_mask() {
        let valid_cnpj: &str = "05200851000100";
        assert_eq!(
            brado::cnpj::mask(valid_cnpj),
            Ok(String::from("05.200.851/0001-00"))
        );
        let invalid_cnpj: &str = "05200851000101";
        assert_eq!(
            brado::cnpj::mask(invalid_cnpj),
            Ok(String::from("05.200.851/0001-01"))
        );
    }

    #[test]
    fn cnpj_mask_2_invalid_mask() {
        let documents = [
            "05.200.851/0001-00", // Valid CNPJ
            "05.200.851/0001-01", // Invalid CNPJ
            "0520085100010",      // Invalid CNPJ
            "052008510001001",    // Invalid CNPJ
        ];
        for document in documents {
            let result = brado::cnpj::mask(document);
            assert_eq!(
                result,
                Err("The given string cannot be masked as CNPJ!"),
            );
        }
    }

    #[test]
    fn cnpj_generate_1() {
        for _ in 0..1000 {
            let cnpj = brado::cnpj::generate();
            assert_eq!(brado::cnpj::validate(&cnpj), true);
            assert_eq!(brado::cnpj::is_bare(&cnpj), true);
        }
    }

    #[test]
    fn cnpj_generate_masked_1() {
        for _ in 0..1000 {
            let cnpj = brado::cnpj::generate_masked();
            assert_eq!(brado::cnpj::validate(&cnpj), true);
            assert_eq!(brado::cnpj::is_masked(&cnpj), true);
        }
    }
}
