#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

mod setup;
mod subcommands;

use clap::{App, AppSettings, ArgMatches};
use subcommands::{pull, list, add, delete, clone, cmd, cmds, status};
use setup::SettingsFile;


fn main() {
    let settings_file = SettingsFile::new();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    call_subcommands(matches, settings_file)
}

fn call_subcommands(matches: ArgMatches, settings_file: SettingsFile) {
    if matches.is_present("clone") {
        clone(matches, settings_file)
    } else if matches.is_present("pull") {
        pull(matches, settings_file)
    } else if matches.is_present("ls") {
        list(matches, settings_file)
    } else if matches.is_present("add") {
        add(matches, settings_file)
    } else if matches.is_present("rm") {
        delete(matches, settings_file)
    } else if matches.is_present("cmd") {
        cmd(matches, settings_file)
    } else if matches.is_present("cmds") {
        cmds(matches, settings_file)
    } else if matches.is_present("status") {
        status(matches, settings_file)
    }
}

