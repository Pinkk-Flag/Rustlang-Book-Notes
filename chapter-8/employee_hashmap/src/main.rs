fn main() {
    println!(
        |====\n
        |\n
        |====\n
        |\n
        |====\n
    );

    println!("Welcome to hashbrown land! This is a program that allows companies to add employees or remove employees from certain sectors, and then allows them to view each sector. \n\nTo begin with, type `rm <NAME OF EMPLOYEE>` to remove an employee, and type `ins <NAME OF EMPLOYEE>` to add an employee. It will then prompt you for which sector to add it to. ");


    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

    The Approach:
    We start by giving the user two options:
    rm or ins (remove or insert). 
    We keep doing an infinite loop of asking the user rm or ins until they are satisfied and want to check the total list.
     */
    {
        use std::io;

        use std::collections::HashMap;
        employees_Sales = hashmap::new();
        employees_Engineering = hashmap::new();
        employees_Shipping = hashmap::new();
        main = hasmap::new();

        let mut original_input = String::new();
        while true {
            original_input = io::stdin().read_line(&mut original_input).expect("Failed to read input. Please try again!");

            let departments = ["Sales", "Engineering", "Shipping"];
            let length_departments = department.len();
            let mut count = 0;
            let selected_department;
            if checkOpener(original_input) == "insert" {
                println!("Which sector do you wish to wish to insert the employee?")
                for department in departments {
                    println!("To select {department} department number [{count}]");
                    count += 1;
                }



            } else if checkOpener(original_input) == "remove" {
                println!("Which sector do you wish to wish to remove the employee?")
                for department in departments {
                    println!("To select {department} department type [{count}]");
                    count += 1;
                }
            } else {

            }

        }
    }
}


fn checkDepartment(inputted_string:String) -> String {
    

}

fn checkOpener(input_command:String) -> String {
    if input_command[0..1] = "rm" {
        return "remove";
    } else if input_command[0..2] = "ins" {
        return "insert";
    }
    else {
        return "None";
    }
}

fn findName(inputted_string:String) -> String {
    let mut words: Vec<String> = inputted_string.split_whitespace().map(|string_input| string_input.to_string()).collect();
    let length: words.len();
    if length >= 2 {
        return words[1];
    } else {
        println!("An error occured! You did not input the name correctly. Please try again later.");
    }
}