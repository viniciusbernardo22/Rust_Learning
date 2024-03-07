use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Jogo: Advinhe o número");

    let secret_number:u32 = rand::thread_rng()
    .gen_range(1, 100);
    println!("Secret number: {}", secret_number);

    loop {

    println!("Informe um número: ");
    //Alocar espaço na memoria para uma string
    let mut guess: String = String::new();
    //Receber entrada pelo console
    io::stdin()
    .read_line(&mut guess)
    //Tratamento de erros
    .expect("Falha ao ler o numero");

    println!("Você chutou o numero {guess}");

    //Conversão de string para u32 
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    //com base no input faz a comparação
    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("Parabéns, você ganhou !!!!!! ");
            println!("o numero secreto é {}", secret_number);
            break;
        },
        Ordering::Less => println!("O numero secreto é maior."),
        Ordering::Greater => println!("O numero secreto é menor."),
    }
        
    }
    
  

}
