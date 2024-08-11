// Trás a cargo io para o escopo atual a partir da cargo padrão std
// A cargo std trás uma série de itens e é conhecida como prelude
// Cargos que não sejam a std precisam ser declaradas usando a palavra chava use
// Caso não fosse importado usando use, poderia ter sido importado no códifo da
// seguinte maneira:
// - std::io::Stdin

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tente adivinhar um número de 1 à 100:");

    // thread_rng - função que retorna um número aleatório local para a thread em andamento e
    // cuja seed é gerada pelo OS.
    // gen_range: método qu espera uma expressão de faixa(range)
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Insira sua tentativa: ");

        // let - palavra chave que declara variáveis, por padrão imutáveis
        // mut - palavra chave que torna a variável mutável
        // String::new() - função que retorna uma instancia de String
        // String - tipo fornecido pela std que pode ser aumentada e é codificada UTF-8
        // :: - indica que a função new é associada ao tipo String
        // new - função que faz novos valores de diversos tipo, nesse caso, cria uma String vazia
        let mut chute = String::new();

        // read_line - pega o que o usuário digita e concatena em um parâmetro string MUTÁVEL
        // sem sobrescrever. Retorna um Result (trata se de uma enumeração/enum, um tipo que pode assumir
        // vários estados, sendo cada estado chamado de variante)
        // As variantes de Result são:
        // - Ok: indica sucesso e contém o valor gerado com sucesso
        // - Err: indica erro e contém informações de porque e como o erro aconteceu
        // As variantes possuem métodos, assim como valores de qualquer tipo, e um desses é o expected.
        // Caso a instância de Result seja um valor Err, expect vai fazer o programa crashar e mostrar uma
        // mensagem passado por parâmetro. O compilador não deixa compilar sem o tratamento expect;

        // & - operador para tratar o argumento como referência, para não ter que copiar o valor
        // em memória. Referências também são imutáveis por padrão, então por isso o uso de mut
        io::stdin()
            .read_line(&mut chute)
            .expect("Falha ao tentar ler");

        let chute: u32 = match chute.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("O valor deve ser numérico");
                continue;
            }
        };

        // cmp - método para comparar dois valores
        // match - expressão que faz algo baseado nos valores das variantes de Ordering
        //       - expressão feita baseada em arms (um padrão onde algum valor é avaliado e a partir dos valores
        //         que ele poderia ter, são tomadas as decisões).
        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("Resposta: O número chutado é muito baixo"),
            Ordering::Greater => println!("Resposta: O número chutado é muito alto"),
            Ordering::Equal => {
                println!("Resposta: O número chutado é o correto");
                break;
            }
        }
    }
}
