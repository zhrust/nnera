//use std::time::SystemTime;
use structopt::StructOpt;

use chrono::prelude::*;
//use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono::{DateTime, Duration, Utc};
use chrono::{Local, NaiveDate};

#[derive(Debug, StructOpt)]
#[structopt(name = "NN", author, about)]
pub struct Opt {
    //#[structopt(short, long, default_value = "42")]
    //echo: String,
    //#[structopt(short, long, default_value = "world")]
    #[structopt(short, long, help = "42 <- NN born 42 day,is witch date?")]
    echo: Option<String>,

    #[structopt(short, long, help = "yymmdd <- this date is NN born ? days")]
    date: Option<String>, //String,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
    //#[structopt(subcommand)]
    //cmd: Option<Command>,  // cmd 变成可选项
}

pub fn run(opt: Opt) {
    let _guard = clia_tracing_config::build()
        .filter_level("error") //fatal,error,warn,info,debug
        .with_ansi(true)
        .to_stdout(false)
        .directory("./log")
        .file_name("debug.log")
        .rolling("daily")
        .init();

    let origin: DateTime<Utc> = Utc.with_ymd_and_hms(2009, 5, 19, 0, 0, 42).unwrap();

    match (opt.echo, opt.date) {
        (None, None) => {
            log::debug!("NN:\n\t {}", "default echo today's NNera");
            if opt.verbose > 0 {
                println!("default echo today's NNera");
            }

            let now = Local::now();
            log::debug!("NN orig. UTC: {}", origin.format("%y%m%d %H:%M:%S %P %z"));
            let di4nn = now.signed_duration_since(origin).num_days();
            log::debug!("NN ~= {}", di4nn);
            println!("{} ~> doday's NNera \n\t more info. $ nn -h", di4nn);

        }
        (Some(echo), None) => {
            //println!("This is NN {} era.", chrono::Local::now().format("%y%m%d"));
            log::debug!("-e NN\n\t echo {} -> witch date", echo);

            let nndays: i64 = match echo.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };
            let days = Duration::days(nndays as i64);
            let future_date = origin + days;
            let formatted_date = future_date.format("%Y-%m-%d").to_string();
            println!("{} <~ date as NNera {}", formatted_date, echo); // 输出：2035-02-15

            if opt.verbose > 0 {
                println!("echo :{} is what YYYY-mm-dd", echo);
            }
        }
        (None, Some(date)) => {
            //let _ts = chrono::Local::now().format("%y%m%d");
            //log::debug!("-d yymmdd\n\t date {} <- menas NNera what?", _ts);

            let aim_date = NaiveDate::parse_from_str(&date.trim(), "%y%m%d").unwrap();
            let aim_datetime = aim_date.and_hms_opt(0, 0, 0).unwrap();
            let local_date = Local.from_local_datetime(&aim_datetime).unwrap();


            //println!("Parse Date from String: {aim_date:?}");
            log::debug!("-d yymmdd\n\t Local:: {} ", local_date);

            let di4nn = local_date.signed_duration_since(origin).num_days();
            println!("{} ~> NNera for {}", di4nn, aim_date);

            if opt.verbose > 0 {
                println!("date :{} means NNera what?", date);
            }
        }
        (Some(echo), Some(date)) => {
            println!("-e or -d NEED only one");
            log::debug!("got all\n\t date-> {} \n\t echo-> {}", date, echo,);
        }
    }
}

