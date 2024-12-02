use std::{env, fs};

use inquire::{Select, Text};
use qams_core::{Criterion, Review};

const ARG_SCORECARD_PATH: usize = 1;

fn get_criterion_selection(criterion: &mut Criterion) {
    // get the criterion label and the labels of the criterion options (preserves indices)
    let label = criterion.label();
    let options: Vec<&str> = criterion
        .options()
        .iter()
        .map(|option| option.label())
        .collect();

    // get the selection from the user
    let result = Select::new(label, options).raw_prompt();

    match result {
        // handle success
        Ok(selection) => {
            criterion.set_selection_index(selection.index);
            // prompt to leave an optional comment
            match Text::new("Add a comment or hit Enter to skip: ").prompt_skippable() {
                // set comment
                Ok(Some(comment)) => {
                    criterion.set_comment(comment.as_str());
                }
                // no comment - do nothing
                Ok(None) => {}
                // handle error recursively
                Err(_) => {
                    println!("Error reading comment! Please try again.");
                    get_criterion_selection(criterion);
                }
            }
        }
        // handle error recursively
        Err(_) => {
            println!("Error getting selection! Please try again.");
            get_criterion_selection(criterion);
        }
    }
}

fn main() {
    // print program header and a few newlines to separate from review dialogue
    println!(
        "{} version={}\n\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // collect program arguments and extract scorecard path
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() > ARG_SCORECARD_PATH,
        "Not enough arguments supplied!"
    );
    let scorecard_path = args.get(ARG_SCORECARD_PATH).unwrap();

    // load scorecard and create the review
    let csv =
        fs::read_to_string(scorecard_path).expect("Failed to read scorecard from path provided!");
    let csv = csv.trim();
    let mut review = Review::from_csv(&csv);

    // get selections for each criterion
    for criterion in review.criteria_mut() {
        get_criterion_selection(criterion);
    }

    // print a few whitespaces to demonstrate the review is over
    print!("\n\n");
    // print the number of points rewarded to the review
    println!(
        "Total score: {} / {}",
        review.total_points(),
        review.max_points()
    );

    // get the output path and save the review as CSV.
    let output_path = Text::new("Path to output:")
        .prompt()
        .expect("Failed to read output path!");
    fs::write(output_path, review.to_csv()).expect("Failed to export to CSV!");
}
