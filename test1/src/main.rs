fn main() {
    println!("Hello, world!");
    let condition:bool = true;
    let number= if(condition) {5} else {15};

    if number < 10 {
        println!("nombre inférieur: {}", number);
    } else {
        println!("nombre supérieur: {}", number);
    }

    let mut compteur:i32 = 0;
    loop {
        println!("compteur:{}", compteur);

        if compteur>=10 {
            break
        }
        compteur+=1;
    }
    while compteur>=0 {
        println!("while compteur:{}", compteur);
        compteur-=1;
    }

    let tableau = [1,2,3,4,5,6,7];
    let mut inc:usize=0;
    while inc<7 {
        println!("valeur de l'index {} vaut {}", inc, tableau[inc]);
        inc+=1;
    }
    for element in tableau.iter() {
        println!("la valeur est:{}", element);
    }
    for element in (1..55).rev() {
        println!("valeur {}", element);
    }

    let mut s = String::from("wesh!!");
    let const_chaine = "la suite";

    s.push_str(const_chaine);
    println!("mon texte:{}",s);

    let x = 5;
    let y = x;

    let s1 = String::from("oui");
    let s2 = s1.clone();
    println!("s1:{}", s1);

    #[derive(Copy, Clone)]
    struct Point { x: i32, y: i32 }


    let mut p1 = Point{ x: -1, y: 2 };
    let p2 = p1;
    p1.x = 1;
    println!("p1: {}, {}", p1.x, p1.y);
    println!("p2: {}, {}", p2.x, p2.y);
    //https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
}

