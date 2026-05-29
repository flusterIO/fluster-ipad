use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserFlags, ParserOptions},
    targets::{Browsers, Features, Targets},
};

// TODO: Remove all references to these FLuster specific methods once there's
// time to focus on other platforms. Right now, I need a paycheck *desperately*,
// so I'm focusing on getting the app released.
pub fn safari_specific_lightning_css_printer_options<'a>() -> PrinterOptions<'a> {
    PrinterOptions { // I think this 'minify' will be faster? I imagine so, since we're
                     // often going to be compiling on
                     // change.
                     minify: false,
                     project_root: None,
                     analyze_dependencies: None,
                     // source_map: None,
                     pseudo_classes: None,
                     targets: Targets { browsers: Some(Browsers { safari: Some((13 << 16) | (2 << 8)),
                                                                  ..Default::default() }),
                                        include: Features::MediaQueries,
                                        exclude: Features::empty() } }
}

pub fn fluster_specific_stylesheet_parser_opts<'i, 'o>() -> ParserOptions<'i, 'o> {
    ParserOptions { filename: "conundrum.scss".to_string(),
                    css_modules: None,
                    source_index: 0,
                    error_recovery: true,
                    warnings: None,
                    flags: ParserFlags::empty() }
}
