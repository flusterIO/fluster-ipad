use async_trait::async_trait;

use crate::{
    parse::by_regex::regex_parsers::mdx_parser::{MdxParser, ParserId},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub struct TagRegexParser;

#[async_trait]
impl MdxParser for TagRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Tag
    }
    async fn parse_async(&self, result: &mut MdxParsingResult) {
        // let mut tags: Vec<SharedTaggableModel> = TagEntity::from_pod_data(data);
        // let r = TagEntity::get_tag_regular_expression();

        // let r = Regex::new(r"\[\[#(?<body>[^\]]+)\]\]").unwrap();
        // let mut new_content = String::from(&data.content);
        // let mut tags: Vec<TagResult> = Vec::new();
        // for result in r.captures_iter(&data.content) {
        //     if let Some(body) = result.get(1) {
        //         let body_as_string = body.as_str();
        //         if !tags
        //             .par_iter()
        //             .any(|tag_item| tag_item.value == body_as_string)
        //         {
        //             tags.push(SharedTaggableModel::new(body_as_string.to_string(), None));
        //             new_content = new_content.replace(
        //                 &format!("[[#{}]]", body_as_string),
        //                 &format!("<Tag value={{\"{}\"}} />", body_as_string),
        //             );
        //         }
        //     }
        // }

        // ParsedContentResult {
        //     results: tags,
        //     new_content,
        // }
        // let regex = RegexParser
        // For now, just return the markdown as is.
        // Actual parsing logic will be added later.
        // markdown.to_string()
    }
}
