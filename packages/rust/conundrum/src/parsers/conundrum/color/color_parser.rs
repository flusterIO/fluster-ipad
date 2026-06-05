use winnow::Parser;
use winnow::combinator::alt;
use winnow::error::ErrMode;
use winnow::token::literal;

use crate::lang::runtime::state::conundrum_error::ConundrumError;
use crate::lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult};
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::color::conundrum_color::ConundrumColor;

/// For the love of God, do not call this function in the main markdown text.
/// It's super inefficient, but it makes sense where a color is expected.
pub fn named_color(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumColor> {
    if let Ok(res) = literal("blanchedalmond").map(|_| ConundrumColor::from_rgba(255, 235, 205, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("azure").map(|_| ConundrumColor::from_rgba(240, 255, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("olive").map(|_| ConundrumColor::from_rgba(128, 128, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("slateblue").map(|_| ConundrumColor::from_rgba(106, 90, 205, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("navy").map(|_| ConundrumColor::from_rgba(0, 0, 128, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("brown").map(|_| ConundrumColor::from_rgba(165, 42, 42, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("greenyellow").map(|_| ConundrumColor::from_rgba(173, 255, 47, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkmagenta").map(|_| ConundrumColor::from_rgba(139, 0, 139, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("limegreen").map(|_| ConundrumColor::from_rgba(50, 205, 50, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("darkolivegreen").map(|_| ConundrumColor::from_rgba(85, 107, 47, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("navajowhite").map(|_| ConundrumColor::from_rgba(255, 222, 173, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumaquamarine").map(|_| ConundrumColor::from_rgba(102, 205, 170, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lime").map(|_| ConundrumColor::from_rgba(0, 255, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("chocolate").map(|_| ConundrumColor::from_rgba(210, 105, 30, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("gold").map(|_| ConundrumColor::from_rgba(255, 215, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("tomato").map(|_| ConundrumColor::from_rgba(255, 99, 71, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("peachpuff").map(|_| ConundrumColor::from_rgba(255, 218, 185, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lightsalmon").map(|_| ConundrumColor::from_rgba(255, 160, 122, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lightslategrey").map(|_| ConundrumColor::from_rgba(119, 136, 153, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("aquamarine").map(|_| ConundrumColor::from_rgba(127, 255, 212, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("teal").map(|_| ConundrumColor::from_rgba(0, 128, 128, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("lightpink").map(|_| ConundrumColor::from_rgba(255, 182, 193, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumvioletred").map(|_| ConundrumColor::from_rgba(199, 21, 133, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("tan").map(|_| ConundrumColor::from_rgba(210, 180, 140, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("lemonchiffon").map(|_| ConundrumColor::from_rgba(255, 250, 205, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("green").map(|_| ConundrumColor::from_rgba(0, 128, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("olivedrab").map(|_| ConundrumColor::from_rgba(107, 142, 35, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("deepskyblue").map(|_| ConundrumColor::from_rgba(0, 191, 255, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkgray").map(|_| ConundrumColor::from_rgba(169, 169, 169, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("lightgreen").map(|_| ConundrumColor::from_rgba(144, 238, 144, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("mintcream").map(|_| ConundrumColor::from_rgba(245, 255, 250, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("salmon").map(|_| ConundrumColor::from_rgba(250, 128, 114, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("dodgerblue").map(|_| ConundrumColor::from_rgba(30, 144, 255, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumturquoise").map(|_| ConundrumColor::from_rgba(72, 209, 204, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("steelblue").map(|_| ConundrumColor::from_rgba(70, 130, 180, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("lightgrey").map(|_| ConundrumColor::from_rgba(211, 211, 211, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumorchid").map(|_| ConundrumColor::from_rgba(186, 85, 211, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("skyblue").map(|_| ConundrumColor::from_rgba(135, 206, 235, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkviolet").map(|_| ConundrumColor::from_rgba(148, 0, 211, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumseagreen").map(|_| ConundrumColor::from_rgba(60, 179, 113, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("paleturquoise").map(|_| ConundrumColor::from_rgba(175, 238, 238, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("plum").map(|_| ConundrumColor::from_rgba(221, 160, 221, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("blueviolet").map(|_| ConundrumColor::from_rgba(138, 43, 226, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lightslategray").map(|_| ConundrumColor::from_rgba(119, 136, 153, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("slategray").map(|_| ConundrumColor::from_rgba(112, 128, 144, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("ivory").map(|_| ConundrumColor::from_rgba(255, 255, 240, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("snow").map(|_| ConundrumColor::from_rgba(255, 250, 250, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("cornsilk").map(|_| ConundrumColor::from_rgba(255, 248, 220, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkgreen").map(|_| ConundrumColor::from_rgba(0, 100, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("orchid").map(|_| ConundrumColor::from_rgba(218, 112, 214, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("bisque").map(|_| ConundrumColor::from_rgba(255, 228, 196, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("purple").map(|_| ConundrumColor::from_rgba(128, 0, 128, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("orange").map(|_| ConundrumColor::from_rgba(255, 165, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("linen").map(|_| ConundrumColor::from_rgba(250, 240, 230, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("darkslateblue").map(|_| ConundrumColor::from_rgba(72, 61, 139, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("grey").map(|_| ConundrumColor::from_rgba(128, 128, 128, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("beige").map(|_| ConundrumColor::from_rgba(245, 245, 220, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("forestgreen").map(|_| ConundrumColor::from_rgba(34, 139, 34, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lavender").map(|_| ConundrumColor::from_rgba(230, 230, 250, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("darkseagreen").map(|_| ConundrumColor::from_rgba(143, 188, 143, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lavenderblush").map(|_| ConundrumColor::from_rgba(255, 240, 245, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("turquoise").map(|_| ConundrumColor::from_rgba(64, 224, 208, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkkhaki").map(|_| ConundrumColor::from_rgba(189, 183, 107, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("gray").map(|_| ConundrumColor::from_rgba(128, 128, 128, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("lightseagreen").map(|_| ConundrumColor::from_rgba(32, 178, 170, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("palegoldenrod").map(|_| ConundrumColor::from_rgba(238, 232, 170, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("powderblue").map(|_| ConundrumColor::from_rgba(176, 224, 230, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("yellowgreen").map(|_| ConundrumColor::from_rgba(154, 205, 50, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lightblue").map(|_| ConundrumColor::from_rgba(173, 216, 230, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("yellow").map(|_| ConundrumColor::from_rgba(255, 255, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("indigo").map(|_| ConundrumColor::from_rgba(75, 0, 130, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("peru").map(|_| ConundrumColor::from_rgba(205, 133, 63, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkorchid").map(|_| ConundrumColor::from_rgba(153, 50, 204, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkred").map(|_| ConundrumColor::from_rgba(139, 0, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("slategrey").map(|_| ConundrumColor::from_rgba(112, 128, 144, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("oldlace").map(|_| ConundrumColor::from_rgba(253, 245, 230, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("cornflowerblue").map(|_| ConundrumColor::from_rgba(100, 149, 237, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkslategray").map(|_| ConundrumColor::from_rgba(47, 79, 79, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("goldenrod").map(|_| ConundrumColor::from_rgba(218, 165, 32, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darksalmon").map(|_| ConundrumColor::from_rgba(233, 150, 122, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("darkgoldenrod").map(|_| ConundrumColor::from_rgba(184, 134, 11, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("khaki").map(|_| ConundrumColor::from_rgba(240, 230, 140, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("fuchsia").map(|_| ConundrumColor::from_rgba(255, 0, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("magenta").map(|_| ConundrumColor::from_rgba(255, 0, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("palevioletred").map(|_| ConundrumColor::from_rgba(219, 112, 147, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("white").map(|_| ConundrumColor::from_rgba(255, 255, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("violet").map(|_| ConundrumColor::from_rgba(238, 130, 238, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("blue").map(|_| ConundrumColor::from_rgba(0, 0, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("crimson").map(|_| ConundrumColor::from_rgba(220, 20, 60, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("lightgoldenrodyellow").map(|_| ConundrumColor::from_rgba(250, 250, 210, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("maroon").map(|_| ConundrumColor::from_rgba(128, 0, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumpurple").map(|_| ConundrumColor::from_rgba(147, 112, 219, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkblue").map(|_| ConundrumColor::from_rgba(0, 0, 139, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("coral").map(|_| ConundrumColor::from_rgba(255, 127, 80, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("deeppink").map(|_| ConundrumColor::from_rgba(255, 20, 147, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("moccasin").map(|_| ConundrumColor::from_rgba(255, 228, 181, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("lightsteelblue").map(|_| ConundrumColor::from_rgba(176, 196, 222, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lawngreen").map(|_| ConundrumColor::from_rgba(124, 252, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumspringgreen").map(|_| ConundrumColor::from_rgba(0, 250, 154, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("black").map(|_| ConundrumColor::from_rgba(0, 0, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("antiquewhite").map(|_| ConundrumColor::from_rgba(250, 235, 215, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lightcoral").map(|_| ConundrumColor::from_rgba(240, 128, 128, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("midnightblue").map(|_| ConundrumColor::from_rgba(25, 25, 112, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lightskyblue").map(|_| ConundrumColor::from_rgba(135, 206, 250, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("burlywood").map(|_| ConundrumColor::from_rgba(222, 184, 135, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("red").map(|_| ConundrumColor::from_rgba(255, 0, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("lightcyan").map(|_| ConundrumColor::from_rgba(224, 255, 255, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("sandybrown").map(|_| ConundrumColor::from_rgba(244, 164, 96, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkgrey").map(|_| ConundrumColor::from_rgba(169, 169, 169, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("silver").map(|_| ConundrumColor::from_rgba(192, 192, 192, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("springgreen").map(|_| ConundrumColor::from_rgba(0, 255, 127, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("lightgray").map(|_| ConundrumColor::from_rgba(211, 211, 211, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("ghostwhite").map(|_| ConundrumColor::from_rgba(248, 248, 255, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("rosybrown").map(|_| ConundrumColor::from_rgba(188, 143, 143, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("aliceblue").map(|_| ConundrumColor::from_rgba(240, 248, 255, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("dimgray").map(|_| ConundrumColor::from_rgba(105, 105, 105, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("aqua").map(|_| ConundrumColor::from_rgba(0, 255, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkorange").map(|_| ConundrumColor::from_rgba(255, 140, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("hotpink").map(|_| ConundrumColor::from_rgba(255, 105, 180, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("sienna").map(|_| ConundrumColor::from_rgba(160, 82, 45, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("pink").map(|_| ConundrumColor::from_rgba(255, 192, 203, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("dimgrey").map(|_| ConundrumColor::from_rgba(105, 105, 105, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("firebrick").map(|_| ConundrumColor::from_rgba(178, 34, 34, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("saddlebrown").map(|_| ConundrumColor::from_rgba(139, 69, 19, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("mistyrose").map(|_| ConundrumColor::from_rgba(255, 228, 225, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("chartreuse").map(|_| ConundrumColor::from_rgba(127, 255, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("royalblue").map(|_| ConundrumColor::from_rgba(65, 105, 225, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("cadetblue").map(|_| ConundrumColor::from_rgba(95, 158, 160, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("seagreen").map(|_| ConundrumColor::from_rgba(46, 139, 87, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("darkslategrey").map(|_| ConundrumColor::from_rgba(47, 79, 79, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("lightyellow").map(|_| ConundrumColor::from_rgba(255, 255, 224, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("orangered").map(|_| ConundrumColor::from_rgba(255, 69, 0, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("gainsboro").map(|_| ConundrumColor::from_rgba(220, 220, 220, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("thistle").map(|_| ConundrumColor::from_rgba(216, 191, 216, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("palegreen").map(|_| ConundrumColor::from_rgba(152, 251, 152, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("darkturquoise").map(|_| ConundrumColor::from_rgba(0, 206, 209, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("darkcyan").map(|_| ConundrumColor::from_rgba(0, 139, 139, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("mediumslateblue").map(|_| ConundrumColor::from_rgba(123, 104, 238, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("cyan").map(|_| ConundrumColor::from_rgba(0, 255, 255, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) =
        literal("floralwhite").map(|_| ConundrumColor::from_rgba(255, 250, 240, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) =
        literal("rebeccapurple").map(|_| ConundrumColor::from_rgba(102, 51, 153, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("wheat").map(|_| ConundrumColor::from_rgba(245, 222, 179, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("seashell").map(|_| ConundrumColor::from_rgba(255, 245, 238, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("honeydew").map(|_| ConundrumColor::from_rgba(240, 255, 240, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("whitesmoke").map(|_| ConundrumColor::from_rgba(245, 245, 245, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("indianred").map(|_| ConundrumColor::from_rgba(205, 92, 92, 1)).parse_next(input) {
        Ok(res)
    } else if let Ok(res) = literal("papayawhip").map(|_| ConundrumColor::from_rgba(255, 239, 213, 1)).parse_next(input)
    {
        Ok(res)
    } else if let Ok(res) = literal("mediumblue").map(|_| ConundrumColor::from_rgba(0, 0, 205, 1)).parse_next(input) {
        Ok(res)
    } else {
        Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Not a named color"))))
    }
}

