use crate::employee::Employee;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

pub fn cli() {
    println!("Employee CLI 1.0");
    let mut employees: Vec<Employee> = vec![];
    loop {
        let mut input: String = String::new();
        read(
            "1. CREATE, 2. UPDATE, 3. SEARCH, 4. REMOVE, 5. LIST or any other number to EXIT",
            &mut input,
        );
        let input: i8 = input.trim().parse().unwrap_or_default();

        match input {
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
    let (mut name, mut phone, mut email, mut rate) =
        (String::new(), String::new(), String::new(), String::new());

    read("Enter new employee name: ", &mut name);
    read("Enter new employee phone: ", &mut phone);
    read("Enter new employee email: ", &mut email);
    read("Enter new employee rate: ", &mut rate);

    Employee {
        name: name.trim().parse().unwrap_or_default(),
        phone: phone.trim().parse().unwrap_or_default(),
        email: email.trim().parse().unwrap_or_default(),
        rate: rate.trim().parse().unwrap_or_default(),
    }
}

fn update_employee(employees: &mut Vec<Employee>) {
    let (mut input, mut flag) = (String::new(), false);

    read("Enter employee details", &mut input);
    let input: String = input.trim().parse().unwrap_or_default();

    for i in 0..employees.len() {
        if employees[i].get_name() == input {
            flag = true;
            employees[i] = create_employee();
        }
    }
    if flag {
        list_employees(employees);
    } else {
        println!("Error! Couldn't find employee")
    }
}

fn search_employee(employees: &mut Vec<Employee>) {
    let mut input: String = String::new();

    read("Enter employee details to search", &mut input);
    let input: String = input.trim().parse().unwrap_or_default();

    for employee in employees {
        if input == employee.get_name() {
            println!("found employee: {}", employee.get_string());
            return;
        }
    }
    println!("No employee found!");
}

fn remove_employee(employees: &mut Vec<Employee>) {
    let mut input: String = String::new();

    read("Enter details of employee to be deleted", &mut input);
    let input: String = input.trim().parse().unwrap_or_default();

    employees.retain(|employee| employee.get_name() != input);
    list_employees(employees);
}

fn list_employees(employees: &Vec<Employee>) {
    for employee in employees {
        println!("{}", employee.get_string());
    }
}

fn read(msg: &str, input: &mut String) {
    println!("{}", msg);
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}
