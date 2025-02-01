use regex::Regex;
use std::{collections::HashSet, env};

pub fn extract_unique_delegates<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let re = Regex::new(r#"class="([^"]*)"#).unwrap();
    lines
        .filter(|line| line.contains(&env::var("DELEGATE_PATTERN").unwrap()))
        .filter_map(|line| re.captures(line).map(|cap| cap[1].to_owned()))
        .collect::<HashSet<_>>()
        .into_iter()
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

pub fn get_combined_variables<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let lines_vec: Vec<&str> = lines.collect();
    let mut result: Vec<String> = extract_context_variables(lines_vec.iter().cloned())
        .into_iter()
        .chain(extract_input_variables(lines_vec.iter().cloned()))
        .chain(extract_output_variables(lines_vec.iter().cloned()))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    result.sort();
    result
}

fn extract_input_variables<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let re = Regex::new(r"<[^>]*>").unwrap();
    lines
        .filter(|line| {
            line.contains("<camunda:inputParameter name=\"outputName\">")
                && line.contains("</camunda:inputParameter>")
                && !line.contains("${")
        })
        .map(|line| re.replace_all(line, "").trim().to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn extract_output_variables<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let name_re = Regex::new(r#"name="([^"]+)""#).unwrap();
    let var_re = Regex::new(r"\$\{([a-zA-Z0-9_]+)\}").unwrap();
    lines
        .filter(|line| {
            line.contains("<camunda:outputParameter name=\"") && !line.contains("${true}")
        })
        .map(|line| {
            var_re
                .captures(line)
                .and_then(|caps| caps.get(1))
                .or_else(|| name_re.captures(line).and_then(|caps| caps.get(1)))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn extract_context_variables<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    let variable_re = Regex::new(r#"(['"])(.*?)(['"])"#).unwrap();
    let mut variables = HashSet::new();
    let mut is_execution = false;

    let splitted_xml: Vec<String> = lines
        .map(|str| str.replace(char::is_whitespace, ""))
        .map(|str| {
            Regex::new(r"<.*?>")
                .unwrap()
                .replace_all(&str, "")
                .to_string()
        })
        .filter(|str| !str.trim().is_empty())
        .collect();

    for str in splitted_xml {
        if str.contains("execution.setVariable(") {
            if let Some(caps) = variable_re.captures(&str) {
                if let Some(var) = caps.get(2) {
                    variables.insert(var.as_str().to_string());
                }
            }
        }

        if is_execution || str.contains("execution.setVariables(") {
            is_execution = true;

            if let Some(caps) = variable_re.captures(&str) {
                if let Some(var) = caps.get(2) {
                    variables.insert(var.as_str().to_string());
                }
            }

            if str == ");"
                || str == ")"
                || str == "])"
                || str == "]);"
                || str.ends_with("])")
                || str.ends_with("]);")
            {
                is_execution = false;
            }
        }
    }

    variables.into_iter().collect()
}
