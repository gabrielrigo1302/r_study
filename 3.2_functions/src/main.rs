fn main() {
    println!("Hello, world!");
    another_function(2, 'c');
    statement_function();
    println!("{}",expression_function(2));
}

fn another_function(x: i32, y: char) {
    println!("x = {x} e y = {y}");
}

fn statement_function() {
    //isso são declarações, não retornam valor;
    let x = 0;
    println!("x = {x}");
}

fn expression_function(x: u32) -> u32 {
    //isso são expressões (z + 1 e y + x), retornam valor;
    let y:u32 = {
        let z:u32 = 4;
        z + 1
    };

    y + x
}