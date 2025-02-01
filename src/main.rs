mod excel;
mod handler;
mod util;

use std::{env, fs};

use excel::Excel;
use handler::{extract_unique_delegates, extract_user_task_attributes, get_combined_variables};
use util::get_latest_bpmn_file;

fn main() {
    dotenv::dotenv().ok();
    let inp_dir = env::var("INPUT_DIR").expect("INPUT_DIR must be set");
    let file_path = get_latest_bpmn_file(&inp_dir)
        .expect(format!("In the directory {} is missing .bpmn files", inp_dir).as_str());

    let orig_xml = fs::read_to_string(file_path).expect("Failed to read file");

    let uniq_delegates = extract_unique_delegates(orig_xml.lines());
    let uniq_user_attributes = extract_user_task_attributes(orig_xml.lines());
    let uniq_combined_variables = get_combined_variables(&orig_xml);

    let _ = Excel::new()
        .write_to_excel_single(uniq_delegates, "Уникальные делегаты")
        .write_to_excel(uniq_user_attributes, "Пользовательские задачи")
        .write_to_excel_single(uniq_combined_variables, "Контекстные переменные");
}
