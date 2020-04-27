struct UneStruct {
    x:u32,
    y:u32
}

fn main() {
    let mut structure = UneStruct {
        x:45,
        y:34
    };
    let mut mot: String = String::from("Un petit mot");
    structure.x = 43;

    let mut var:u32 =45;
    let machaine = "j'ai une chaine";
    let mut slice:String = machaine.to_uppercase();
    slice.push('p');
    while var>10 {
        println!("var vaut:{}", var);
        var=var-1;
    }
    for unevar in -15..45 {
        println!("var vaut:{}", unevar)
    }
    println!("Hello, world! {}", var);

    let _chaine:&str = "poi";
    let uneautrechaine:String;
uneautrechaine = var.to_string();
    println!("ma chaine vaut:{}, {}", uneautrechaine, slice);
    let mut chaineMutStr:&str = "chaine mutable";
    chaineMutStr = "poi";
    autrefonction(&mut chaineMutStr);
    let chaineString:String=String::from("");
    fonctionStr(chaineString);
    println!("chaineString: {}", chaineString);
}
fn fonctionStr(mut chaine:String){
    chaine = String::from("poipoi");
}

fn autrefonction(chaine:&mut str) {
    let mut souschaine:String;
    souschaine = chaine.to_owned();
    souschaine = souschaine+"poi";
    println!("autre fonction {}", souschaine);
}
