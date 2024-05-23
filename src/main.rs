use std::{env, process};
use std::fs::File;
use tally_votes_exercise::{ContestList, Vote, count_votes};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Validate arguments.
    if args.len() != 3 {
        eprintln!("Missing input files");
        process::exit(1)
    }

    let contest_file_path = &args[1];
    let votes_file_path = &args[2];

    // Contest JSON.
    let contest_file = match File::open(contest_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open contest file {}: {}", contest_file_path, err);
            std::process::exit(1);
        }
    };

    // Votes JSON.
    let votes_file = match File::open(votes_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open votes file {}: {}", votes_file_path, err);
            process::exit(1);
        }
    };

    // Parse contests.
    let contest_list: ContestList = serde_json::from_reader(&contest_file).unwrap_or_else(|_error| {
        eprintln!("[ERROR] Could not read contest file {}", contest_file_path);
        process::exit(1)
    });

    // Parse votes.
    let votes: Vec<Vote> = serde_json::from_reader(&votes_file).unwrap_or_else(|_error| {
        eprintln!("[ERROR] Could not read votes file {}", votes_file_path);
        process::exit(1)
    });

    // Tally votes.
    let contest_results = count_votes(votes, contest_list);

    // Print JSON result.
    println!("{}", serde_json::to_string_pretty(&contest_results).unwrap());
}
