use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Trouve le nombre:");

    let mut secret = rand::thread_rng().gen_range(1, 101);
    for _elem in 1..100 {
        secret = rand::thread_rng().gen_range(1, 101);
        println!("Entre un nombre: {}", secret);
    }
    loop {
        println!("Entre un nombre: {}", secret);

        let mut chaine = String::new();
        let retour = io::stdin().read_line(&mut chaine);

        assert!(retour.is_ok());

        println!(
            "tu as entré {}, retour expect {}",
            chaine,
            retour.expect("oush")
        );
        let valeur = match chaine.trim().parse() {
            /*Ok(50) => {
                println!("tu as entrée 50?");
                50;
            }*/
            Ok(num) => num,
            
            Err(_) => {
                println!("tu t'es planté connard!!!");
                continue;
            }
        };

        match secret.cmp(&valeur) {
            Ordering::Greater => println!("plus grand"),
            Ordering::Less => println!("plus petit"),
            Ordering::Equal => {
                println!("égal");
                break;
            }
        }
    }
}
