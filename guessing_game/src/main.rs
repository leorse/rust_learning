use std::io;
use rand::Rng;

fn main() {
    println!("Trouve le nombre:");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Entre un nombre: {}", secret);

    let mut chaine = String::new();
    let retour = io::stdin().read_line(&mut chaine);
    //.expect("ça a planté");
    assert!(retour.is_ok());

    println!("tu as entré {}, retour expect {}", chaine, retour.expect("oush"));
}
