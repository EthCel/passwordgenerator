use passwords::PasswordGenerator;
use passwords::hasher;
//use passwords::analyzer;
use diesel::insert_into;
use diesel::prelude::*;

use std::io;

pub mod models;
pub mod schema;
pub mod database;

use crate::schema::users::dsl::users;

fn main() {
    println!("Password Manager");
    println!("Press (1) and enter to create a password");
    println!("Press (2) and enter to analyze a password");
    println!("Press (3) and enter to do something else");

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Invalid input");

    match input.trim() {
        "1" => {
            println!("How many passwords?");
            let mut password_batch = String::new();
            io::stdin().read_line(&mut password_batch).expect("Invalid input");
            let password_batch: u32 = password_batch.trim().parse().expect("Invalid input");

            println!("Specify parameters");
            println!("Password length");
            let mut length = String::new();
            io::stdin().read_line(&mut length).expect("Invalid input");
            let length: u32 = length.trim().parse().expect("Invalid input");

            
            println!("Uses symbols? (y/n)");
            let mut symbols = String::new();
            io::stdin().read_line(&mut symbols).expect("Invalid input");
            let mut symbols = match symbols.trim() {
                "y" => true,
                "n" => false,
                _   => panic!("Invalid input"),
            };

            if password_batch >= 2 {
                generate_multiple(length.try_into().unwrap(), symbols, password_batch);
            } else {
                generate_password(length.try_into().unwrap(), symbols)
            }
        }

        &_ => println!("Invalid input"),
    }
}
fn generate_password(length:usize, symbols:bool) { 
    let pg = PasswordGenerator {
        length: length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: symbols,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    println!("{}", pg.generate_one().unwrap());
    //println!("{:?}", pg.generate(5).unwrap());
}

fn generate_multiple (length:usize, symbols:bool, number:u32) { 
    let pg = PasswordGenerator {
        length: length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: symbols,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    println!("{:?}", pg.generate(number.try_into().unwrap()).unwrap());
}

fn store(username: String, password: String) {
    use crate::schema::users::*;
    use crate::schema::users::dsl::users;

    let connection = database::establish_connection();
    // hash it 
    let salt = hasher::gen_salt();
    let hashed = hasher::bcrypt(10, &salt, &password);
    // insert it
    // can't insert pg connections
    // must insert a table
    insert_into(users::table)
        // adds random name and hashed password to database
        .values((name.eq(username.to_string()), password.eq(hashed.to_string())))
        .execute(connection)
}
