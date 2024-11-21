use std::{path::PathBuf, process::exit};

pub struct Args {
    pub file: PathBuf,
    pub outdir: PathBuf,
}

fn print_usage(exe: &str) -> ! {
    eprintln!("angle - (c) buj 2024");
    eprintln!("This software is distributed under GPLv3 (or later) license.");
    eprintln!();
    eprintln!("Figura .moon to avatar dir converter");
    eprintln!();
    eprintln!("usage: {exe} [infile] [outdir]");
    exit(-1);
}

pub fn parse_args() -> Args {
    let mut iter = std::env::args();
    let exe = iter.next().unwrap_or("angle".into());
    let Some(file) = iter.next() else {
        print_usage(&exe);
    };
    let Some(outdir) = iter.next() else {
        print_usage(&exe);
    };

    Args {
        file: file.into(),
        outdir: outdir.into(),
    }
}
