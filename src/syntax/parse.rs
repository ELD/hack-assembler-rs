use std::collections::HashMap;
use regex::Regex;

pub struct Parser<'a> {
    pc: u32,
    l_command_regex: Regex,
    a_command_regex: Regex,
    comp_bits: HashMap<&'a str, (&'a str, &'a str)>,
    dest_bits: HashMap<&'a str, &'a str>,
    jump_bits: HashMap<&'a str, &'a str>,
    //symbol_table: HashMap<String, u32>,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LCommand,
    ACommand,
    CCommand,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let mut comp_bits = HashMap::new();
        comp_bits.insert("0", ("0", "101010"));
        comp_bits.insert("1", ("0", "111111"));
        comp_bits.insert("-1", ("0", "111010"));
        comp_bits.insert("D", ("0", "001100"));
        comp_bits.insert("A", ("0", "110000"));
        comp_bits.insert("!D", ("0", "001101"));
        comp_bits.insert("!A", ("0", "110001"));
        comp_bits.insert("-D", ("0", "001111"));
        comp_bits.insert("-A", ("0", "110011"));
        comp_bits.insert("D+1", ("0", "011111"));
        comp_bits.insert("A+1", ("0", "110111"));
        comp_bits.insert("D-1", ("0", "001110"));
        comp_bits.insert("A-1", ("0", "110010"));
        comp_bits.insert("D+A", ("0", "000010"));
        comp_bits.insert("D-A", ("0", "010011"));
        comp_bits.insert("A-D", ("0", "000111"));
        comp_bits.insert("D&A", ("0", "000000"));
        comp_bits.insert("D|A", ("0", "010101"));
        comp_bits.insert("M", ("1", "110000"));
        comp_bits.insert("!M", ("1", "110001"));
        comp_bits.insert("-M", ("1", "110011"));
        comp_bits.insert("M+1", ("1", "110111"));
        comp_bits.insert("M-1", ("1", "110010"));
        comp_bits.insert("D+M", ("1", "000010"));
        comp_bits.insert("D-M", ("1", "010011"));
        comp_bits.insert("M-D", ("1", "000111"));
        comp_bits.insert("D&M", ("1", "000000"));
        comp_bits.insert("D|M", ("1", "010101"));

        let mut dest_bits = HashMap::new();
        dest_bits.insert("null", "000");
        dest_bits.insert("M", "001");
        dest_bits.insert("D", "010");
        dest_bits.insert("MD", "011");
        dest_bits.insert("A", "100");
        dest_bits.insert("AM", "101");
        dest_bits.insert("AD", "110");
        dest_bits.insert("AMD", "111");

        let mut jump_bits = HashMap::new();
        jump_bits.insert("null", "000");
        jump_bits.insert("JGT", "001");
        jump_bits.insert("JEQ", "010");
        jump_bits.insert("JGE", "011");
        jump_bits.insert("JLT", "100");
        jump_bits.insert("JNE", "101");
        jump_bits.insert("JLE", "110");
        jump_bits.insert("JMP", "111");

        Parser {
            pc: 0,
            l_command_regex: Regex::new(r"\((.*)\)").unwrap(),
            a_command_regex: Regex::new(r"@([\w|\d].*)").unwrap(),
            comp_bits: comp_bits,
            dest_bits: dest_bits,
            jump_bits: jump_bits,
        }
    }

    pub fn parse<'b>(&'b self, token: &'b str) -> String {
        let mut opcode = String::new();
        match self.token_type(token) {
            TokenType::LCommand => {
                unimplemented!();
            },
            TokenType::ACommand => {
                opcode.push_str("0");
                let capture = self.a_command_regex.captures(token).unwrap();
                let digit = capture.at(1).unwrap();

                let bits = format!("{:0>15b}", digit.parse::<i32>().unwrap());
                opcode = opcode + &bits;
            },
            TokenType::CCommand => {
                opcode.push_str("111");
                let comp_bits = self.get_comp_bits(token);
                let dest_bits = self.get_dest_bits(token);
                let jump_bits = self.get_jump_bits(token);
                unimplemented!();
            },
        }

        opcode
    }

    pub fn token_type<'b>(&'b self, token: &'b str) -> TokenType {
        if self.l_command_regex.is_match(token) {
            return TokenType::LCommand;
        }

        if self.a_command_regex.is_match(token) {
            return TokenType::ACommand;
        }

        return TokenType::CCommand;
    }

    pub fn get_comp_bits<'b>(&'b self, token: &'b str) -> String {
        unimplemented!();
    }

    pub fn get_dest_bits<'b>(&'b self, token: &'b str) -> String {
        unimplemented!();
    }

    pub fn get_jump_bits<'b>(&'b self, token: &'b str) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup<'a>() -> Parser<'a> {
        Parser::new()
    }

    #[test]
    fn create_parser() {
        let parser = setup();
    }

    #[test]
    fn recognizes_token_types() {
        let parser = setup();

        assert_eq!(parser.token_type("(LOOP)"), TokenType::LCommand);
        assert_eq!(parser.token_type("@i"), TokenType::ACommand);
        assert_eq!(parser.token_type("@R2"), TokenType::ACommand);
        assert_eq!(parser.token_type("M=M+D"), TokenType::CCommand);
    }

    #[test]
    fn parse_output_for_a_command() {
        let parser = setup();

        assert_eq!(parser.parse("@5"), "0000000000000101");
    }

    #[test]
    fn parse_output_for_c_command() {
        let parser = setup();

        assert_eq!(parser.parse("M=D+A"), "1110000010001000");
        assert_eq!(parser.parse("AMD=D|A"), "1110010101111000");
        assert_eq!(parser.parse("0;JMP"), "1110101010000111");
        assert_eq!(parser.parse("A;JGE"), "1110110000000011");
    }

    #[test]
    fn test_get_comp_bits() {
        let parser = setup();

        assert_eq!(parser.parse("0;JMP"), "0101010");
        assert_eq!(parser.parse("M=!A"), "0110001");
        assert_eq!(parser.parse("D=!M"), "1110001");
        assert_eq!(parser.parse("D&M;JLT"), "1000000");
    }
}

