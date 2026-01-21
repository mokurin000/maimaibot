use std::path::PathBuf;
use std::sync::Arc;

use kovi::log::LevelFilter;
use spdlog::info;
use spdlog::terminal_style::StyleMode;
use spdlog::{Level, LevelFilter::MoreSevereEqual, sink::StdStreamSink};

fn main() -> Result<(), Box<dyn snafu::Error>> {
    let Args {
        database_path,
        config_path,
    } = argh::from_env();

    nyquest_preset::register();
    spdlog::init_log_crate_proxy()?;
    kovi::log::set_max_level(LevelFilter::Info);

    _ = userdb::DATABASE_PATH.set(database_path);

    let logger = spdlog::default_logger().fork_with(|log| {
        log.set_level_filter(MoreSevereEqual(if cfg!(debug_assertions) {
            Level::Debug
        } else {
            Level::Info
        }));
        let sink = StdStreamSink::builder()
            .stderr()
            .style_mode(StyleMode::Always)
            .build()?;
        *log.sinks_mut() = vec![Arc::new(sink)];
        Ok(())
    })?;
    _ = spdlog::swap_default_logger(logger);

    let mut bot = {
        let config: kovi::bot::KoviConf = toml::from_str(&std::fs::read_to_string(config_path)?)?;
        info!("loaded config: {config:?}");
        kovi::bot::Bot::build(&config)
    };
    for plugin_cotr in [
        bind_user::__kovi_build_plugin,
        help_commands::__kovi_build_plugin,
        import_records::__kovi_build_plugin,
        pcrt_plot::__kovi_build_plugin,
        phigros_tips::__kovi_build_plugin,
        ping::__kovi_build_plugin,
        play_voice::__kovi_build_plugin,
    ] {
        let plugin = plugin_cotr();
        info!("Mounting: {}", plugin.name);
        bot.mount_plugin(plugin);
    }

    // access control
    bot.set_plugin_startup_use_file_ref();
    bot.run();

    Ok(())
}
#[derive(argh::FromArgs)]
#[argh(description = "Maimai bot.")]
struct Args {
    /// path of user data database
    #[argh(option, default = "PathBuf::from(\"userdata.db\")")]
    database_path: PathBuf,

    /// path of user data database
    #[argh(option, default = "PathBuf::from(\"kovi.conf.toml\")")]
    config_path: PathBuf,
}
