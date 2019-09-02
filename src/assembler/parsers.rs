use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace1};
use nom::combinator::{cut, map};
use nom::multi::many1;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use nom::error::context;

use crate::assembler::{AssemblyInstruction, Program, Token};
use crate::instruction::Opcode;

type ParseResult<'a, T> = IResult<&'a str, T>;

/// Parses opcode part of the instruction.
pub fn parse_opcode(input: &str) -> ParseResult<Token> {
    let (next_input, result) = alpha1(input)?;
    Ok((next_input, Token::Opcode(Opcode::from(result))))
}

/// Parses the register part. i.e. $0. We don't enforce the register
/// count limit here. It'll be taken care of at the assembler level.
pub fn parse_register(input: &str) -> ParseResult<Token> {
    map(
        context("register", preceded(tag("$"), cut(digit1))),
        |num: &str| Token::Register(num.parse::<u8>().unwrap()),
    )(input)
}

/// Parses the number operand #123.
pub fn parse_number(input: &str) -> ParseResult<Token> {
    map(
        context("integer", preceded(tag("#"), cut(digit1))),
        |num: &str| Token::IntegerOperand(num.parse::<i32>().unwrap()),
    )(input)
}

/// Parses opcode only instructions.
fn parse_instruction0(input: &str) -> ParseResult<AssemblyInstruction> {
    match parse_opcode(input.trim()) {
        Ok((next_input, opcode)) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: opcode,
                operand1: None,
                operand2: None,
                operand3: None,
            },
        )),
        Err(e) => Err(e),
    }
}

/// Parses instruction of the form
///     opcode $reg #num i.e. LOAD $1 #200
fn parse_instruction1(input: &str) -> ParseResult<AssemblyInstruction> {
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
pub fn parse_program(input: &str) -> ParseResult<Program> {
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
        assert_eq!(parse_opcode("HLT"), Ok(("", Token::Opcode(Opcode::HLT))));
        assert_eq!(parse_opcode("load"), Ok(("", Token::Opcode(Opcode::LOAD))));
        assert_eq!(parse_opcode("AdD"), Ok(("", Token::Opcode(Opcode::ADD))));
        assert_eq!(parse_opcode("mUL"), Ok(("", Token::Opcode(Opcode::MUL))));
        assert_eq!(parse_opcode("SuB"), Ok(("", Token::Opcode(Opcode::SUB))));
        assert_eq!(parse_opcode("DIv"), Ok(("", Token::Opcode(Opcode::DIV))));
        assert_eq!(parse_opcode("jMP"), Ok(("", Token::Opcode(Opcode::JMP))));
        assert_eq!(parse_opcode("jmpf"), Ok(("", Token::Opcode(Opcode::JMPF))));
        assert_eq!(parse_opcode("jmpB"), Ok(("", Token::Opcode(Opcode::JMPB))));
        assert_eq!(parse_opcode("Eq"), Ok(("", Token::Opcode(Opcode::EQ))));
        assert_eq!(parse_opcode("neQ"), Ok(("", Token::Opcode(Opcode::NEQ))));
        assert_eq!(parse_opcode("GT"), Ok(("", Token::Opcode(Opcode::GT))));
        assert_eq!(parse_opcode("GTE"), Ok(("", Token::Opcode(Opcode::GTE))));
        assert_eq!(parse_opcode("LT"), Ok(("", Token::Opcode(Opcode::LT))));
        assert_eq!(parse_opcode("LTE"), Ok(("", Token::Opcode(Opcode::LTE))));
        assert_eq!(parse_opcode("JEQ"), Ok(("", Token::Opcode(Opcode::JEQ))));
        assert_eq!(parse_opcode("JNEQ"), Ok(("", Token::Opcode(Opcode::JNEQ))));
        assert_eq!(parse_opcode("IGL"), Ok(("", Token::Opcode(Opcode::IGL))));

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
    fn test_parse_instruction0() {
        let result = parse_instruction0("  hlt\t\n  ");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblyInstruction {
                    opcode: Token::Opcode(Opcode::HLT),
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
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
