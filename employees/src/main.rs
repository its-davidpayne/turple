// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, "Add Sally
// to Engineering" or "Add Amir to Sales". Then let the user retrieve a list
// of all people in a department or all people in the company by department, 
// sorted alphabetically.

use std::env;
use std::collections::HashMap;

fn build_table() -> HashMap<String, Vec<String>> {
    let mut employee_table: HashMap<String, Vec<String>> = HashMap::new();
    employee_table.insert(String::from("Sales"), vec![String::from("Aaron"), String::from("Danny")]);
    employee_table.insert(String::from("IT"), vec![String::from("Billy"), String::from("Erin")]);
    employee_table.insert(String::from("Engineering"), vec![String::from("Claire"), String::from("Fran")]);
    employee_table
}

fn process_instructions(instructions: &Vec<&str>, mut employee_table: &mut HashMap<String, Vec<String>>) {
    match instructions[0] {
        "add" => add_user(instructions[1].to_string(), instructions[3].to_string(), &mut employee_table),
        "remove" | "delete" => remove_user(instructions[1].to_string(), instructions[3].to_string(), &mut employee_table),
        "list" => list_employees_in_department(instructions[1].to_string(), &employee_table),
        _ => println!("Instruction not recognised"),
    };
}

fn add_user(username: String, department: String, table: &mut HashMap<String, Vec<String>>) {
    println!("Adding {} to {}", username, department);
    table.entry(department).or_insert(Vec::new()).push(username);
    
}

fn remove_user(username: String, department: String, table: &mut HashMap<String, Vec<String>>) {
    let no_match = vec![String::from("No match")];
    let old_list = match table.get(&department) {
        Some(x) => x,
        None => &no_match,
    };
    println!("Removing {} from {}", &username, &department);
    let new_list: Vec<String> = old_list.into_iter()
                                        .filter(|&v| v != &username)
                                        .map(|v| String::from(v))
                                        .collect();
    table.insert(department, new_list);
    // I suspect there'll be an easier way to do the above removing-value-from-HashMap operation
    // but I can't find it. At least this works.
}

fn list_employees_in_department(department: String, table: &HashMap<String, Vec<String>>) {
    let intermediate_list = table.get(&department);
    let no_match = vec![String::from("No match")];
    let final_list = match intermediate_list {
        Some(c) => c,
        None => &no_match,
    };
    println!("{} contains: {:#?}", department, final_list);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut employee_table = build_table();

    if args.len() == 1 { // if no args supplied, assume user wants printout of entire table
        println!("{:#?}", employee_table);
    } else {
        let instructions: Vec<&str> = args[1].split_whitespace().collect();
        process_instructions(&instructions, &mut employee_table);
        println!("Final table:");
        println!("{:#?}", employee_table);
    }
}

