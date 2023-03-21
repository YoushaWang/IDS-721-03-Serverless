use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use proj::average;
extern crate csv;
use std::collections::HashMap;
// use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder};
pub fn average(select: &str) -> f64 { //std::io::Result<f64>
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
    average_salary
    // average_salary / job_salaries.len() as f64
}
//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        "calculate average year experience a job need, please type CEO,CFO,VICE_PRESIDENT,JUNIOR")
}
//create a function that returns a hello world
#[get("/CEO")]
async fn CEO() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "calculate average year experience a job need: {}",
        average("CEO")
    ))
}
//create a function that returns a hello world
#[get("/CFO")]
async fn CFO() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "calculate average year experience a job need: {}",
        average("CFO")
    ))
}
//create a function that returns a hello world
#[get("/VICE_PRESIDENT")]
async fn VICE_PRESIDENT() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "calculate average year experience a job need: {}",
        average("VICE_PRESIDENT")
    ))
}
//create a function that returns a hello world
#[get("/JUNIOR")]
async fn JUNIOR() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "calculate average year experience a job need: {}",
        average("JUNIOR")
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(CEO).service(CFO).service(VICE_PRESIDENT).service(JUNIOR))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// fn main() -> Result<(), Box<dyn Error>> {
//     // Open the train.csv file
//     let file = File::open("./data/train.csv")?;

//     // Create a CSV reader and iterate over the records
//     let reader = std::io::BufReader::new(file);
//     let mut csv_reader = ReaderBuilder::new()
//         .has_headers(true)
//         .from_reader(reader);
//     let mut job_salaries: HashMap<String, (usize, f64)> = HashMap::new();

//     for result in csv_reader.records(){
//         let record = result?;

//         let job_title = &record[2];
//         let salary_num = record[6].parse::<f64>()?;
//         // println!("Job Title: {}", job_title);
//         // println!("Salary_in_num: {}", salary_num);

//         // Update the average salary for the job title in the HashMap
//         let (count, total_salary) = job_salaries.entry(job_title.to_string()).or_insert((0, 0.0));
//         *count += 1;
//         *total_salary += salary_num;
//     }

//     // Print the average salary for each job title
//     for (job_title, (count, total_salary)) in &job_salaries {
//         let average_salary = total_salary / (*count as f64);
//         println!("Job Title: {}, Average Year Experience: {}", job_title, average_salary);
//     }

//     Ok(())
// }