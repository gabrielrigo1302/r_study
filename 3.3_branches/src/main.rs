fn main() {
    let number = 3;

    if number < 5 {
        println!("{number} é menor que 5");
    } else {
        println!("{number} é maior ou igual à 5");
    }

    //======================

    let condition = true;
    // cada um dos blocos deve retornar valores de mesmo tipo;
    let number = if condition { 5 } else { -5 };
    println!("O valor de condition={condition} implica em number={number}");

    // ====== loop ======
    let mut counter: i8 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Resultado = {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // ====== while ======

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // ====== for ======

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

