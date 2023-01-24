use std::{collections::HashMap, io};

struct Employee {
    name: String,
    department: String,
}

fn main() {
    let mut employees: HashMap<String, String> = HashMap::new();

    loop {
        println!("Enter 1 to add new employee");
        println!("Enter 2 to display all employess from a particular department");
        println!("Enter 3 to display employess from all departments");
        println!("Enter 4 to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input > 4 {
            println!("Invalid option");
            continue;
        } else if input == 1 {
            add_employee(&mut employees);
        } else if input == 2 {
            display_employees_department(&mut employees)
        } else if input == 3 {
            display_all_employees(&mut employees)
        } else if input == 4 {
            println!("Have a nice day!");
            break;
        }
    }
}

fn add_employee(employees: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    println!("Enter employee's name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Enter employee's department");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read department");

    employees.insert(name, department);
    return employees;
}

fn display_employees_department(employees: &mut HashMap<String, String>) {
    println!("Enter department");
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read department");

    let mut employees_by_dept: Vec<Employee> = Vec::new();
    for person in employees.keys() {
        let department = employees.get(person);
        match department {
            None => None,
            Some(department) => Some({
                let employee = Employee {
                    name: person.to_string(),
                    department: department.to_string(),
                };
                if department == &dept {
                    employees_by_dept.push(employee)
                }
            }),
        };
    }
    employees_by_dept.sort_by_key(|k| k.department.clone());
    for employee in employees_by_dept {
        print!("Name: {0}", employee.name,);
        print!("Department: {0}", employee.department);
        println!("")
    }
}

fn display_all_employees(employees: &mut HashMap<String, String>) {
    let mut employees_by_dept: Vec<Employee> = Vec::new();
    for person in employees.keys() {
        let department = employees.get(person);
        match department {
            None => None,
            Some(department) => Some({
                let employee = Employee {
                    name: person.to_string(),
                    department: department.to_string(),
                };
                employees_by_dept.push(employee)
            }),
        };
    }
    employees_by_dept.sort_by_key(|k| k.department.clone());
    for employee in employees_by_dept {
        print!("Name: {0}", employee.name,);
        print!("Department: {0}", employee.department);
        println!("")
    }
}
