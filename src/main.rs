// Copyright (C) $year$ $param.company_name$
// All Rights Reserved.
//
// NOTICE: All information contained herein is, and remains
// the property of $param.company_name$.
// The intellectual and technical concepts contained
// herein are proprietary to $param.company_name$
// and are protected by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from $param.company_name$.


use clap::Parser;
{{#if with_toml}}
use serde::Deserialize;
use std::{fs, io::ErrorKind, process::exit};
{{/if}}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "default.conf")]
    config: String,
}
{{#if with_toml}}
#[derive(Deserialize, Debug)]
struct Config {
    param: u32,
}
{{/if}}

fn main() {
    let args = Args::parse();
    println!("Value for config: {}", args.config);

    {{#if with_toml}}
    let config: Config = match fs::read_to_string(&args.config) {
        Ok(config) => toml::from_str(&config).unwrap(),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("`{}` not exists.", args.config);
                exit(2);
            } else {
                panic!("Error: {}", e);
            }
        },
    };
    println!("Value for param: {}", config.param);
    println!("Config: {:#?}", config);
    {{/if}}
}
