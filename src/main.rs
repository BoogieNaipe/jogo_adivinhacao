
use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    println!("Bem vindo ao jogo da Senha!");
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1,10);
    println!("O número secreto é: {numero_secreto}");

    loop{

    println!("Por favor, digite o número.");

    let mut palpite = String::new();

    io::stdin()
    .read_line(&mut palpite)
    .expect("Falha ao ler linha");

    let palpite: u32 = match palpite.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };
   
    println!("Você digitou:{palpite}");

    match palpite.cmp(&numero_secreto){
        Ordering::Less => println!("Muito baixo!"),
        
        Ordering::Greater =>println!("Muito alto!"),
        Ordering::Equal => {println!("Você acertou!");
        break;
    }
}
}
}
