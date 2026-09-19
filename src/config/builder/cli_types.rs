use argh::FromArgs;
use compact_str::CompactString;

#[derive(FromArgs, Debug)]
/// A Metrics exporter (json / line_protocol)
pub struct MainCliArguments {
    #[argh(switch, short = 'v')]
    /// enable debugging
    pub verbose: bool,
    #[argh(option, short = 'c')]
    /// set custom config path
    pub config: Option<CompactString>,

}
