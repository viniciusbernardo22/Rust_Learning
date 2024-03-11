
fn eh_primo(num: i32) -> bool {

    if num <= 1 {
      return false;  
    } 

    let limite: i32 = (num as f32).sqrt() as i32 +1;

    for i in 2..limite {

        if num % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let numero = 10;
    let ehprimo = eh_primo(numero);

    if ehprimo {
        println!("O numero {} é primo", numero)
    } else {
        println!("O numero {} não é primo", numero)
    }
    
}
