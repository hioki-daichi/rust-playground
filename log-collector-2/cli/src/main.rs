use clap::arg_enum;
use clap::{App, AppSettings, Arg, SubCommand};
use reqwest::Client;
use std::io;

arg_enum! {
    #[derive(Debug)]
    enum Format {
        Csv,
        Json,
    }
}

fn main() {
    let client = Client::new();

    let server_arg = Arg::with_name("SERVER")
        .help("server url")
        .short("s")
        .long("server")
        .takes_value(true)
        .value_name("URL")
        .default_value("localhost:3000");

    let format_arg = Arg::with_name("FORMAT")
        .help("log format")
        .short("f")
        .long("format")
        .takes_value(true)
        .case_insensitive(true)
        .possible_values(&Format::variants());

    let get_subcommand = SubCommand::with_name("get")
        .about("get logs")
        .arg(format_arg);

    let post_subcommand = SubCommand::with_name("post").about("post logs, taking input from stdin");

    let opts = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(server_arg)
        .subcommand(get_subcommand)
        .subcommand(post_subcommand);

    let matches = opts.get_matches();

    let url: String = matches.value_of("SERVER").unwrap().into();

    match matches.subcommand() {
        ("get", sub_match) => {
            let format = sub_match
                .and_then(|m| m.value_of("FORMAT"))
                .map(|m| m.parse().unwrap())
                .unwrap();

            match format {
                Format::Csv => {
                    let out = io::stdout();
                    let mut out = out.lock();

                    client
                        .get(&format!("http://{}/csv", &url))
                        .send()
                        .unwrap()
                        .copy_to(&mut out)
                        .map(|_| ())
                        .expect("api request failed")
                }
                Format::Json => {
                    let res: api::logs::get::Response = client
                        .get(&format!("http://{}/logs", &url))
                        .send()
                        .unwrap()
                        .json()
                        .expect("api request failed");

                    println!("{}", serde_json::to_string(&res).unwrap());
                }
            }
        }
        ("post", _) => {
            for result_of_logs_as_post_request in
                csv::Reader::from_reader(io::stdin()).into_deserialize::<api::logs::post::Request>()
            {
                let logs_as_post_request = match result_of_logs_as_post_request {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("[WARN] failed to parse a line, skipping: {}", e);
                        continue;
                    }
                };

                client
                    .post(&format!("http://{}/logs", &url))
                    .json(&logs_as_post_request)
                    .send()
                    .map(|_| ())
                    .expect("api request failed")
            }
        }

        _ => unreachable!(),
    }
}
