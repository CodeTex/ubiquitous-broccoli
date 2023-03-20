use std::collections::HashMap;

pub fn read_prompt(prompt: &String, departments: &mut HashMap<String, Vec<String>>) {
    let words: Vec<&str> = prompt.split_whitespace().collect();

    match words[0].to_lowercase().as_str() {
        "add" => add_employee(&words[1], &words[3], departments),
        "get" => {
            match words[1].to_lowercase().as_str() {
                "department" => get_department(&words[2], departments),
                "employee" => get_employee(&words[2], departments),
                _ => {
                    println!("'get' method only supports 'department' and 'employee'.");
                    return
                },
            }
        },
        _ => {
            println!("Only 'add' and 'get' method are defined.");
            return
        },
    }
}

fn add_employee(name: &str, department: &str, departments: &mut HashMap<String, Vec<String>>) {
    departments.entry(department.to_owned()).or_insert(Vec::new()).push(name.to_owned());
    println!("Added {name} to {department}");
}

fn get_department(department: &str, departments: &mut HashMap<String, Vec<String>>) {
    let mut employees = Vec::new();

    match departments.get(&department.to_owned()) {
        Some(val) => employees = val.to_vec(),
        None => {},
    };
    println!("Employees @ {department}: {:?}", employees);
}

fn get_employee(name: &str, departments: &mut HashMap<String, Vec<String>>) {
    for (department, employees) in departments {
        if employees.iter().any(|x| x == name) {
            println!("{name} works @ {department}");
            return
        }
    }
    println!("{name} has no workplace ");
}

