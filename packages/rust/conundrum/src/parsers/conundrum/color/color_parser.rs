pub fn named_color(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumColor> {
    alt((
            
            literal("blanchedalmond").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("azure").map(|_| {
                ConundrumColor::from_rgba(0, 1, 1, 1)
            }),
            
            literal("olive").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("slateblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("navy").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("brown").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("greenyellow").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("darkmagenta").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("limegreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkolivegreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("navajowhite").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("mediumaquamarine").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lime").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("chocolate").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("gold").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("tomato").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("peachpuff").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("lightsalmon").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("lightslategrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("aquamarine").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("teal").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightpink").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("mediumvioletred").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("tan").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lemonchiffon").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("green").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("olivedrab").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("deepskyblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 1, 1)
            }),
            
            literal("darkgray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightgreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mintcream").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("salmon").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("dodgerblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 1, 1)
            }),
            
            literal("mediumturquoise").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("steelblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightgrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mediumorchid").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("skyblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkviolet").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mediumseagreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("paleturquoise").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("plum").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("blueviolet").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightslategray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("slategray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("ivory").map(|_| {
                ConundrumColor::from_rgba(1, 1, 0, 1)
            }),
            
            literal("snow").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("cornsilk").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("darkgreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("orchid").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("bisque").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("purple").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("orange").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("linen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkslateblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("grey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("beige").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("forestgreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lavender").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkseagreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lavenderblush").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("turquoise").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkkhaki").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("gray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightseagreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("palegoldenrod").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("powderblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("yellowgreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("yellow").map(|_| {
                ConundrumColor::from_rgba(1, 1, 0, 1)
            }),
            
            literal("indigo").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("peru").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkorchid").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkred").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("slategrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("oldlace").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("cornflowerblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkslategray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("goldenrod").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darksalmon").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkgoldenrod").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("khaki").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("fuchsia").map(|_| {
                ConundrumColor::from_rgba(1, 0, 1, 1)
            }),
            
            literal("magenta").map(|_| {
                ConundrumColor::from_rgba(1, 0, 1, 1)
            }),
            
            literal("palevioletred").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("white").map(|_| {
                ConundrumColor::from_rgba(1, 1, 1, 1)
            }),
            
            literal("violet").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("blue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 1, 1)
            }),
            
            literal("crimson").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightgoldenrodyellow").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("maroon").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mediumpurple").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("coral").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("deeppink").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("moccasin").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("lightsteelblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lawngreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mediumspringgreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("black").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("antiquewhite").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightcoral").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("midnightblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightskyblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("burlywood").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("red").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("lightcyan").map(|_| {
                ConundrumColor::from_rgba(0, 1, 1, 1)
            }),
            
            literal("sandybrown").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkgrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("silver").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("springgreen").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("lightgray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("ghostwhite").map(|_| {
                ConundrumColor::from_rgba(0, 0, 1, 1)
            }),
            
            literal("rosybrown").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("aliceblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 1, 1)
            }),
            
            literal("dimgray").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("aqua").map(|_| {
                ConundrumColor::from_rgba(0, 1, 1, 1)
            }),
            
            literal("darkorange").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("hotpink").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("sienna").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("pink").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("dimgrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("firebrick").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("saddlebrown").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mistyrose").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("chartreuse").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("royalblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("cadetblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("seagreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkslategrey").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("lightyellow").map(|_| {
                ConundrumColor::from_rgba(1, 1, 0, 1)
            }),
            
            literal("orangered").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("gainsboro").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("thistle").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("palegreen").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkturquoise").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("darkcyan").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("mediumslateblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("cyan").map(|_| {
                ConundrumColor::from_rgba(0, 1, 1, 1)
            }),
            
            literal("floralwhite").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("rebeccapurple").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("wheat").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("seashell").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("honeydew").map(|_| {
                ConundrumColor::from_rgba(0, 1, 0, 1)
            }),
            
            literal("whitesmoke").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("indianred").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
            literal("papayawhip").map(|_| {
                ConundrumColor::from_rgba(1, 0, 0, 1)
            }),
            
            literal("mediumblue").map(|_| {
                ConundrumColor::from_rgba(0, 0, 0, 1)
            }),
            
    )).parse_next(input)
}