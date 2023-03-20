mod hr_cli;
mod pig_latin;
mod vector_methods;

use std::collections::HashMap;
use std::io;

use crate::hr_cli::read_prompt;
use crate::pig_latin::convert_to;
use crate::vector_methods::{median, mode};

fn main() {
    // Exercise #1
    println!("Running Exercise #1\n");

    let arr1 = vec![1,3,4,5,6,2,2,3,5];
    println!("Median: {}", median(&arr1));
    println!("Mode: {}", mode(&arr1));
    println!("{:?}", arr1);

    let arr2 = vec![1,1,2,2,3,3,4,5,6,2];
    println!("Median: {}", median(&arr2));
    println!("Mode: {}", mode(&arr2));
    println!("{:?}", arr2);

    // Exercise #2
    println!("\nRunning Exercise #2\n");
   
    let str1 = String::from("first");
    println!("'{str1}' in pig latin: {}", convert_to(&str1));

    let str2 = String::from("apple");
    println!("'{str2}' in pig latin: {}", convert_to(&str2));
    
    // Exercise #3
    println!("\nRunning Exercise #3\n");
    
    println!("Add employee: 'Add <name> to <department>'");
    println!("Get employee: 'Get employee <name>'");
    println!("Get department: 'Get department <department>'\n");
    
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    let mut i: u8 = 0;
    while i < 10 {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line!");

        if prompt.is_empty() || prompt == "\n" {
            break;
        }

        read_prompt(&prompt, &mut employees);
        
        i += 1;
    }

    println!("\nEnd");
}
