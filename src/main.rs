#![allow(unused_variables)]
/*
 *   Copyright (c) 2022 
 *   All rights reserved.
 */


use std::io::{self,Write};
use std::process::exit;

struct Person 
{
    name:String,
    age : u8

}

impl Person
{
    pub fn new(p_name: String,p_age:u8) -> Person
    {
        Person {
            name:p_name,
            age : p_age
        }
    }

    pub fn get_name(&self) -> &str
    {
        self.name.as_str()
    }

    pub fn get_age(&self) -> u8
    {
        self.age
    }
}


fn main() {


    let mut input;
    let mut num_of_people;

    let mut pname;
    let mut age;

    let mut container:Vec<Person> = vec![]; 

    loop
    {

        input = (*read_input("Enter the number of peole you want to add ::>> "))
                .trim()
                .parse::<u8>();

        if input.is_err()
        {
            println!("Expects number! got {:?}",input);
            exit(1);
        }

        // otherwise

        num_of_people = input.unwrap();

        if num_of_people <= 0
        {
            continue;
        }
        
        for idx in 0..num_of_people
        {

            pname = *read_input("Enter person's name ::>> ");
            age   = (*read_input(format!("Enter {}'s age ::>> ",pname.trim()).as_str()))
                    .trim()
                    .parse::<u8>();
                   
            match age
            {
                Ok(ok) => {
                    container.push(Person::new(*(Box::new(pname.to_owned())),age.unwrap()))
                },
                Err(err) => {
                    println!("Expects number! got {}",err);
                    exit(1);
                }
            }
                
        }

        let mut eldest = &container[0];

        for i in 0..container.len()
        {
            for j in 0..container.len()
            {
                if container[j].get_age() > container[i].get_age()
                {
                    eldest = &container[j];
                }
            }
        }

        println!("{} is the eldest among them, at {} years old.",eldest.get_name().trim(),eldest.get_age());
        container.clear();
        
    }

}


pub fn read_input(message:&str) -> Box<String>
{
    let mut data:String = String::new();

    print!("{}",message);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut data)
    .expect("IoError: io error while reading");

    return Box::new(data.to_owned());
}