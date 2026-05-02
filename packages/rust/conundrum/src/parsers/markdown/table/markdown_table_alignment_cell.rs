use winnow::{
    Parser,
    combinator::{alt, repeat},
    stream::Stream,
};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Default, serde::Serialize, Clone)]
pub enum MarkdownTableAlignmentCell {
    #[default]
    Default,
    Left,
    Right,
    Center,
}

pub fn default_aligned_table_cell(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableAlignmentCell> {
    let _: Vec<char> = repeat(1.., '-').parse_next(input)?;
    Ok(MarkdownTableAlignmentCell::Default)
}

pub fn left_aligned_alignment_table_cell(input: &mut ConundrumInput)
                                         -> ConundrumModalResult<MarkdownTableAlignmentCell> {
    let start = input.input.checkpoint();
    ':'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    default_aligned_table_cell.void().parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&start);
                                                        })?;
    Ok(MarkdownTableAlignmentCell::Left)
}

pub fn right_aligned_alignment_table_cell(input: &mut ConundrumInput)
                                          -> ConundrumModalResult<MarkdownTableAlignmentCell> {
    let start = input.input.checkpoint();
    default_aligned_table_cell.void().parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&start);
                                                        })?;

    ':'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(MarkdownTableAlignmentCell::Right)
}

pub fn center_aligned_alignment_table_cell(input: &mut ConundrumInput)
                                           -> ConundrumModalResult<MarkdownTableAlignmentCell> {
    let start = input.input.checkpoint();
    ':'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    default_aligned_table_cell.void().parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&start);
                                                        })?;

    ':'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(MarkdownTableAlignmentCell::Center)
}

impl ConundrumParser<MarkdownTableAlignmentCell> for MarkdownTableAlignmentCell {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableAlignmentCell> {
        alt((center_aligned_alignment_table_cell,
             left_aligned_alignment_table_cell,
             right_aligned_alignment_table_cell,
             default_aligned_table_cell)).parse_next(input)
    }

    fn matches_first_char(char: char) -> bool {
        char == ':' || char == '-'
    }
}
