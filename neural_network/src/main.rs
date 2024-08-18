fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

fn dsigmoid(value: f64) -> f64 {
    value * (1.0 - value)
}

fn main() {
    let result1 = dsigmoid(1.0);
    let result2 = sigmoid(1.0);
    
    println!("Resultado 1 = {result1} ");
    println!("Resultado 2 = {result2} ");
}
