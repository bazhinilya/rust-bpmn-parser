use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const DELEGATE: &str = "class=\"";
const USER_TASK: &str = "<bpmn:userTask id=\"";
const INP_START: &str = "<camunda:inputParameter name=\"outputName\">";
const INP_END: &str = "</camunda:inputParameter>";
const START_SHIELD: &str = "${";
const END_SHIELD: &str = "}";
const OUT: &str = "<camunda:outputParameter name=\"";
const OUT_EX: &str = "${true}";
const OUT_VALUE: &str = ".";
const SET_VAR_1: &str = "execution.setVariable(\"";
const SET_VAR_2: &str = "execution.setVariable('";
const SET_VARS: &str = "execution.setVariables(";
const SET_VARS_1: &str = "execution.setVariables(\"";
const SET_VARS_2: &str = "execution.setVariables('";
const SET_VARS_END_1: &str = "])";
const SET_VARS_END_2: &str = ")";
const NAME_PREF: &str = "name=\"";
const DOUBLE_QUOTE: &str = "\"";
const ONE_QUOTE: &str = "'";
const COMMENTS: &str = "//";
const END_BPMN: &str = "<bpmndi:BPMNDiagram";

pub fn read_file(
    path: &PathBuf,
) -> Result<(Vec<String>, Vec<String>, Vec<(String, String)>), Box<dyn Error>> {
    let mut uniq_user_task_attributes: HashSet<(String, String)> = HashSet::new();
    let mut uniq_delegates: HashSet<String> = HashSet::new();
    let mut uniq_context_variables: HashSet<String> = HashSet::new();
    let mut is_execution = false;

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;
            let line = line.trim_start();

            if line.starts_with(USER_TASK) {
                if let Some((id, name)) = extract_id_and_name(&line) {
                    uniq_user_task_attributes.insert((id.to_owned(), name.to_owned()));
                }
            } else if line.contains(DELEGATE) {
                if let Some(value) = extract_var(line, DELEGATE, DOUBLE_QUOTE) {
                    uniq_delegates.insert(value.to_owned());
                }
            } else if line.starts_with(INP_START)
                && line.ends_with(INP_END)
                && !line.contains(START_SHIELD)
            {
                if let Some(value) = extract_var(line, INP_START, INP_END) {
                    uniq_context_variables.insert(value.to_owned());
                }
            } else if line.starts_with(OUT) && !line.contains(OUT_EX) {
                if line.contains(OUT_VALUE) {
                    if let Some(value) = extract_var(line, OUT, DOUBLE_QUOTE) {
                        uniq_context_variables.insert(value.to_owned());
                    }
                } else {
                    if let Some(value) = extract_var(line, START_SHIELD, END_SHIELD) {
                        uniq_context_variables.insert(value.to_owned());
                    }
                }
            } else if line.contains(SET_VAR_1) && !line.starts_with(COMMENTS) {
                if let Some(value) = extract_var(line, SET_VAR_1, DOUBLE_QUOTE) {
                    uniq_context_variables.insert(value.to_owned());
                }
            } else if line.contains(SET_VAR_2) && !line.starts_with(COMMENTS) {
                if let Some(value) = extract_var(line, SET_VAR_2, ONE_QUOTE) {
                    uniq_context_variables.insert(value.to_owned());
                }
            } else if line.contains(SET_VARS) {
                if let Some(value) = extract_var(line, SET_VARS_1, DOUBLE_QUOTE) {
                    uniq_context_variables.insert(value.to_owned());
                }
                if let Some(value) = extract_var(line, SET_VARS_2, ONE_QUOTE) {
                    uniq_context_variables.insert(value.to_owned());
                }
                is_execution = !line.ends_with(SET_VARS_END_1);
            } else if is_execution {
                let marker = if line.starts_with(DOUBLE_QUOTE) {
                    DOUBLE_QUOTE
                } else {
                    ONE_QUOTE
                };
                if let Some(value) = extract_var(line, marker, marker) {
                    uniq_context_variables.insert(value.to_owned());
                }
                if line.starts_with(SET_VARS_END_1)
                    || line.ends_with(SET_VARS_END_1)
                    || line.starts_with(SET_VARS_END_2)
                {
                    is_execution = false;
                }
            } else if line.starts_with(END_BPMN) {
                break;
            }
        }
    }
    let mut sorted_delegates: Vec<String> = uniq_delegates.drain().collect();
    sorted_delegates.sort();
    let mut sorted_context_variables: Vec<String> = uniq_context_variables.drain().collect();
    sorted_context_variables.sort();
    let mut sorted_user_task_attributes: Vec<(String, String)> =
        uniq_user_task_attributes.drain().collect();
    sorted_user_task_attributes.sort_by(|a, b| a.1.cmp(&b.1));
    Ok((
        sorted_delegates,
        sorted_context_variables,
        sorted_user_task_attributes,
    ))
}

fn extract_id_and_name(input: &str) -> Option<(&str, &str)> {
    let (_, id_part) = input.split_once(USER_TASK)?;
    let (id, _) = id_part.split_once(DOUBLE_QUOTE)?;

    let (_, name_part) = input.split_once(NAME_PREF)?;
    let (name, _) = name_part.split_once(DOUBLE_QUOTE)?;

    Some((id, name))
}

fn extract_var<'a>(inp: &'a str, start_pref: &'a str, end_pref: &'a str) -> Option<&'a str> {
    let (_, start) = inp.split_once(start_pref)?;
    let (res, _) = start.split_once(end_pref)?;
    Some(res)
}
