use std::sync::Arc;

use spdlog::terminal_style::StyleMode;
use spdlog::{Level, LevelFilter::MoreSevereEqual, sink::StdStreamSink};

use kovi::build_bot;

fn main() -> Result<(), Box<dyn snafu::Error>> {
    nyquest_preset::register();

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

    build_bot!(kovi_plugin_cmd, ping, bind_user, phigros_tips, user_region, help_commands).run();
    Ok(())
}
