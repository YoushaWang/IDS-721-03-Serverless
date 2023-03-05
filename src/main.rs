extern crate csv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // Open the train.csv file
    let file = File::open("./data/train.csv")?;

    // Create a CSV reader and iterate over the records
    let reader = std::io::BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);
    let mut job_salaries: HashMap<String, (usize, f64)> = HashMap::new();

    for result in csv_reader.records(){
        let record = result?;

        let job_title = &record[2];
        let salary_num = record[6].parse::<f64>()?;
        // println!("Job Title: {}", job_title);
        // println!("Salary_in_num: {}", salary_num);

        // Update the average salary for the job title in the HashMap
        let (count, total_salary) = job_salaries.entry(job_title.to_string()).or_insert((0, 0.0));
        *count += 1;
        *total_salary += salary_num;
    }

    // Print the average salary for each job title
    for (job_title, (count, total_salary)) in &job_salaries {
        let average_salary = total_salary / (*count as f64);
        println!("Job Title: {}, Average Year Experience: {}", job_title, average_salary);
    }

    Ok(())
}
