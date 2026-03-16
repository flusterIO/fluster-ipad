use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;

pub trait MdxComponentResult {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String;
}
