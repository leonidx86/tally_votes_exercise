use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/**
Represents available voting choice in a contest.

# Example data:
```text
{"id": 1, "text": "Rust"}
```
 */
#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub id: u64,
    pub text: String,
}

/// List of choices available in a contest.
#[derive(Serialize, Deserialize)]
pub struct ChoiceList(Vec<Choice>);

/**
Represents a contest.

# Example data:

```text
{
  "id": 1,
  "description": "Best Programming Language",
  "choices": [
	{"id": 1, "text": "Rust"},
	{"id": 2, "text": "Python"},
	{"id": 3, "text": "Go"}
  ]
}
```
 */
#[derive(Serialize, Deserialize)]
pub struct Contest {
    pub id: u64,
    pub description: String,
    pub choices: ChoiceList,
}

/// List of contests.
#[derive(Serialize, Deserialize)]
pub struct ContestList(Vec<Contest>);

/**
Represents a vote in voting data.

# Example data:

```text
{"contest_id": 1, "choice_id": 1}
```
 */
#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub contest_id: u64,
    pub choice_id: u64,
}


/**
Represents a summary of votes for a specific choice in a specific contest.

# Example data:

```text
{"choice_id": 1, "total_count": 2}
```
 */
#[derive(Serialize, Deserialize)]
pub struct ResultVote {
    pub choice_id: u64,
    pub total_count: u64,
}


/**
Represents a winner vote in a specific contest.

# Example data:

```text
{"choice_id": 1, "text": "Rust"}
```
 */
#[derive(Serialize, Deserialize)]
pub struct Winner {
    pub choice_id: u64,
    pub text: String,
}

/**
Represents result for all casted votes across all contests.

# Example data:

```text
{
  "contest_id": 1,
  "total_votes": 4,
  "results": [
	{"choice_id": 1, "total_count": 2},
	{"choice_id": 2, "total_count": 1},
	{"choice_id": 3, "total_count": 1}
  ],
  "winner": {"choice_id": 1, "text": "Rust"}
}
```
 */
#[derive(Serialize, Deserialize)]
pub struct ContestResult {
    pub contest_id: u64,
    pub total_votes: u64,
    pub results: Vec<ResultVote>,
    pub winner: Winner,
}

impl ContestList {
    /// Retrieve a contest by its id.
    pub fn get_by_id(&self, contest_id: u64) -> Option<&Contest> {
        for contest in &self.0 {
            if contest.id == contest_id {
                return Some(contest);
            }
        }
        None
    }
}

impl ChoiceList {
    /// Retrieve a contest choice by its id.
    pub fn get_by_id(&self, choice_id: u64) -> Option<&Choice> {
        for choice in &self.0 {
            if choice.id == choice_id {
                return Some(choice);
            }
        }
        None
    }
}

/**
Main logic of the program. Counting the votes taken from two input files: contest JSON and votes JSON.
There are check that the files are provided, and are not malformed.
Invalid votes are ignored, such as with nonexistent contest or choice id.
 */
pub fn count_votes(votes: Vec<Vote>, contest_list: ContestList) -> Vec<ContestResult> {
    // An intermediate variable to represent all the votes.
    // The external HashMap has contest id as the key, and votes as internal HashMap value.
    // The internal HashMap uses choice id as the key, and total votes as value.
    // Example representation of the value:
    // {1: {1: 2, 2: 1, 3: 1}}
    let mut votes_sum: HashMap<u64, HashMap<u64, u64>> = HashMap::new();

    // Count votes.
    for vote in votes {
        // Validate contest id and skip invalid votes.
        let contest = match contest_list.get_by_id(vote.contest_id) {
            Some(c) => c,
            None => {
                eprintln!("[ERROR] Invalid contest id {}", vote.contest_id);
                continue;
            }
        };

        // Validate choice id and skip invalid votes.
        let choice = match contest.choices.get_by_id(vote.choice_id) {
            Some(c) => c,
            None => {
                eprintln!("[ERROR] Invalid choice {} for contest {}", vote.choice_id, vote.contest_id);
                continue;
            }
        };

        // Get or create the contest key.
        let contest_record = votes_sum
            .entry(vote.contest_id)
            .or_insert(HashMap::new());

        // Get or create the choice key for the contest.
        let vote_record = contest_record
            .entry(choice.id) // vote.choice_id
            .or_insert(0);

        // Update total votes for the choice.
        *vote_record += 1;
    }

    // The return results value.
    let mut contest_results: Vec<ContestResult> = Vec::new();

    // Populate results.
    for (contest_id, votes) in votes_sum {
        // Already validated contests before.
        let contest = contest_list.get_by_id(contest_id).unwrap();

        // Total votes for a contest.
        let mut total_votes: u64 = 0;
        // The winner vote count.
        let mut max_vote: u64 = 0;
        // Contest winner choice id.
        let mut winner_choice_id: u64 = 0;
        // Results for a contest.
        let mut results: Vec<ResultVote> = Vec::new();

        for (choice_id, total_count) in votes {
            total_votes += total_count;
            if max_vote < total_count {
                max_vote = total_count;
                winner_choice_id = choice_id;
            }
            results.push(ResultVote { choice_id, total_count });
        }

        // Get the text of the winner and create winner object.
        let choice = contest.choices.get_by_id(winner_choice_id).unwrap();
        let winner = Winner { choice_id: winner_choice_id, text: choice.text.clone() };

        // Add contest result entry.
        contest_results.push(ContestResult {
            contest_id,
            total_votes,
            results,
            winner,
        });
    }

    contest_results
}
