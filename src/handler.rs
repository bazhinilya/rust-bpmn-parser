use regex::Regex;
use std::{collections::HashSet, env};

pub fn extract_unique_delegates<'a>(lines: impl Iterator<Item = &'a str>) -> HashSet<String> {
    let re = Regex::new(r#"class="([^"]*)"#).unwrap();
    lines
        .filter(|line| line.contains(&env::var("DELEGATE_PATTERN").unwrap()))
        .filter_map(|line| re.captures(line).map(|cap| cap[1].to_owned()))
        .collect()
}
