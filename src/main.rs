use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. Pertambahan");
        println!("2. Pengurangan");
        println!("3. Perkalian");
        println!("4. Pembagian");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        let choice: u32 = match choice.trim().parse() {
            Ok(num)=> num,
            Err(_)=> {
                println!("bro, my expectation for you were low but holy fuck. yang bener lah, nyet.");
            continue;
            }
        };

        if choice ==5 {
            println!("Bye.");
            break;
        }

        if choice >5 {
            println!("bro pikir dia think outside the box.");
            break;
        }

        if choice ==0 {
            println!("bro pikir dia think outside the box.");
            break;
        }
        println!("Masukkan angka pertama:");
        let mut num1= String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line.");
        let num1: f64 = match num1.trim().parse() {
            Ok(num)=> num,
            Err(_)=>  {
                println!("Masukin angka bro, 0 lah minimal.");
            continue;
            }
            };

        println!("Masukkan angka kedua:");
        let mut num2= String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line.");
        let num2: f64 = match num2.trim().parse() {
            Ok(num)=> num,
            Err(_)=> { 
                println!("bro, my expectation for you were low but holy fuck. yang bener lah, nyet.");
            continue;
                }
         };

        match choice {
            1=> {
                let result = num1 + num2;
                println!("Hasil: {}", result);
            }
            
            2=> {
                let result = num1 - num2;
                println!("Hasil: {}", result);
            }

            3=> {
                let result = num1 * num2;
                println!("Hasil: {}", result);
            }

            4=> {
                if num2 != 0.0 {
                    let result = num1 * num2;
                    println!("Hasil: {}", result);
                } else {
                    println!("gabisa bro, pusing gw jadinya.");
                }
            }
            5=> {
                break;
            }
            _=> {
                println!("Emang ada opsinya? bisa baca ngga?nyet.");
            }
        }
    }
}
