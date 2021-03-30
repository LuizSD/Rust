struct Guilda<'a>{
    nome: &'a str,
    apresentador: &'a str
}

fn main(){
    let mut nome = "Fractal";
    let mut apresentador = "";

    let guilda = Guilda{
        nome,
        apresentador
    };
    
    println!("A melhor guilda é a {}", guilda.nome);
    
    nome = "Frontend";
    
    let guilda2 = Guilda{
        nome,
        apresentador
    };

    println!("A segunda melhor guilda é a {}", guilda2.nome);
}

// struct Guilda<'a>{
//     nome: &'a str,
//     apresentador: &'a str
// }

// fn main(){
//     let nome = "Fractal";
//     let apresentador = "Luiz";

//     let guilda = Guilda{
//         nome,
//         apresentador
//     };

//     println!("A melhor guilda é a {}", guilda.nome);
// }