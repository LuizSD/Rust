fn valida(denominador: f64) -> Result<f64, String> {
    if denominador == 0.0 {
        Err(String::from("Por zero não ne irmão"))
    } else {
        Ok(denominador)
    }
}

fn divide(numerador: f64, denominador: f64) -> Result<f64, String> {
    let denominador = valida(denominador)?;
    Ok(numerador/denominador)
}

fn main() {
    let retorno = divide(15.,3.);
    let _teste = retorno.unwrap();
    println!("{}",_teste);
    //assert_eq!(retorno,Ok(5.));
}

// match retorno {
//     Ok(v) => println!("Sucesso: {}", v),
//     Err(e) => println!("Erro: {}", e),
// }