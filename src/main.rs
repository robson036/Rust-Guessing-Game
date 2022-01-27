use std::io;
use rand::Rng;
use std::cmp::Ordering;


struct Colors {
    yellow: String,
    green: String,
    blue: String,
    bold: String,
    red: String,
    end: String
}

impl Colors {
    fn new() -> Colors {
        Colors {
            yellow: "\x1b[93m".to_string(),
            green: "\x1b[92m".to_string(),
            blue: "\x1b[94m".to_string(),
            red: "\x1b[91m".to_string(),
            bold: "\x1b[1m".to_string(),
            end: "\x1b[0m".to_string()
        }
    }
}

fn clean_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {

    let colors = Colors::new();
    clean_terminal();
    println!("{}Adivinhe o número!{}", colors.blue, colors.end);

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let hits;

    loop {
        println!("{}Por favor, insira seu palpite.{}", colors.bold, colors.end);

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect(
            &format!("{}Falha ao ler entrada{}", colors.red, colors.end)
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}Você digitou um número?{}", colors.red, colors.end);
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}Muito baixo{}", colors.yellow, colors.end),
            Ordering::Greater => println!("{}Muito alto{}", colors.yellow, colors.end),
            Ordering::Equal => {
                println!("{}Acertou!{}", colors.green, colors.end);

                hits = secret_number;

                println!("{}Jogar novamente? s/n{}", colors.bold, colors.end);
                let mut play_again = String::new();

                io::stdin()
                .read_line(&mut play_again)
                .expect(
                    &format!("{}Falha ao ler entrada{}", colors.red, colors.end)
                );

                if play_again.contains("s") {
                   main();
                }

                break;
            }
        }
    }

    println!("{}Palpite correto: {}{}", colors.green, hits, colors.end);
}


