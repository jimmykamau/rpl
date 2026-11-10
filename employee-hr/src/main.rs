// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company; for example, “Add Sally to Engineering” or “Add
// Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people
// in the company by department, sorted alphabetically.

use std::{collections::HashMap, io};

fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    loop {
        println!();
        println!("Respond with the employee name and department, for instance");
        println!("Add Sally to Engineering");
        println!("Or");
        println!("Add Amir to Sales");
        println!("Reply with Done to go back to the main menu");

        let mut response: String = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Kindly enter a command");

        let trimmed = response.trim();
        if trimmed.is_empty() {
            return;
        }
        if trimmed == "Done" {
            println!();
            return;
        }

        let trimmed_parts: Vec<&str> = trimmed.split(' ').collect();
        if trimmed_parts.len() < 4 {
            println!("Malformed request");
            continue;
        }
        let employee_name: &str = trimmed_parts[1];
        let department_name: &str = trimmed_parts[3];
        let department = departments.entry(department_name.to_string()).or_default();
        department.push(String::from(employee_name));
        println!();
        println!("Added {employee_name} to {department_name}");
    }
}

fn list_department_employees(departments: &HashMap<String, Vec<String>>) {
    loop {
        println!();
        println!("Enter the department name");
        println!("Or Done to return to the main menu");

        let mut department_name = String::new();
        io::stdin()
            .read_line(&mut department_name)
            .expect("Kindly enter a department name");
        let trimmed = department_name.trim();
        if trimmed.is_empty() {
            return;
        }
        if trimmed == "Done" {
            println!();
            return;
        }
        let department = departments.get(trimmed);
        match department {
            Some(employees) => {
                println!();
                println!("Employees in {trimmed}:");
                let mut sorted = employees.clone();
                sorted.sort();
                for name in sorted {
                    println!("{name}");
                }
            }
            None => {
                println!("Department not found");
                continue;
            }
        }
    }
}

fn list_company_employees(departments: &HashMap<String, Vec<String>>) {
    println!();
    println!("Listing all employees per department");
    let mut department_names: Vec<_> = departments.keys().cloned().collect();
    department_names.sort();
    for dep_name in department_names {
        println!();
        println!("{dep_name}");
        let mut employee_names = departments[dep_name.as_str()].clone();
        employee_names.sort();
        for name in employee_names {
            println!("- {name}");
        }
    }
    println!();
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Choose an option:");
        println!("1. Retrieve a list of all people in the company ");
        println!("2. Retrieve a list of all people in a department");
        println!("3. Add employee name to a department");
        println!("Only reply with 1, 2 or 3");

        let mut chosen_option: String = String::new();
        let bytes = io::stdin()
            .read_line(&mut chosen_option)
            .expect("Kindly enter an option");
        if bytes == 0 {
            return;
        }
        let chosen_option: u8 = match chosen_option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match chosen_option {
            1 => list_company_employees(&departments),
            2 => list_department_employees(&departments),
            3 => add_employee(&mut departments),
            _ => {
                println!("Invalid choice");
                continue;
            }
        }
    }
}
