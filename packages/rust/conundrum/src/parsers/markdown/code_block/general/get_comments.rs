use anyhow::Result;
use std::str::FromStr;
use syntect::{
    easy::ScopeRegionIterator,
    highlighting::ScopeSelector,
    parsing::{ParseState, ScopeStack, ScopeStackOp, SyntaxSet},
};
use winnow::error::ErrMode;

use crate::{
    lang::runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax,
};

// TODO: Handle this when you have more time. It'd be a cool feature to be able
// to populate various outputs from comment context inside of code blocks in a
// variety of languages.
// There's more info in the syntex repo examples dir.
fn get_comment_line(ops: &[(usize, ScopeStackOp)], line: &str, stack: &mut ScopeStack) -> Result<Option<String>> {
    let comment_selector = ScopeSelector::from_str("comment - comment.block.attribute").unwrap();
    // let doc_comment_selector =
    //     ScopeSelectors::from_str("comment.line.documentation,
    // comment.block.documentation").unwrap();
    for (s, op) in ScopeRegionIterator::new(ops, line) {
        stack.apply(op)?;
        if s.is_empty() {
            continue;
        }
        if comment_selector.does_match(stack.as_slice()).is_some() {}
    }
    Ok(None)
}

pub fn get_comments(code: &str, lang: SupportedCodeBlockSyntax) -> ConundrumModalResult<String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = lang.load_syntax(&ss)?;
    let mut state = ParseState::new(&syntax);
    let mut stack = ScopeStack::new();
    for l in code.lines() {
        let output = state.parse_line(l, &ss).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Backtrack(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Parser error", "Conundrum could not gather documentation comments out of one of your codeblocks successfully."))
            )
        })?;
        get_comment_line(&output, &l, &mut stack);
    }

    Ok(String::from(""))
}
