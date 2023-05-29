fn Exemplo_1() {
    // Fuction main
    struct Numbers {
        y: i32, // Estou declarando Dois numemros do tipo i32
        x: i32,
    }
    let Alocamento = Numbers { y: 383, x: 30 }; // Aqui estou alocando o Valor

    println!(
        "O valor y é de ({}) e o valor x é de ({})",
        Alocamento.y, Alocamento.x
    );
}

fn Exemplo_2() {
    // Exemplo dois Alocando o valor na variavel
    // Fuction main
    struct Numbers {
        y: i32, // Estou declarando Dois numemros do tipo i32
        x: i32,
    }
    let mut Alocamento = Numbers { y: 383, x: 30 }; // Aqui estou alocando o Valor

    Alocamento.y = 10; // Estou mundado os valores na struct
    Alocamento.x = 20;

    println!(
        "O valor y é de ({}) e o valor x é de ({})",
        Alocamento.y, Alocamento.x
    );
}

fn main() {
    Exemplo_1();
    Exemplo_2();
}
