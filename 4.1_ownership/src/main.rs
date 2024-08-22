fn main() { // string_value NÃO é valida aqui
    let _string_value = "UMA STRING, UAU!"; // string_value é valida A PARTIR daqui

    // -------------------------------------------

    let _string_literal = "literalmente uma string";
    //Sem mut o borrow check me bateu, não sei porque ainda
    //::from solicita/aloca memória no Heap
    let mut uma_senhora_string_de_respeito = String::from("ISSO É UMA STRING, em heap no caso");

    println!("Antes - {uma_senhora_string_de_respeito}");

    uma_senhora_string_de_respeito.push_str(", e agora alterada pelo push");
    println!("Depois - {uma_senhora_string_de_respeito}");
} // string_value NÃO é valida aqui, pois aqui está sendo chamado implicitamente a variável drop 
  // que desocupa a memória que estava sendo usada pela "uma_senhora_string_de_respeito"
