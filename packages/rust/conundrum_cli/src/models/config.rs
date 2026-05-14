use conundrum::lang::runtime::run_conundrum::ParseConundrumOptions;

/// ## Plans
/// Shocker... I've accomplished none of these yet. This is basically just
/// a placeholder until I get [Fluster](https://flusterapp.com) released and I'm in a better living
/// situation to where I can focus more time on the cli.
///
/// - [ ] Read from file
///   - [ ] yaml
///   - [ ] json
///   - [ ] Conundrum (a future goal, once the logic layer is in place)
/// - [ ] Accept config file path as parameter.
/// - [ ] Allow global and project specific configuration.
pub struct CliConfig(pub ParseConundrumOptions);

impl CliConfig {
    pub fn read() -> Self {
        CliConfig(ParseConundrumOptions::default())
    }
}
