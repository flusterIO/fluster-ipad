use crate::lang::runtime::state::parse_state::ParseState;

pub trait MdxComponentResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String;
}
