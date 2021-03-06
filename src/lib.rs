#![feature(proc_macro_hygiene)]

use arcropolis_api::*;
use std::path::{Path, PathBuf};
mod config;
use config::*;

#[arc_callback]
fn normal_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    match SHARED_FILES.lock().unwrap().get(&hash) {
        Some(info) => {
            match std::fs::read(info.path.to_path_buf()) {
                Ok(file) => {
                    data[..file.len()].copy_from_slice(&file);
                    Some(file.len())
                }
                Err(_err) => {
                    match std::fs::read(&info.fuse_path) {
                        Ok(fuse_file) => {
                            data[..fuse_file.len()].copy_from_slice(&fuse_file);
                            Some(fuse_file.len())
                        }
                        Err(_err) => {
                            load_original_file(&info.hash, data)
                        }
                    }
                }
            }
        },
        None => load_original_file(hash, data)
    }
}

#[stream_callback]
fn stream_callback(hash: u64) -> Option<PathBuf> {
    match SHARED_FILES.lock().unwrap().get(&hash) {
        Some(info) => Some(info.path.to_path_buf()),
        None => None
    }
}

fn get_configs() {
    read_from_umm_path(Path::new("sd:/ultimate/mods"));
    println!("[Shared Files::get_configs] Finished reading UMM path!");
}

#[skyline::main(name = "share-files")]
pub fn main() {
    get_configs();

    for (k, v) in SHARED_FILES.lock().unwrap().iter() {
        match v.section {
            Section::Normal => {
                match v.size {
                    Some(size) => {
                        normal_callback::install(*k, size)
                    },
                    None => {
                        println!(
                            "[Shared Files::main] Did not install callback {:#x} since file was not found ({}).",
                            v.hash.as_u64(),
                            v.fuse_path,
                            v.path.display()
                        );
                    }
                }
            },
            Section::Stream => stream_callback::install(*k)
        }
    }
}