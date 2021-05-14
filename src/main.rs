mod configuration;
mod format_text;
mod process_plugin;

pub use configuration::resolve_config;
pub use format_text::format_text;
pub use process_plugin::*;

const CONFIG_KEY: &str = "shfmt";
const FILE_EXTENSIONS: [&str; 1] = ["sh"];
const HELP_URL: &str = "https://github.com/mvdan/sh/blob/master/cmd/shfmt/shfmt.1.scd#examples";
const CONFIG_SCHEMA_URL: &str = "";
const LICENSE_TEXT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE"));

fn main() -> Result<(), dprint_core::types::ErrBox> {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    struct Opt {
        /// init
        #[structopt(long)]
        init: bool,
        /// parent-pid
        #[structopt(long = "parent-pid")]
        parent_pid: u32,
    }

    let opt = Opt::from_args();

    use dprint_core::plugins::process::{
        handle_process_stdio_messages, start_parent_process_checker_thread,
    };

    let parent_process_id = opt.parent_pid;
    start_parent_process_checker_thread(String::from(env!("CARGO_PKG_NAME")), parent_process_id);

    handle_process_stdio_messages(MyProcessPluginHandler::default())
}
