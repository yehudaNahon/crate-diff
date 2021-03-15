use crates_index_diff::Index;
use fs_extra::dir::CopyOptions;
use log::{error, info, trace, warn};
use std::path::PathBuf;
use structopt::StructOpt;
/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CratesDiff {
    /// repository path
    #[structopt(parse(from_os_str))]
    repo: PathBuf,

    /// crates path
    #[structopt(parse(from_os_str))]
    crates: PathBuf,

    /// commit or refvar to start with
    #[structopt()]
    starting_commit: String,

    /// a directory to copy all exported files to
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// the commit to finish the check with
    #[structopt(default_value = "HEAD")]
    end_commit: String,
}

fn main() {
    env_logger::init();
    let opt: CratesDiff = CratesDiff::from_args();

    let base_src_dir = opt.crates.as_path();
    assert!(base_src_dir.is_dir(), "src folder does not exists");

    let base_output_dir = opt.output.as_path();
    if !base_output_dir.is_dir() {
        warn!("output folder does not exists creating one...");
        std::fs::create_dir(base_output_dir).expect("failed to create folder");
    }

    info!("getting list of changed packages");
    let index = Index::from_path_or_cloned(opt.repo).unwrap();
    let crates_changed = index.changes(opt.starting_commit, opt.end_commit).unwrap();

    for changed_crate in crates_changed.iter() {
        let src_path = base_src_dir
            .join(&changed_crate.name)
            .join(&changed_crate.version);
        if !src_path.is_dir() {
            warn!("Skipping {:?} path does not exists", src_path.as_os_str());
            continue;
        }

        let dst_path = base_output_dir.join(&changed_crate.name);

        fs_extra::dir::create_all(&dst_path, false).expect("failed to create directory");

        let mut options = CopyOptions::new();
        options.skip_exist = true;

        if let Err(e) = fs_extra::dir::copy(&src_path, &dst_path, &options) {
            error!("Failed coping {:?} to {:?} :: {}", src_path, dst_path, e);
        }

        // println!("{:?} -> {:?}", src_path, dst_path);
    }
}
