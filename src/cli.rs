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
            1 => employees.push(create()),
            2 => println!("{}", update(&mut employees)),
            3 => println!("{}", search(&mut employees)),
            4 => println!("{}", remove(&mut employees)),
            5 => println!("{}", list(&employees)),
            _ => exit(0),
        }
    }
}

fn create() -> Employee {
    Employee {
        name: read(EMP_NAME).trim().to_string(),
        phone: read(EMP_PHONE).trim().to_string(),
        email: read(EMP_EMAIL).trim().to_string(),
        rate: read(EMP_RATE).trim().parse().unwrap_or_default(),
    }
}

fn update(employees: &mut Vec<Employee>) -> String {
    let mut flag = false;
    let input = read(EMP_DETAILS).trim().to_string();
    for i in 0..employees.len() {
        if employees[i].name == input {
            flag = true;
            employees[i] = create();
        }
    }
    if flag {
        list(employees)
    } else {
        EMP_404.to_string()
    }
}

fn search(employees: &mut Vec<Employee>) -> String {
    let input = read(EMP_DETAILS).trim().to_string();
    for employee in employees {
        if input == employee.name {
            return employee.to_string();
        }
    }
    EMP_404.to_string()
}

fn remove(employees: &mut Vec<Employee>) -> String {
    let input = read(EMP_DETAILS).trim().to_string();
    employees.retain(|employee| employee.name != input);
    list(employees)
}

fn list(employees: &Vec<Employee>) -> String {
    let mut list = String::new();
    for employee in employees {
        list += &employee.to_string();
    }
    list
}

fn read(msg: &str) -> String {
    println!("{msg}: ");
    let mut input = String::new();
    stdout().flush().expect(FLUSH_ERR);
    stdin().read_line(&mut input).expect(READ_ERR);
    input
}
