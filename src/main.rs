use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    act: String,
    components: String,
    summary: String,
    category: String,
    reasoning: String,
}

fn main() {
    // Read in apartheid law graphic organizer
    let input = fs::read_to_string("apartheid-law-graphic-organizer.md").unwrap();

    // Use regex to get all the points of points that we need
    let re = Regex::new(r"## +(.*)\n+- +(.*)\n- +(.*)\n- +(.*)\n- +(.*)\n").unwrap();

    // Create a vector to hold points
    let mut res: Vec<Point> = Vec::new();

    // Iterator through the capture groups of regex matching
    for caps in re.captures_iter(&input) {
        // Push it to the vector of points
        res.push(Point {
            act: caps.get(1).unwrap().as_str().to_string(),
            components: caps.get(2).unwrap().as_str().to_string(),
            summary: caps.get(3).unwrap().as_str().to_string(),
            category: caps.get(4).unwrap().as_str().to_string(),
            reasoning: caps.get(5).unwrap().as_str().to_string(),
        });
    }
    // Turn that into a vector of objects
    let out_json_object = serde_json::to_string(&res).unwrap();
    // Write to out.json
    fs::write("out.json", out_json_object).expect("error writing file");
}
