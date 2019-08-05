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


extern crate clap;

use clap::{App, Arg};

fn main() {
    let mc = App::new("$name_camel_case$")
        .version("$version$")
        .author("$param.author_name$ <$param.author_email_lower_case$>")
        .about("$param.desc$")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set config file")
                .takes_value(true),
        )
        .get_matches();

    let config = mc.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
}
