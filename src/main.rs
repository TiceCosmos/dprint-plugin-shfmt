mod configuration;
mod format_text;
mod plugin;

pub use plugin::*;

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

    use dprint_core::plugins::process::{handle_process_stdio_messages, start_parent_process_checker_thread};

    let parent_process_id = opt.parent_pid;
    start_parent_process_checker_thread(String::from(env!("CARGO_PKG_NAME")), parent_process_id);

    handle_process_stdio_messages(MyProcessPluginHandler::default())
}
