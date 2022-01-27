use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let hits;

    loop {
        println!("Por favor, insira seu palpite.");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Acertou!");

                hits = secret_number;

                println!("Jogar novamente? s/n");
                let mut play_again = String::new();

                io::stdin().read_line(&mut play_again).expect("Digitou errado");

                if play_again.contains("s") {
                   main();
                }

                break;
            }
        }
    }

    println!("Palpite correto: {}", hits);
}


