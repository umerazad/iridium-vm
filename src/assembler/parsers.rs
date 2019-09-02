use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace1};
use nom::combinator::{cut, map};
use nom::multi::many1;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use nom::error::context;

use crate::assembler::{AssemblyInstruction, Program, Token};
use crate::instruction::Opcode;

/// Parses opcode part of the instruction.
pub fn parse_opcode<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let (next_input, result) = alpha1(input)?;
    Ok((next_input, Token::Opcode(Opcode::from(result))))
}

/// Parses the register part. i.e. $0. We don't enforce the register
/// count limit here. It'll be taken care of at the assembler level.
pub fn parse_register<'a>(input: &'a str) -> IResult<&'a str, Token> {
    map(
        context("register", preceded(tag("$"), cut(digit1))),
        |num: &str| Token::Register(num.parse::<u8>().unwrap()),
    )(input)
}

/// Parses the number operand #123.
pub fn parse_number<'a>(input: &'a str) -> IResult<&'a str, Token> {
    map(
        context("integer", preceded(tag("#"), cut(digit1))),
        |num: &str| Token::IntegerOperand(num.parse::<i32>().unwrap()),
    )(input)
}

/// Parses instruction of the form
///     opcode $reg #num i.e. LOAD $1 #200
pub fn parse_instruction1<'a>(input: &'a str) -> IResult<&'a str, AssemblyInstruction> {
    let parser = tuple((
        parse_opcode,
        preceded(multispace1, parse_register),
        preceded(multispace1, parse_number),
    ));

    match parser(input.trim()) {
        Ok((next_input, (opcode, reg, num))) => {
            Ok((
                next_input,
                AssemblyInstruction {
                    opcode: opcode,
                    operand1: Some(reg),
                    operand2: Some(num),
                    operand3: None, // Not used in this instruction format.
                },
            ))
        }
        Err(err) => Err(err),
    }
}

/// Parses a complete program.
pub fn parse_program<'a>(input: &'a str) -> IResult<&'a str, Program> {
    match many1(parse_instruction1)(input.trim()) {
        Ok((next_input, instructions)) => Ok((next_input, Program { instructions })),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Failure;

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode("load"), Ok(("", Token::Opcode(Opcode::LOAD))));
        assert_eq!(
            parse_opcode("hlt bla bla"),
            Ok((" bla bla", Token::Opcode(Opcode::HLT)))
        );
    }

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register("$0"), Ok(("", Token::Register(0))));
        assert_eq!(
            parse_register("$31 #999"),
            Ok((" #999", Token::Register(31)))
        );
        assert_eq!(
            parse_register("$a $b"),
            Err(Failure(("a $b", ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("#500"), Ok(("", Token::IntegerOperand(500))));
        assert_eq!(
            parse_number("#1000 ;1k"),
            Ok((" ;1k", Token::IntegerOperand(1000)))
        );
    }

    #[test]
    fn test_parse_instruction1() {
        let result = parse_instruction1("  load   $9   #299  \t\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblyInstruction {
                    opcode: Token::Opcode(Opcode::LOAD),
                    operand1: Some(Token::Register(9)),
                    operand2: Some(Token::IntegerOperand(299)),
                    operand3: None
                }
            ))
        )
    }

    #[test]
    fn test_parse_program() {
        let result = parse_program(" load $0 #100\n load $1 #200 \n");
        assert_eq!(result.is_ok(), true);

        let (remaining_input, program) = result.unwrap();

        // Ensure that the complete program is consumed.
        assert_eq!("", remaining_input);

        assert_eq!(
            program.instructions[0],
            AssemblyInstruction {
                opcode: Token::Opcode(Opcode::LOAD),
                operand1: Some(Token::Register(0)),
                operand2: Some(Token::IntegerOperand(100)),
                operand3: None
            }
        );

        assert_eq!(
            program.instructions[1],
            AssemblyInstruction {
                opcode: Token::Opcode(Opcode::LOAD),
                operand1: Some(Token::Register(1)),
                operand2: Some(Token::IntegerOperand(200)),
                operand3: None
            }
        );
    }
}
