use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::lang::{
    elements::parsed_elements::ParsedElement,
    runtime::{
        compile_conundrum::compile_elements,
        parse_conundrum_string::parse_conundrum_string,
        state::{
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
            parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
        },
        traits::conundrum_input::get_conundrum_input,
    },
};

#[typeshare::typeshare]
#[derive(uniffi::Record, Debug, Serialize, Deserialize)]
pub struct TitleGroup {
    pub title: String,
    pub subtitle: Option<String>,
}

pub fn get_title_group(content: String,
                       modifiers: Vec<ConundrumModifier>,
                       target: ConundrumCompileTarget)
                       -> ConundrumResult<TitleGroup> {
    let mut input = get_conundrum_input(content.as_str(),
                                        ParseState { modifiers,
                                                     compile_target: target,
                                                     ..Default::default() });
    if let Ok((ems, res)) = parse_conundrum_string(&mut input) {
        let x = ems.par_iter().find_map_first(|em| match em {
                                  ParsedElement::Heading(h) => Some(h),
                                  _ => None,
                              });

        match x {
            Some(heading) => {
                // let mut state = res.state.borrow_mut();
                let state = Arc::clone(&res.state);
                let title_string = compile_elements(&heading.children.0, &state)?;
                let subtitle_string = match &heading.subtitle {
                    Some(s) => {
                        let res = compile_elements(&s.0, &state)?;
                        Some(res)
                    }
                    None => None,
                };
                Ok(TitleGroup { title: title_string,
                                subtitle: subtitle_string })
            }
            None => Err(ConundrumErrorVariant::FailToFindTitleGroup),
        }
    } else {
        Err(ConundrumErrorVariant::FailToFindTitleGroup)
    }
}

#[cfg(test)]
mod tests {
    use fluster_core_utilities::test_utilities::get_test_mdx_content;

    use super::*;

    #[test]
    fn gets_proper_title_group() {
        let test_content = get_test_mdx_content::get_test_note_content_with_everything();

        let title_group =
            get_title_group(test_content,
                            vec![ConundrumModifier::PreferInlineMarkdownSyntax],
                            ConundrumCompileTarget::Markdown).expect("Finds title group when title is present.");

        assert!(title_group.title == "Metuque at _inferius_ nebulas", "Finds the proper title");
        println!("Title Group: {:#?}", title_group);

        assert!(title_group.subtitle.is_some_and(|s| s == "My note with a subtitle"), "Finds the proper subtitle");
    }
}
