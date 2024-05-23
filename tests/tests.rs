use std::fs::File;
use std::path::Path;
use std::process;
use tally_votes_exercise::{ContestList, Vote, count_votes};

/// Normal files.
#[test]
fn normal() {
    let contest_file_path = Path::new("tests").join("contest.json");
    let votes_file_path = Path::new("tests").join("votes.json");

    let contest_file = match File::open(&contest_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes_file = match File::open(&votes_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes: Vec<Vote> = serde_json::from_reader(&votes_file).unwrap();
    let contest_list: ContestList = serde_json::from_reader(&contest_file).unwrap();
    let contest_results = count_votes(votes, contest_list);
    assert_eq!(contest_results.len(), 1);
}

/// Empty contest list.
#[test]
fn empty() {
    let contest_file_path = Path::new("tests").join("contest_empty.json");
    let votes_file_path = Path::new("tests").join("votes.json");

    let contest_file = match File::open(&contest_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes_file = match File::open(&votes_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes: Vec<Vote> = serde_json::from_reader(&votes_file).unwrap();
    let contest_list: ContestList = serde_json::from_reader(&contest_file).unwrap();
    let contest_results = count_votes(votes, contest_list);
    assert_eq!(contest_results.len(), 0);
}

/// Some nonexistent contest and choice id in votes file.
#[test]
fn wrong_id() {
    let contest_file_path = Path::new("tests").join("contest.json");
    let votes_file_path = Path::new("tests").join("votes_wrong_id.json");

    let contest_file = match File::open(&contest_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes_file = match File::open(&votes_file_path) {
        Ok(file) => file,
        Err(_err) => {
            process::exit(1);
        }
    };

    let votes: Vec<Vote> = serde_json::from_reader(&votes_file).unwrap();
    let contest_list: ContestList = serde_json::from_reader(&contest_file).unwrap();
    let contest_results = count_votes(votes, contest_list);
    assert_eq!(contest_results.len(), 1);
}