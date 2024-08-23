
use std::io;
use rand::Rng;


fn main() {
    println!("Bem vindo ao jogo da Senha!");
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1,10);
    println!("O número secreto é: {numero_secreto}");

    println!("Por favor, digite o número.");

    let mut palpite = String::new();

    io::stdin()
    .read_line(&mut palpite)
    .expect("Falha ao ler linha");
    println!("Você digitou:{palpite}");
}
