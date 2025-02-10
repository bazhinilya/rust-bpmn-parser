mod excel;
mod handler;
mod util;

use std::time::Instant;

use chrono::Local;
use excel::Excel;
use handler::read_file;
use util::get_latest_bpmn_file;

fn main() {
    let start = Instant::now();
    let inp_dir = "./input";
    let file_path = get_latest_bpmn_file(&inp_dir)
        .expect(format!("In the directory {} is missing .bpmn files", inp_dir).as_str());

    let (uniq_delegates, uniq_context_variables, uniq_user_task_attributes) =
        read_file(&file_path).expect("Failed to get value");

    let date_time = Local::now().format("%H%M%S_%d%m%y").to_string();

    let _ = Excel::new()
        .write_to_excel_single(uniq_delegates, "Уникальные делегаты", &date_time)
        .write_to_excel_single(uniq_context_variables, "Контекстные переменные", &date_time)
        .write_to_excel(
            uniq_user_task_attributes,
            "Пользовательские задачи",
            &date_time,
        );

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
