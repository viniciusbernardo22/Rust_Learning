use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    return data_input.trim().parse::<i32>().unwrap();
}

fn main() {
    let mut soma: i32 = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Erro na leitura do valor_entrada");

    let mut valor_i32: i32 = convert_to_int(&valor_entrada);

    while valor_i32 != 0 {
        let mut r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;

        println!("{}", soma)
    }

}
