fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128];

    for &c in input.as_bytes() {
        let indice: usize = c as usize;
        println!("Caractere: {}, indice {}", c as char, indice);

        if conjunto_de_caracteres[indice] {
            println!("Caractere duplicado encontrado!");
            return false;
        }
        conjunto_de_caracteres[indice] = true;
    }

    return true;
}

fn main() {
    let palavra = "cateto";

    if tem_caracteres_unicos(palavra) {
        println!("Possui caracteres unicos");
    } else {
        println!("Não Possui caracteres unicos");

    }
}
