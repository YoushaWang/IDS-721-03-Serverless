use actix_web::{get, web::ServiceConfig};
use shuttle_service::ShuttleActixWeb;

// use proj::average;
extern crate csv;
use std::collections::HashMap;
// use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder};
pub fn average(select: &str) -> String { //std::io::Result<f64>
    let file = File::open("./data/train.csv").unwrap();
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut job_salaries: HashMap<String, (usize, f64)> = HashMap::new();
    for result in csv_reader.records() {
        let record = result.unwrap();
        let job_title = &record[2];
        let salary_num = record[6].parse::<f64>().unwrap();

        // Update the average salary for the job title in the HashMap
        let (count, total_salary) = job_salaries.entry(job_title.to_string()).or_insert((0, 0.0));
        *count += 1;
        *total_salary += salary_num;
    }

    // Print the average salary for each job title
    let mut average_salary = 0.0;
    for (job_title, (count, total_salary)) in &job_salaries {
        if job_title==select{
            let average_salary_for_job_title = total_salary / (*count as f64);
            println!("Job Title: {}, Average Salary: {}", job_title, average_salary_for_job_title);
            average_salary += average_salary_for_job_title;
        }
    }

    // Calculate and return the overall average salary
    // average_salary
    format!(
        "calculate average year experience a job need: {}",
        average_salary
    )
    // average_salary / job_salaries.len() as f64
}

#[get("/")]
async fn hello() -> &'static str {
    "calculate average year experience a job need, please type CEO,CFO,VICE_PRESIDENT,JUNIOR"
}

#[get("/CEO")]
async fn ceo() -> String {
    average("CEO")
}

#[get("/CFO")]
async fn cfo() -> String {
    average("CFO")
}
#[get("/VICE_PRESIDENT")]
async fn vice() -> String {
    average("VICE_PRESIDENT")
}

#[get("/JUNIOR")]
async fn junior() -> String {
    average("JUNIOR")
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(hello).service(ceo).service(cfo).service(vice).service(junior);
    })
}
