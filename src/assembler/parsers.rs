use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace1};
use nom::combinator::{cut, map, opt};
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use nom::error::context;

use crate::assembler::{AssemblyInstruction, Program, Token};
use crate::instruction::Opcode;

type ParseResult<'a, T> = IResult<&'a str, T>;

/// Parses opcode part of the instruction.
pub fn parse_opcode(input: &str) -> ParseResult<Token> {
    let (next_input, result) = alpha1(input.trim())?;
    Ok((next_input, Token::Opcode(Opcode::from(result))))
}

/// Parses the register part. i.e. $0. We don't enforce the register
/// count limit here. It'll be taken care of at the assembler level.
pub fn parse_register(input: &str) -> ParseResult<Token> {
    map(
        context("register", preceded(tag("$"), cut(digit1))),
        |num: &str| Token::Register(num.parse::<u8>().unwrap()),
    )(input.trim())
}

/// Parses the number operand #123.
pub fn parse_number(input: &str) -> ParseResult<Token> {
    map(
        context("integer", preceded(tag("#"), cut(digit1))),
        |num: &str| Token::IntegerOperand(num.parse::<i32>().unwrap()),
    )(input)
}

/// Parses an operand.
pub fn parse_operand(input: &str) -> ParseResult<Token> {
    alt((parse_number, parse_register))(input.trim())
}

/// Parses a label declaration. Labels are of the form
/// label_1: ....
fn parse_label_declaration(input: &str) -> ParseResult<Token> {
    map(
        context("label declaration", terminated(alphanumeric1, tag(":"))),
        |label: &str| Token::LabelDeclaration(label.to_string()),
    )(input.trim())
}

/// Parses label usage i.e. @label
fn parse_label_usage(input: &str) -> ParseResult<Token> {
    map(
        context("label usage", preceded(tag("@"), alphanumeric1)),
        |label: &str| Token::LabelUsage(label.to_string()),
    )(input.trim())
}

/// Parses directive declaration i.e. .code or .data or .asciiz
fn parse_directive_declaration(input: &str) -> ParseResult<Token> {
    map(
        context("directive", preceded(tag("."), alphanumeric1)),
        |s: &str| Token::Directive(s.to_string()),
    )(input.trim())
}

/// Parses a labeled directive.
///  howdy: .asciiz 'Hello'
fn parse_directive_combined(input: &str) -> ParseResult<AssemblyInstruction> {
    let parser = tuple((
        opt(parse_label_declaration),
        parse_directive_declaration,
        opt(parse_operand),
        opt(parse_operand),
        opt(parse_operand),
    ));

    match parser(input.trim()) {
        Ok((next_input, (label, directive, op1, op2, op3))) => Ok((
            next_input,
            AssemblyInstruction {
                label: label, // opt returns an Option
                directive: Some(directive),
                operand1: op1,
                operand2: op2,
                operand3: op3,
                ..Default::default()
            },
        )),
        Err(e) => Err(e),
    }
}

/// Parses opcode only instructions.
fn parse_instruction0(input: &str) -> ParseResult<AssemblyInstruction> {
    match parse_opcode(input.trim()) {
        Ok((next_input, opcode)) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: Some(opcode),
                ..Default::default()
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
        Ok((next_input, (opcode, reg, num))) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: Some(opcode),
                operand1: Some(reg),
                operand2: Some(num),
                ..Default::default()
            },
        )),
        Err(err) => Err(err),
    }
}

/// Parses instructions of the form:
///     Opcode $reg $reg $reg i.e. ADD $0 $1 $2
fn parse_instruction2(input: &str) -> ParseResult<AssemblyInstruction> {
    let parser = tuple((
        parse_opcode,
        preceded(multispace1, parse_register),
        preceded(multispace1, parse_register),
        preceded(multispace1, parse_register),
    ));

    match parser(input.trim()) {
        Ok((next_input, (opcode, r1, r2, r3))) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: Some(opcode),
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3),
                ..Default::default()
            },
        )),
        Err(err) => Err(err),
    }
}

/// Parses instructions of the form:
///     Opcode $reg $reg i.e. EQ $0 $1
fn parse_instruction3(input: &str) -> ParseResult<AssemblyInstruction> {
    let parser = tuple((
        parse_opcode,
        preceded(multispace1, parse_register),
        preceded(multispace1, parse_register),
    ));

    match parser(input.trim()) {
        Ok((next_input, (opcode, r1, r2))) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: Some(opcode),
                operand1: Some(r1),
                operand2: Some(r2),
                ..Default::default()
            },
        )),
        Err(err) => Err(err),
    }
}

/// Parses instruction of the form:
///       Opcode $reg i.e. Jmp $0
fn parse_instruction4(input: &str) -> ParseResult<AssemblyInstruction> {
    let parser = tuple((parse_opcode, preceded(multispace1, parse_register)));

    match parser(input.trim()) {
        Ok((next_input, (opcode, r1))) => Ok((
            next_input,
            AssemblyInstruction {
                opcode: Some(opcode),
                operand1: Some(r1),
                ..Default::default()
            },
        )),
        Err(err) => Err(err),
    }
}

/// This is the high level instruction parser combinator that parses
/// all forms of instructions.
pub fn parse_instruction(input: &str) -> ParseResult<AssemblyInstruction> {
    // Its important that the opcode only instruction is parsed as the last resort
    // given that its format matches all other types of instructions.
    alt((
        parse_instruction1, // Opcode $reg #num -> LOAD $0 #99
        parse_instruction2, // Opcode $1 $2 $3  -> ADD $0 $2 $3
        parse_instruction3, // Opcode $1 $2     -> EQ $0 $2
        parse_instruction4, // Opcode $2        -> i.e. JMP $2
        parse_instruction0, // HLT
    ))(input)
}

/// Parses a complete program.
pub fn parse_program(input: &str) -> ParseResult<Program> {
    match many1(parse_instruction)(input.trim()) {
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
    fn test_parse_operand() {
        assert_eq!(parse_operand(" #99 "), Ok(("", Token::IntegerOperand(99))));
        assert_eq!(parse_operand(" $23 "), Ok(("", Token::Register(23))));
    }

    #[test]
    fn test_parse_label_declaration() {
        assert_eq!(
            parse_label_declaration("label1: "),
            Ok(("", Token::LabelDeclaration("label1".to_string())))
        );
    }

    #[test]
    fn test_parse_label_usage() {
        assert_eq!(
            parse_label_usage("@label1"),
            Ok(("", Token::LabelUsage("label1".to_string())))
        );
    }

    #[test]
    fn test_parse_directive_declaration() {
        assert_eq!(
            parse_directive_declaration(".code "),
            Ok(("", Token::Directive("code".to_string())))
        );
    }

    #[test]
    fn test_parse_directive_combined() {
        // TODO: Fix this test once we've added support for string literals.
        let result = parse_directive_combined("test1: .asciiz ");
        assert_eq!(result.is_ok(), true);

        let (_, directive) = result.unwrap();

        assert_eq!(
            directive,
            AssemblyInstruction {
                label: Some(Token::LabelDeclaration("test1".to_string())),
                directive: Some(Token::Directive("asciiz".to_string())),
                ..Default::default()
            }
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
                    opcode: Some(Token::Opcode(Opcode::HLT)),
                    ..Default::default()
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
                    opcode: Some(Token::Opcode(Opcode::LOAD)),
                    operand1: Some(Token::Register(9)),
                    operand2: Some(Token::IntegerOperand(299)),
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction2() {
        let result = parse_instruction2("  add $0 $1 $3 \t\n  ");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblyInstruction {
                    opcode: Some(Token::Opcode(Opcode::ADD)),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::Register(1)),
                    operand3: Some(Token::Register(3)),
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction3() {
        let result = parse_instruction3("  EQ $0 $1 \t\n  ");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblyInstruction {
                    opcode: Some(Token::Opcode(Opcode::EQ)),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::Register(1)),
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction4() {
        let result = parse_instruction4("  jmp $30  \t\n  ");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblyInstruction {
                    opcode: Some(Token::Opcode(Opcode::JMP)),
                    operand1: Some(Token::Register(30)),
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_program() {
        let result = parse_program(
            r##" load $0 #100
                 load $1 #200
                 add $0 $1 $2
                 jmp $9
                 EQ $0 $2
                 hlt
                 "##,
        );

        assert_eq!(result.is_ok(), true);

        let (remaining_input, program) = result.unwrap();

        // Ensure that the complete program is consumed.
        assert_eq!("", remaining_input);

        assert_eq!(
            program.instructions[0],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::LOAD)),
                operand1: Some(Token::Register(0)),
                operand2: Some(Token::IntegerOperand(100)),
                ..Default::default()
            }
        );

        assert_eq!(
            program.instructions[1],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::LOAD)),
                operand1: Some(Token::Register(1)),
                operand2: Some(Token::IntegerOperand(200)),
                ..Default::default()
            }
        );

        assert_eq!(
            program.instructions[2],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::ADD)),
                operand1: Some(Token::Register(0)),
                operand2: Some(Token::Register(1)),
                operand3: Some(Token::Register(2)),
                ..Default::default()
            }
        );

        assert_eq!(
            program.instructions[3],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::JMP)),
                operand1: Some(Token::Register(9)),
                ..Default::default()
            }
        );

        assert_eq!(
            program.instructions[4],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::EQ)),
                operand1: Some(Token::Register(0)),
                operand2: Some(Token::Register(2)),
                ..Default::default()
            }
        );

        assert_eq!(
            program.instructions[5],
            AssemblyInstruction {
                opcode: Some(Token::Opcode(Opcode::HLT)),
                ..Default::default()
            }
        );
    }
}
