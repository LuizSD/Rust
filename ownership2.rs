fn aniversario(idade: i32) -> i32 {
    idade + 1
}

fn main() {
    let idade_atual = 27;
    let nova_idade = aniversario(idade_atual);

    println!("Sua nova idade é: {}, antes vocês tinha {}", nova_idade, idade_atual);
}


// fn aniversario(idade: i32 mut) -> i32 {
//     idade = idade + 1;
//     return idade;
// }

// fn main() {
//     let idade_atual = 27;
//     let nova_idade = aniversario(idade_atual);

//     println!("Sua nova idade é: {}, antes vocês tinha {}", nova_idade, idade_atual);
// }


// fn aniversario(idade: &mut u32) {
//     *idade += 1;
// }

// fn main() {
//     let mut idade_atual: u32 = 27;
//     aniversario(&mut idade_atual);

//     println!("Sua nova idade é: {}", idade_atual);
// }



