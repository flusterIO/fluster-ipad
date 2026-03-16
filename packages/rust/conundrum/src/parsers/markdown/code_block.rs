use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::complete::{anychar, line_ending, multispace0, space0},
    combinator::opt,
};

use crate::lang::elements::parsed_code_block::ParsedCodeBlock;

pub fn parse_markdown_block(input: &str) -> IResult<&str, ParsedCodeBlock> {
    // 1. Skip leading whitespace but keep track of the start point
    let (i, _) = multispace0(input)?;
    let start_point = i;

    // 2. Parse the opening: backticks and language
    let (i, ticks) = take_while1(|c: char| c == '`')(i)?;
    let (i, language) = take_while1(|c: char| c != ' ' && c != '\t' && c != '\n' && c != '\r')(i)?;

    // 3. Optional metadata
    let (i, meta_opt) = opt(|input| {
        let (input, _) = space0(input)?;
        let (input, _) = tag("--")(input)?;
        let (input, _) = space0(input)?;
        take_till(|c| c == '\n' || c == '\r')(input)
    })
    .parse(i)?;

    let (i, _) = space0(i)?;
    let (i, _) = line_ending(i)?;

    // 4. Capture content until exact backtick match
    let (i, raw_content) = take_until(ticks)(i)?;
    let (i, end_ticks) = tag(ticks)(i)?;

    // 5. Calculate the full_match span
    // We calculate the length consumed and slice from the start_point
    let consumed_len =
        end_ticks.as_ptr() as usize + end_ticks.len() - start_point.as_ptr() as usize;
    let full_match = &start_point[..consumed_len];

    let meta_data = meta_opt
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let content = raw_content
        .strip_suffix("\r\n")
        .or_else(|| raw_content.strip_suffix('\n'))
        .unwrap_or(raw_content);

    Ok((
        i,
        ParsedCodeBlock {
            language: language.to_string(),
            meta_data,
            content: content.to_string(),
            full_match: full_match.to_string(),
        },
    ))
}

pub fn parse_all_blocks(input: &str) -> Vec<ParsedCodeBlock> {
    let mut results = Vec::new();
    let mut curr_input = input;

    while !curr_input.is_empty() {
        match parse_markdown_block(curr_input) {
            Ok((next_input, block)) => {
                results.push(block);
                curr_input = next_input;
            }
            Err(_) => {
                // If no block found at this position, skip one character
                let mut chars = curr_input.chars();
                if chars.next().is_some() {
                    curr_input = chars.as_str();
                } else {
                    break;
                }
            }
        }
    }
    results
}
