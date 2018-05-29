fn main() {
    println!("Hello, world!");
}


pub mod lisp_parser; // synthesized by LALRPOP

#[test]
fn lisp_parser() {
    assert!(lisp_parser::TermParser::new().parse("22").is_ok());
    assert!(lisp_parser::TermParser::new().parse("(22)").is_ok());
    assert!(lisp_parser::TermParser::new().parse("((((22))))").is_ok());
    assert!(lisp_parser::TermParser::new().parse("((22)").is_err());
}
