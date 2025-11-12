use 
mod tests{
    #[cfg(test)]
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_args(){
        let args = CliArgs::parse_from(["prog", "127.0.0.1"]);
        assert_eq!(args.target, "127.0.0.1");
    }
    
    #[test]
    fn test_parse_single_port(){
        let range = parse_port_range("22").unwrap();
        assert_eq!(range.start, 22);
        assert_eq!(range.end, 22);
    }

    #[test]
    fn test_parse_port_range(){
        let range = parse_port_range("20-25").unwrap();
        assert_eq!(range.start, 20);
        assert_eq!(range.end, 25);
    }

    #[test]
    fn test_invalid_range(){
        assert!(parse_port_range("70000-80000").is_err());
    }

}
