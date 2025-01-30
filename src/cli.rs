use crate::employee::Employee;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

// messages
const TITLE: &str = "Employee CLI 1.0";
const ACTION_MSG: &str =
    "1. CREATE, 2. UPDATE, 3. SEARCH, 4. REMOVE, 5. LIST any other key to EXIT";
const EMP_NAME: &str = "employee name";
const EMP_PHONE: &str = "employee phone";
const EMP_EMAIL: &str = "employee email";
const EMP_RATE: &str = "employee rate";
const EMP_DETAILS: &str = "employee details";
const EMP_404: &str = "employee not found";
const READ_ERR: &str = "failed to read";
const FLUSH_ERR: &str = "failed to flush";

pub fn cli() {
    println!("{TITLE}");
    let mut employees: Vec<Employee> = vec![];
    loop {
        match read(ACTION_MSG).trim().parse().unwrap_or_default() {
            1 => employees.push(create_employee()),
            2 => update_employee(&mut employees),
            3 => search_employee(&mut employees),
            4 => remove_employee(&mut employees),
            5 => list_employees(&employees),
            _ => exit(0),
        }
    }
}

fn create_employee() -> Employee {
    Employee {
        name: read(EMP_NAME).trim().to_string(),
        phone: read(EMP_PHONE).trim().to_string(),
        email: read(EMP_EMAIL).trim().to_string(),
        rate: read(EMP_RATE).trim().parse().unwrap_or_default(),
    }
}

fn update_employee(employees: &mut Vec<Employee>) {
    let mut flag = false;
    let input = read(EMP_DETAILS).trim().to_string();
    for i in 0..employees.len() {
        if employees[i].name == input {
            flag = true;
            employees[i] = create_employee();
        }
    }
    if flag {
        list_employees(employees);
    } else {
        println!("{EMP_404}")
    }
}

fn search_employee(employees: &mut Vec<Employee>) {
    let input = read(EMP_DETAILS).trim().to_string();

    for employee in employees {
        if input == employee.name {
            println!("{employee}");
            return;
        }
    }
    println!("{EMP_404}")
}

fn remove_employee(employees: &mut Vec<Employee>) {
    let input = read(EMP_DETAILS).trim().to_string();
    employees.retain(|employee| employee.name != input);
    list_employees(employees);
}

fn list_employees(employees: &Vec<Employee>) {
    for employee in employees {
        println!("{employee}");
    }
}

fn read(msg: &str) -> String {
    println!("{msg}: ");
    let mut input = String::new();
    stdout().flush().expect(FLUSH_ERR);
    stdin().read_line(&mut input).expect(READ_ERR);
    input
}
