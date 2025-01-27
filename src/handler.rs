use regex::Regex;
use std::{collections::HashSet, env};

pub fn extract_unique_delegates<'a>(lines: impl Iterator<Item = &'a str>) -> HashSet<String> {
    let re = Regex::new(r#"class="([^"]*)"#).unwrap();
    lines
        .filter(|line| line.contains(&env::var("DELEGATE_PATTERN").unwrap()))
        .filter_map(|line| re.captures(line).map(|cap| cap[1].to_owned()))
        .collect()
}

pub fn extract_user_task_attributes<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Vec<(String, String)> {
    let re = Regex::new(r#"<bpmn:userTask.*?id="([^"]+)"[^>]*name="([^"]*)"#).unwrap();
    let mut attrs = lines
        .flat_map(|line| re.captures_iter(line))
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect::<Vec<_>>();
    attrs.sort_by(|a, b| a.1.cmp(&b.1));
    attrs
}
