#[cfg(test)]
mod cpf_tests {
    use brado;

    #[test]
    fn validate_cpf_1() {
        let bare_cpf = String::from("11111111111");
        assert_eq!(false, brado::cpf::validate(&bare_cpf, false));
    }

    #[test]
    fn validate_cpf_2() {
        let bare_cpf = String::from("111.111.111-11");
        assert_eq!(false, brado::cpf::validate(&bare_cpf, true));
    }

    #[test]
    fn validate_cpf_3() {
        let bare_cpf = String::from("63929247011");
        assert_eq!(true, brado::cpf::validate(&bare_cpf, false));
    }

    #[test]
    fn validate_cpf_4() {
        let bare_cpf = String::from("639.292.470-11");
        assert_eq!(true, brado::cpf::validate(&bare_cpf, true));
    }

    #[test]
    fn validate_str_cpf_1() {
        let bare_cpf = "11111111111";
        assert_eq!(false, brado::cpf::validate_str(&bare_cpf, false));
    }

    #[test]
    fn validate_str_cpf_2() {
        let bare_cpf = "111.111.111-11";
        assert_eq!(false, brado::cpf::validate_str(&bare_cpf, true));
    }

    #[test]
    fn validate_str_cpf_3() {
        let bare_cpf = "63929247011";
        assert_eq!(true, brado::cpf::validate_str(&bare_cpf, false));
    }

    #[test]
    fn validate_str_cpf_4() {
        let bare_cpf = "639.292.470-11";
        assert_eq!(true, brado::cpf::validate_str(&bare_cpf, true));
    }
}
