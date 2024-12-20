mod service;
mod args;

use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{arg, Command, builder::styling, Arg};
use cloudevents::{AttributesReader, Data};
use cloudevents::binding::reqwest::RequestBuilderExt;
use config::Map;
use log::debug;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

// =============================
// ========= Cli Setup =========
// =============================
fn cli() -> Command {
    const STYLES: styling::Styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default().bold())
        .usage(styling::AnsiColor::Green.on_default().bold())
        .literal(styling::AnsiColor::Blue.on_default().bold())
        .placeholder(styling::AnsiColor::Cyan.on_default());

    let default_args: [Arg; 2] = [
        arg!(-i --id <ID> "The CloudEvent ID of the event").required(true),
        arg!(-s --source <SOURCE> "The source of the event").required(true),
    ];

    Command::new("cdevent")
        .about("A CLI for sending CD events")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .color(clap::ColorChoice::Auto)
        .styles(STYLES)
        .args([
            arg!(-e --endpoint <URL> "The endpoint to send events to").required(true),
            arg!(-q --quiet "Suppress output"),
            arg!(-c --config <FILE> "The configuration to use for the event"),
            arg!(-o --output <FILE> "The file to write the event to"),
            arg!(-f --format <FORMAT> "The format to write the event in"),
            arg!(-v --verbose "Increase verbosity"),
            arg!(-H --headers <HEADERS> ... "Headers to include in the request").value_parser(parse_headers),
        ])
        // .subcommand(
        //     Command::new("artifact")
        //         .about("An artifact produced by a build")
        //         .arg(arg!(<REMOTE> "The remote to clone"))
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("branch")
        //         .about("A branch in a software configuration management (SCM)repository")
        //         .arg(arg!(base: [COMMIT]))
        //         .arg(arg!(head: [COMMIT]))
        //         .arg(arg!(path: [PATH]).last(true))
        //         .arg(
        //             arg!(--color <WHEN>)
        //                 .value_parser(["always", "auto", "never"])
        //                 .num_args(0..=1)
        //                 .require_equals(true)
        //                 .default_value("auto")
        //                 .default_missing_value("always"),
        //         ),
        // )
        // .subcommand(
        //     Command::new("build")
        //         .about("A software build")
        //         .arg(arg!(<REMOTE> "The remote to target"))
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("change")
        //         .about("A change proposed to the content of a repository")
        //         .arg_required_else_help(true)
        //         .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        // )
        // .subcommand(
        //     Command::new("environment")
        //         .about("An environment where to run services")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("incident")
        //         .about("A problem in a production environment")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("pipelinerun")
        //         .about("An instance of a pipeline")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("repository")
        //         .about("A software configuration management (SCM)repository")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        .subcommand(
            Command::new("service")
                .about("A service running software in an environment")
                .args_conflicts_with_subcommands(true)
                .flatten_help(true)
                .subcommand_required(true)
                .subcommand(
                    Command::new("deployed")
                        .args(default_args)
                        .args(service::deployed_args()))
                // .subcommand(Command::new("published").arg(arg!([STASH])))
                // .subcommand(Command::new("removed").arg(arg!([STASH])))
                // .subcommand(Command::new("rolledback").arg(arg!([STASH])))
                // .subcommand(Command::new("upgraded").args(push_args()))
        )
        // .subcommand(
        //     Command::new("taskrun")
        //         .about("An instance of a task")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("testcaserun")
        //         .about("The execution of a software testCase")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("testoutput")
        //         .about("The execution of a software testSuite")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
        // .subcommand(
        //     Command::new("testsuiterun")
        //         .about("The execution of a software testSuite")
        //         .args_conflicts_with_subcommands(true)
        //         .flatten_help(true)
        //         .args(push_args())
        //         .subcommand(Command::new("push").args(push_args()))
        //         .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
    // missing ticket - A ticket in a ticketing system

}

fn push_args() -> Vec<clap::Arg> {
    vec![
        arg!(--id <ID> "The CloudEvent ID of the event").required(true),
        arg!(-s --source <SOURCE> "The source of the event").required(true),
    ]
}

// ========================
// ========= Main =========
// ========================
#[tokio::main]
async fn main() -> Result<ExitCode, ExitCode> {
    let matches = cli().get_matches();

    let endpoint = matches.get_one::<String>("endpoint");
    let empty_header: Vec<(String,String)> = Vec::new();
    let headers:HashMap<String,String> = matches.try_get_one::<Vec<(String,String)>>("headers")
        .unwrap()
        .map(|c| c.into_iter().map(move |t| { let x = t.clone(); (x.0, x.1)}).collect())
        .unwrap_or(HashMap::new());
    
    match matches.subcommand() {
        // Some(("clone", sub_matches)) => {
        //     println!(
        //         "Cloning {}",
        //         sub_matches.get_one::<String>("REMOTE").expect("required")
        //     );
        // }
        // Some(("diff", sub_matches)) => {
        //     let color = sub_matches
        //         .get_one::<String>("color")
        //         .map(|s| s.as_str())
        //         .expect("defaulted in clap");
        //
        //     let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
        //     let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
        //     let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
        //     if path.is_none() {
        //         path = head;
        //         head = None;
        //         if path.is_none() {
        //             path = base;
        //             base = None;
        //         }
        //     }
        //     let base = base.unwrap_or("stage");
        //     let head = head.unwrap_or("worktree");
        //     let path = path.unwrap_or("");
        //     println!("Diffing {base}..{head} {path} (color={color})");
        // }
        // Some(("push", sub_matches)) => {
        //     println!(
        //         "Pushing to {}",
        //         sub_matches.get_one::<String>("REMOTE").expect("required")
        //     );
        // }
        // Some(("add", sub_matches)) => {
        //     let paths = sub_matches
        //         .get_many::<PathBuf>("PATH")
        //         .into_iter()
        //         .flatten()
        //         .collect::<Vec<_>>();
        //     println!("Adding {paths:?}");
        // }
        Some(("service", sub_matches)) => {
            let service_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));
            match service_command {
                ("deployed", sub_matches) => {
                    let args = service::deployed_parse(sub_matches);
                    // let cd_event: CDEvent = CDEvent::from(args.clone());
                    let cloud_event = service::to_cloud_event(&args);
                    // let custom_data = get_custom_data(&cloud_event);
                    // match custom_data {
                    //     Some(data) => {
                    //         println!("Event {}: Deployed service {} to environment {} with custom data count {}", &cloud_event.id(), &cloud_event.subject().unwrap(), args.env_id, data.iter().count());
                    //     }
                    //     None => {
                    //         println!("Event {}: Deployed service {} to environment {} with no custom data", &cloud_event.id(), &cloud_event.subject().unwrap(), args.env_id);
                    //     }
                    // }
                    let id = cloud_event.id();
                    let sub = cloud_event.subject().unwrap();
                    println!("Posting to endpoint: {endpoint:?}, id: {id:?}, subject: {sub:?}");
                    let httpHeaders: HeaderMap = HeaderMap::from_iter(headers.iter().map(|(k,v)| (HeaderName::try_from(k.as_str()).unwrap(), HeaderValue::from_str(v.as_str()).unwrap())));
                    let response = reqwest::Client::new()
                        .post(endpoint.unwrap())
                        .event(cloud_event)
                        .map_err(|e| ExitCode::FAILURE)?
                        .header("Access-Control-Allow-Origin", "*")
                        .headers(httpHeaders)
                        .send()
                        .await
                        .map_err(|e| ExitCode::FAILURE)?;

                    println!("Response: {:?}", response);
                }
                ("created", sub_matches) => {
                    println!("Created");
                }
                ("modified", sub_matches) => {
                    println!("Modified");
                }
                (name, sub) => {
                    panic!("Unknown subcommand: {}", name);
                }
            }
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here..
    // return success code
    Ok(ExitCode::SUCCESS)
}

fn get_custom_data(event: &cloudevents::Event) -> Option<HashMap<String, String>> {
    debug!("Event: {:?}", event);
    event.data().and_then(|data| {
        if let Data::Json(json) = data {
            println!("data in event {}", json);
            match json {
                serde_json::Value::Object(map) => {
                    // get "customData" if it exists
                    match map.get("customData") {
                        Some(custom_data) => {
                            match custom_data {
                                serde_json::Value::Object(custom_map) => {
                                    let custom_data:HashMap<String,String> = custom_map.iter().map(|(k,v)| (k.clone(), v.as_str().unwrap().to_string())).collect();
                                    Some(custom_data)
                                }
                                _ => {
                                    println!("customData in event is not an object");
                                    None
                                }
                            }
                        }
                        None => {
                            println!("customData not found in event");
                            None
                        }
                    }
                }
                _ => Some(HashMap::<String,String>::new())
            }
        } else {
            println!("data in event {}", data);
            None
        }
    })
}

fn parse_headers(s: &str) -> Result<Vec<(String, String)>, Box<dyn Error + Send + Sync + 'static>>
{
    args::parse_comma_delimited(s, "No headers provided")
}