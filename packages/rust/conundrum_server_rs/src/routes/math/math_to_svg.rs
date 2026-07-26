// use ratex_layout::to_display_list;
// use ratex_parser::parse;
// use ratex_svg::{SvgOptions, render_to_svg};

// use crate::errors::server_error::ServerResult;
// println!("{}", svg_string);

// pub async fn math_to_svg() -> ServerResult<String> {
//     let tex_string = "x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}";
//     let parse_tree = parse(tex_string).unwrap();
//     let display_list = to_display_list(&parse_tree);

//     // Render options (defaults to webfont text nodes)
//     let options = SvgOptions { font_size: 10.0,
//                                padding: 2.0,
//                                embed_glyphs: false,
//                                stroke_width: Default::default(),
//                                font_dir: Default::default() };

//     let svg_string = render_to_svg(&display_list, &options);
//     Ok(String::from(""))
// }
