use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use kovi::log::LevelFilter;
use spdlog::terminal_style::StyleMode;
use spdlog::{Level, LevelFilter::MoreSevereEqual, sink::StdStreamSink};

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

#[crabtime::function]
fn spawn_bot(plugins: Vec<String>) {
    crabtime::output!(let mut bot = {
        let config: kovi::bot::KoviConf = toml::from_str(
            &std::fs::read_to_string(
                CONFIG_PATH.get().expect("config were not set!")
            )?)?;
        kovi::bot::Bot::build(&config)
    };);

    for plugin in plugins {
        crabtime::output!({
            let plugin = {{plugin}}::__kovi_build_plugin();
            kovi::log::info!("Mounting plugin: {}", &plugin.name);
            bot.mount_plugin(plugin);
        });
    }

    crabtime::output!(
        bot.set_plugin_startup_use_file_ref();
        bot.run();
    );
}

fn main() -> Result<(), Box<dyn snafu::Error>> {
    let Args {
        database_path,
        config_path,
    } = argh::from_env();

    nyquest_preset::register();
    spdlog::init_log_crate_proxy()?;
    kovi::log::set_max_level(LevelFilter::Warn);

    _ = userdb::DATABASE_PATH.set(database_path);
    _ = CONFIG_PATH.set(config_path);

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
    spdlog::swap_default_logger(logger);

    spawn_bot!([
        bind_user,
        help_commands,
        import_records,
        pcrt_plot,
        phigros_tips,
        ping,
        play_voice,
        user_region
    ]);

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
