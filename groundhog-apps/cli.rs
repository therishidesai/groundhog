use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::BTreeMap;

use clap::{arg, Command};


fn cli() -> Command<'static> {
    Command::new("gh")
        .about("A tool to relive terrible days")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("bundle")
                .about("bundle up some traces")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Traces to bundle").allow_invalid_utf8(true)),
        )
		.subcommand(
            Command::new("replay")
                .about("Replay a bundle")
                .arg(arg!(<BUNDLE> "The bundle to replay"))
                .arg_required_else_help(true),
        )
}

struct Event {
	node: String,
	e_type: String,
	data: String,
}

fn bundle(traces: Vec<PathBuf>) {
	println!("Bundling {:?}", traces);
	let mut bundle: BTreeMap<String, Vec<Event>> = BTreeMap::new();

	for t in traces {
		let node_name = t.into_os_string().into_string().unwrap();
		let file = fs::read_to_string(node_name.clone()).unwrap();
		for l in file.lines() {
			let split = l.split(",");
			let split_vec: Vec<&str> = split.collect();
			match bundle.get_mut(split_vec[0]) {
				Some(v) => {
					v.push(Event {node: node_name.clone(), e_type: split_vec[1].to_string(), data: split_vec[2].to_string()});
				}
				None => {
					let mut v = Vec::new();
					v.push(Event {node: node_name.clone(), e_type: split_vec[1].to_string(), data: split_vec[2].to_string()});
					bundle.insert(split_vec[0].to_string(), v);
				} 
			}
		}
	}
	let _bundle_file = File::create("bundle.gh").unwrap();
	let mut bundle_file = File::options().append(true).open("bundle.gh").unwrap();
	
	for (time, events) in bundle.iter() {
		let mut time_str: String = String::new();
		for event in events {
			let s = format!("{} [{},{}], ", event.node, event.e_type, event.data);
			time_str.push_str(&s);
		}
		if let Err(e) = writeln!(bundle_file, "{}: {}", time, time_str) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}
}

fn replay(bundle: &str) {
	println!("Replaying ");
	let file = fs::read_to_string(bundle).unwrap();
	for l in file.lines() {
		println!("{}", l);
	}
}

fn main() {
	let matches = cli().get_matches();

	match matches.subcommand() {
        Some(("replay", sub_matches)) => {
            let bundle = sub_matches.value_of("BUNDLE").expect("required");
			replay(bundle);
        }
        Some(("bundle", sub_matches)) => {
            let paths = sub_matches
                .values_of_os("PATH")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
			bundle(paths);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
