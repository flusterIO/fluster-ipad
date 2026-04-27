use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

/// As a super easy target for other developers and the place where Conundrum
/// originated, compiling to mdx will allow developers to fill only a subset of
/// components as truly required while letting a markdown rendering library
/// handle the rest.
///
/// Rendering to mdx offers other advantages over html & javascript as well,
/// like the abiility to maintain complex state more easily in a React
/// environment as opposed to plain JS approaches.
///
/// The downside however is obvious... that extra React render cycle. That was
/// one of the biggest motivators towards creating an independent language....
/// to have complete control of the Render cycle, and by embedding the output in
/// a stateful application in a manner where your components actually depends on
/// global state, you're kind of giving that up. It's a give-take though... so
/// you do you.
pub trait MdxComponentResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}
