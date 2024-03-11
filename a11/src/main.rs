use std::io;

fn taboada(num: i32){
    for i in 1..11{
        println!("{} x {} = {}", num, i, num * i)
    }
}

fn main() {

    let mut numero = String::new();

    std::io::stdin().read_line(&mut numero).expect("Falha ao ler a entrada");

    let num: i32 = numero.trim().parse().expect("Por favor, insira um número inteiro!");


    taboada(num);
}
