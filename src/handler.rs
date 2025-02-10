use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const DELEGATE_PREF: &str = "class=\"";
const USER_TASK_PAT: &str = "<bpmn:userTask id=\"";
const INPUT_START_PAT: &str = "<camunda:inputParameter name=\"outputName\">";
const INPUT_END_PAT: &str = "</camunda:inputParameter>";
const INPUT_EXC_PAT: &str = "${";
const OUTPUT_PAT: &str = "<camunda:outputParameter name=\"";
const OUTPUT_EX_PAT: &str = "${true}";
const SET_VAR_PAT: &str = "execution.setVariable(";
const SET_VARS_PAT: &str = "execution.setVariables(";

const NAME_PREF: &str = "name=\"";
const END_QUOTE: &str = "\"";

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

            if line.starts_with(&USER_TASK_PAT) {
                if let Some((id, name)) = extract_id_and_name(&line) {
                    uniq_user_task_attributes.insert((id.to_owned(), name.to_owned()));
                }
            } else if line.contains(&DELEGATE_PREF) {
                if let Some(value) = extract_class_name(&line) {
                    uniq_delegates.insert(value.to_owned());
                }
            } else if line.starts_with(INPUT_START_PAT)
                && line.ends_with(INPUT_END_PAT)
                && !line.contains(INPUT_EXC_PAT)
            {
                if let Some(value) = extract_input_variable(&line) {
                    uniq_context_variables.insert(value.to_owned());
                }
            } else if line.starts_with(OUTPUT_PAT) && !line.contains(OUTPUT_EX_PAT) {
                if line.contains(".") {
                    if let Some(value) = extract_output_key(&line) {
                        uniq_context_variables.insert(value.to_owned());
                    }
                } else {
                    if let Some(value) = extract_output_value(&line) {
                        uniq_context_variables.insert(value.to_owned());
                    }
                }
            } else if line.contains(SET_VAR_PAT) && !line.starts_with("//") {
                if let Some(value) = extract_set_variable(&line) {
                    uniq_context_variables.insert(value.to_owned());
                }
            } else if (is_execution || line.contains(SET_VARS_PAT)) && !line.starts_with("//") {
                let line_without_space: String =
                    line.chars().filter(|c| !c.is_whitespace()).collect();
                if line_without_space.contains(pat)
                is_execution = true;
                if let Some(value) = extract_set_variables(&line_without_space) {
                    uniq_context_variables.insert(value.to_owned());
                }
                if line_without_space.contains(")]")
                    || line_without_space.contains(")];")
                    || line_without_space.contains(");")
                    || line_without_space.starts_with(")")
                {
                    is_execution = false;
                }
            } else if line.starts_with(END_BPMN) {
                break;
            }
        }
    }
    Ok((
        uniq_delegates.into_iter().collect(),
        uniq_context_variables.into_iter().collect(),
        uniq_user_task_attributes.into_iter().collect(),
    ))
}

fn extract_class_name(input: &str) -> Option<&str> {
    let (_, class_value_part) = input.split_once(DELEGATE_PREF)?;
    let (class_name, _) = class_value_part.split_once(END_QUOTE)?;
    Some(class_name)
}

fn extract_id_and_name(input: &str) -> Option<(&str, &str)> {
    let (_, id_part) = input.split_once(USER_TASK_PAT)?;
    let (id, _) = id_part.split_once(END_QUOTE)?;

    let (_, name_part) = input.split_once(NAME_PREF)?;
    let (name, _) = name_part.split_once(END_QUOTE)?;

    Some((id, name))
}

fn extract_input_variable(input: &str) -> Option<&str> {
    let (_, start_part) = input.split_once(INPUT_START_PAT)?;
    let (user_info, _) = start_part.split_once(INPUT_END_PAT)?;
    Some(user_info)
}

fn extract_output_key(input: &str) -> Option<&str> {
    let (_, start_part) = input.split_once(OUTPUT_PAT)?;
    let (user_info, _) = start_part.split_once(END_QUOTE)?;
    Some(user_info)
}

fn extract_output_value(input: &str) -> Option<&str> {
    let start_marker = "${";
    let end_marker = "}";
    let (_, start_part) = input.split_once(start_marker)?;
    let (user_info, _) = start_part.split_once(end_marker)?;
    Some(user_info)
}

fn extract_set_variable(inp: &str) -> Option<&str> {
    let (_, start) = inp.split_once(SET_VAR_PAT)?;
    let end = if start.starts_with("'") { "'" } else { "\"" };
    let (user_info, _) = start.split_once(end)?;
    Some(user_info)
}

//TODO: Проверка
// execution.setVariables(
//     "owners": owners
//     );

//     def contractNumber = ((new Random()).nextInt(100) + 1) + ""
//     def contractConclusionDate = new Date()
//     def contractConclusionDateStr = contractConclusionDate.format("dd.MM.yyyy")

//     execution.setVariables(
//     "contractNumber": contractNumber,
//     "contractConclusionDate": contractConclusionDate,
//     "contractConclusionDateStr": contractConclusionDateStr
//     );</bpmn:script>
fn extract_set_variables(inp: &str) -> Option<&str> {
    let marker = if inp.starts_with("'") { "'" } else { "\"" };
    let (_, start_part) = inp.split_once(marker)?;
    let (variable, _) = start_part.split_once(marker)?;
    Some(variable)
}
