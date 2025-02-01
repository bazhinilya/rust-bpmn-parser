use regex::Regex;
use std::{collections::HashSet, env};

pub fn extract_unique_delegates<'a, I: Iterator<Item = &'a str>>(lines: I) -> Vec<String> {
    let re = Regex::new(r#"class="([^"]*)"#).unwrap();
    lines
        .filter(|line| {
            if let Ok(delegate_pattern) = env::var("DELEGATE_PATTERN") {
                line.contains(&delegate_pattern)
            } else {
                false
            }
        })
        .filter_map(|line| re.captures(line).map(|cap| cap[1].to_owned()))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn extract_user_task_attributes<'a, I: Iterator<Item = &'a str>>(
    lines: I,
) -> Vec<(String, String)> {
    let re = Regex::new(r#"<bpmn:userTask.*?id="([^"]+)"[^>]*name="([^"]*)"#).unwrap();
    let mut attrs = lines
        .flat_map(|line| re.captures_iter(line))
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect::<Vec<_>>();
    attrs.sort_by(|a, b| a.1.cmp(&b.1));
    attrs
}

pub fn get_combined_variables<'a>(orig_xml: &'a str) -> Vec<String> {
    let lines: Vec<&str> = orig_xml.lines().collect();

    let mut result = Vec::new();

    result.extend(extract_context_variables(&lines));
    result.extend(extract_input_variables(&lines));
    result.extend(extract_output_variables(&lines));

    result.sort();
    result.dedup();
    result
}

fn extract_input_variables<'a>(lines: &[&'a str]) -> Vec<String> {
    let re = Regex::new(r"<[^>]*>").unwrap();
    lines
        .iter()
        .filter_map(|line| {
            if line.contains("<camunda:inputParameter name=\"outputName\">")
                && line.contains("</camunda:inputParameter>")
                && !line.contains("${")
            {
                Some(re.replace_all(*line, "").trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn extract_output_variables<'a>(lines: &[&'a str]) -> Vec<String> {
    let name_re = Regex::new(r#"name="([^"]+)"#).unwrap();
    let var_re = Regex::new(r"\$\{([a-zA-Z0-9_]+)\}").unwrap();
    lines
        .iter()
        .filter_map(|line| {
            if line.contains("<camunda:outputParameter name=\"") && !line.contains("${true}") {
                var_re
                    .captures(line)
                    .and_then(|caps| caps.get(1))
                    .or_else(|| name_re.captures(line).and_then(|caps| caps.get(1)))
                    .map(|m| m.as_str().trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn remove_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    for c in input.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}

fn extract_variables_from_line(line: &str, variable_re: &Regex) -> Vec<String> {
    let mut variables = Vec::new();
    if let Some(capture) = variable_re.captures(line) {
        if let Some(var) = capture.get(1) {
            variables.push(var.as_str().to_string());
        }
    }
    variables
}

fn extract_context_variables(lines: &[&str]) -> Vec<String> {
    let variable_re = Regex::new(r#"['"](.*?)['"]"#).unwrap();
    let mut variables = Vec::new();
    let mut is_execution = false;
    let execution_set_variable = "execution.setVariable(";
    let execution_set_variables = "execution.setVariables(";
    let execution_endings = [");", ")", "])", "]);"];

    for line in lines {
        let cleaned_line = line.trim();
        if cleaned_line.is_empty() {
            continue;
        }

        let clean_line = remove_tags(cleaned_line);

        if clean_line.contains(execution_set_variable) {
            variables.extend(extract_variables_from_line(&clean_line, &variable_re));
        }

        if !is_execution && clean_line.contains(execution_set_variables) {
            is_execution = true;
            variables.extend(extract_variables_from_line(&clean_line, &variable_re));
        }

        if is_execution {
            if execution_endings
                .iter()
                .any(|&ending| clean_line.ends_with(ending))
            {
                is_execution = false;
            } else {
                variables.extend(extract_variables_from_line(&clean_line, &variable_re));
            }
        }
    }
    variables
}
