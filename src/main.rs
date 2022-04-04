use crates_index_diff::Index;
use fs_extra::dir::CopyOptions;
use log::{error, info, trace, warn};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// A utility for exporting all changed crates between 2 commits to a seperate folder
#[derive(StructOpt, Debug)]
// #[structopt(name = "basic")]
struct Arguments {
    /// panamax path
    #[structopt(parse(from_os_str))]
    panamax_dir: PathBuf,

    /// src commit or refvar to start with
    #[structopt()]
    starting_commit: String,

    /// the commit to finish the check with
    #[structopt(default_value = "HEAD")]
    end_commit: String,
}

fn get_crate_tar_path(name: &str, version: &str, base_dir: &Path) -> PathBuf {
    let dir = match name.len() {
        1 => base_dir.join("1"),
        2 => base_dir.join("2"),
        3 => base_dir.join("3"),
        _ => base_dir.join(&name[0..2]).join(&name[2..4]),
    };

    dir.join(&name).join(&version)
}

fn main() {
    simple_log::quick!();
    let opt: Arguments = Arguments::from_args();

    let crates = opt.panamax_dir.join("crates");
    let repo = opt.panamax_dir.join("crates.io-index");
    let new_crates = opt.panamax_dir.join("new-crates");

    let base_src_dir = crates.as_path();
    assert!(base_src_dir.is_dir(), "src folder does not exists");

    let base_output_dir = new_crates.as_path();
    if !base_output_dir.is_dir() {
        warn!("output folder does not exists creating one...");
        std::fs::create_dir(base_output_dir).expect("failed to create folder");
    }

    info!("getting list of changed packages");
    let index = Index::from_path_or_cloned(repo).unwrap();
    let crates_changed = index.changes(opt.starting_commit, opt.end_commit).unwrap();

    for changed_crate in crates_changed.iter() {
        let src_path =
            get_crate_tar_path(&changed_crate.name, &changed_crate.version, base_src_dir);
        if !src_path.is_dir() {
            warn!("Skipping {:?} path does not exists", src_path.as_os_str());
            continue;
        }

        let dst_path =
            get_crate_tar_path(&changed_crate.name, &changed_crate.version, base_output_dir);

        fs_extra::dir::create_all(&dst_path, false).expect("failed to create directory");

        let mut options = CopyOptions::new();
        options.skip_exist = true;

        if let Err(e) = fs_extra::dir::copy(&src_path, &dst_path, &options) {
            error!("Failed coping {:?} to {:?} :: {}", src_path, dst_path, e);
        }

        // println!("{:?} -> {:?}", src_path, dst_path);
    }
}
