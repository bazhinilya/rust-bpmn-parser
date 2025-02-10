mod excel;
mod handler;
mod util;

use std::time::Instant;

use excel::Excel;
use handler::read_file;
use util::get_latest_bpmn_file;

fn main() {
    let start = Instant::now();
    dotenv::dotenv().ok();
    let inp_dir = "./input";
    let file_path = get_latest_bpmn_file(&inp_dir)
        .expect(format!("In the directory {} is missing .bpmn files", inp_dir).as_str());

    let (uniq_delegates, uniq_context_variables, uniq_user_task_attributes) =
        read_file(&file_path).expect("Failed to get value");

    // println!("{:?}", uniq_delegates);
    // println!("{:?}", uniq_delegates.len());
    println!("=================================================");
    println!("{:?}", uniq_context_variables);
    println!("{:?}", uniq_context_variables.len());
    println!("=================================================");
    // println!("{:?}", uniq_user_task_attributes);
    // println!("{:?}", uniq_user_task_attributes.len());
    // let _ = Excel::new()
    //     .write_to_excel_single(uniq_delegates, "Уникальные делегаты")
    //     .write_to_excel_single(uniq_context_variables, "Контекстные переменные")
    //     .write_to_excel(uniq_user_task_attributes, "Пользовательские задачи");

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
