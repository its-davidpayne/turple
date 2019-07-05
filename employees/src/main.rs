// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, "Add Sally
// to Engineering" or "Add Amir to Sales". Then let the user retrieve a list
// of all people in a department or all people in the company by deprtment, 
// sorted alphabetically.

use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut employee_table: HashMap<String, Vec<String>> = HashMap::new();
    employee_table.insert(String::from("Sales"), vec![String::from("Aaron Andrews"), String::from("Danny Diller")]);
    employee_table.insert(String::from("IT"), vec![String::from("Billy Babbish"), String::from("Erin Euler")]);
    employee_table.insert(String::from("Engineering"), vec![String::from("Claire Cullen"), String::from("Fran Freud")]);

    if args.len() == 1 { // if no args supplied, assume user wants printout of entire table
        println!("{:#?}", employee_table);
    } else {
        println!("Hello");
    }
}

