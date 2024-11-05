// pub fn initialize_env_logger() {
//     let env = env_logger::Env::default();
//     env_logger::Builder::from_env(env)
//     .format(|buf, record| {
//         use chrono::Local;
//         use env_logger::fmt::style::{AnsiColor, Style};
//         use std::io::Write;

//         let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
//         let level_style = buf.default_level_style(record.level());

//         writeln!(
//             buf,
//             "{subtle}[{subtle:#}{} {level_style}{:<5}{level_style:#}{subtle}]{subtle:#} {}",
//             Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
//             record.level(),
//             record.args()
//         )
//     })
//     .init();
// }

use log::Level;

pub fn initialize_env_logger() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            use chrono::Local;
            // use env_logger::fmt::style::{AnsiColor, Style};
            use std::io::Write;

            // let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            let level_style = buf.default_level_style(record.level());
            
            // Check environment variable to see if time should be included
            let show_time = std::env::var("LOG_SHOW_TIME").unwrap_or_else(|_| "false".into()) == "true";

            if show_time {
                if record.level() == Level::Info || record.level() == Level::Warn {
                    writeln!(
                        buf,
                        "[{level_style}{:<4}{level_style:#}] [{}] [{}] {}",
                        record.level(),
                        record.target(),
                        Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                        record.args()
                    )
                } else {
                    writeln!(
                        buf,
                        "[{level_style}{:<5}{level_style:#}][{}] [{}] {}",
                        record.level(),
                        record.target(),
                        Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                        record.args()
                    )
                }
                
            } else {
                if record.level() == Level::Info || record.level() == Level::Warn {
                writeln!(
                    buf,
                    "[{level_style}{:<4}{level_style:#}] [{}] {}",
                    record.level(),
                    record.target(),
                    record.args()
                )
            } else {
                writeln!(
                    buf,
                    "[{level_style}{:<5}{level_style:#}][{}] {}",
                    record.level(),
                    record.target(),
                    record.args()
                )
            }
            }
        })
        .init();
}

// pub fn initialize_env_logger() {
//     let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
//     env_logger::Builder::from_env(env)
//         .format(|buf, record| {
//             use chrono::Local;
//             use env_logger::fmt::style::{AnsiColor, Style};
//             use std::io::Write;

//             let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
//             let level_style = buf.default_level_style(record.level());

//             // Default to not showing the time unless explicitly set
//             let show_time = std::env::var("LOG_SHOW_TIME").unwrap_or_else(|_| "false".into()) == "true";

//             // Create a formatted level string with brackets; add a trailing space if the level is short
//             let level_str = if record.level().as_str().len() < 5 {
//                 format!("[{: ^5}] ", record.level()) // Space after the bracket if level is short
//             } else {
//                 format!("[{}]", record.level()) // No extra space for longer levels
//             };

//             if show_time {
//                 writeln!(
//                     buf,
//                     "{} [{}]  [{}]  {}",
//                     level_style.(level_str), // Apply the color style
//                     record.target(),
//                     Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
//                     record.args()
//                 )
//             } else {
//                 writeln!(
//                     buf,
//                     "{} [{}]  {}",
//                     level_style.apply_to(level_str), // Apply the color style
//                     record.target(),
//                     record.args()
//                 )
//             }
//         })
//         .init();
// }


// pub fn initialize_env_logger() {
//     let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
//     env_logger::Builder::from_env(env)
//         .format(|buf, record| {
//             use chrono::Local;
//             use env_logger::fmt::style::{AnsiColor, Style};
//             use std::io::Write;

//             let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
//             let level_style = buf.default_level_style(record.level());

//             // Default to not showing the time unless explicitly set
//             let show_time = std::env::var("LOG_SHOW_TIME").unwrap_or_else(|_| "false".into()) == "true";

//             // Create a formatted level string with brackets; add a trailing space if the level is short
//             let level_str = if record.level().as_str().len() < 5 {
//                 format!("[{: ^5}] ", record.level()) // Space after the bracket if level is short
//             } else {
//                 format!("[{}]", record.level()) // No extra space for longer levels
//             };

//             // Format with or without time based on `show_time` flag
//             if show_time {
//                 writeln!(
//                     buf,
//                     "{level_style}[{level_style:#}{}]  [{subtle}{}]  [{}]  {subtle:#}{}",
//                     level_str,
//                     record.target(),
//                     Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
//                     record.args()
//                 )
//             } else {
//                 writeln!(
//                     buf,
//                     "{level_style}[{level_style:#}{}]  [{subtle}{}]  {subtle:#}{}",
//                     level_str,
//                     record.target(),
//                     record.args()
//                 )
//             }
//         })
//         .init();
// }
