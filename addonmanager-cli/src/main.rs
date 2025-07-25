use std::error::Error;
use std::process;

use clap::Parser;

use downloader_core::banner;
use downloader_core::constants::{
    default_description, default_figure_text, default_manifest_url, default_version_label,
};

#[cfg(target_os = "windows")]
use std::io::Write;

fn main() {
    #[cfg(not(unix))]
    colored::control::set_virtual_terminal(true).unwrap();

    banner::print_banner(default_figure_text(), &default_description());
}
