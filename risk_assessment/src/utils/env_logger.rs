pub fn initialize_env_logger() {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env)
    .format(|buf, record| {
        use chrono::Local;
        use env_logger::fmt::style::{AnsiColor, Style};
        use std::io::Write;

        let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
        let level_style = buf.default_level_style(record.level());

        writeln!(
            buf,
            "{subtle}[{subtle:#}{} {level_style}{:<5}{level_style:#}{subtle}]{subtle:#} {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            record.level(),
            record.args()
        )
    })
    .init();
}