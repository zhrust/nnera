#![allow(unused)]
//use std::env;
//use std::path::PathBuf;
//use log::debug;
//use clia_tracing_config::build;
//use clia_tracing_config::{Config as ConfigBase};
//use clap::{App, Arg};
//use clap::Parser;
use structopt::StructOpt;

//use nn::{Command, Opt};
use nn::Opt;

fn main() {
    //println!("Hello, world!");
    //log::debug!("NN:\n\t {:?}",_guard);
    //log::debug!("NN:\n\t {:?}",_guard["WorkerGuard"]);

    let opt = Opt::from_args();
    //opt.run();
    nn::run(opt);

    //log::debug!("NN:\n\t {:?}",args);
}
