use color_eyre::owo_colors::OwoColorize;
use indoc::formatdoc;
use rig::completion::Usage;

pub fn log_usage(usage: Usage) {
    let s = formatdoc! {"
            Tokens Expended
            Input:    {}
            Output:   {}
            Total:    {}
                ", usage.input_tokens.bright_green(), usage.output_tokens.bright_red(), usage.total_tokens};
    log::info!("\n\n{}", s);
}
