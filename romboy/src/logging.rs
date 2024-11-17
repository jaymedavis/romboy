use crate::settings::Settings;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init(settings: &Settings) {
    let logging_mode = settings.clone().logging.mode;
    let logging_pattern = "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}";
    let appender = logging_mode.to_lowercase();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logging_pattern)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logging_pattern)))
        .build("logs/app.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender(appender).build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
