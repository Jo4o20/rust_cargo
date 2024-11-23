fn main() {
    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}\n", indice + 1, notas[indice]);
    }

    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Domingo));

    cores();
    conteudo_opcional();
    vector();
    conta_corrente();
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4],
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CymkColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

fn cores() {
    let cor = Color::CymkColor {
        cyan: 100,
        magenta: 100,
        yellow: 100,
        black: 0,
    };

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RGBColor(0, 0, 0) => "preta",
            Color::RGBColor(255, 255, 255) => "branco",
            Color::RGBColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor { black: 255, .. } => "preta",
            Color::CymkColor { .. } => "CYMK desconhecida",
        }
    );
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));
    match conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe"),
    };
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
    // None
}

fn vector() {
    let mut notas: Vec<f32> = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("Exibe todas as notas iniciadas no vetor");
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    println!("Mostra a nota 4, que não existe ainda, então mostra 0 no lugar");
    println!(
        "Nota 4 = {}",
        match notas.get(3) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    notas.push(7.5);
    println!("Mostra a nota 4");
    println!(
        "Nota 4 = {}",
        match notas.get(3) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    println!("Mostra todas as notas em sequência");
    for nota in &notas {
        println!("Nota = {}", nota);
    }

    println!("Capacidade = {}", notas.capacity());
}

struct Titular {
    nome: String,
    sobrenome: String,
}

struct Conta {
    titular: Titular,
    saldo: f64,
}

impl Conta {
    fn sacar(&mut self, valor :f64){
        self.saldo -= valor;
    }
    
}

fn conta_corrente() {
    let titular = Titular { nome: String::from("João"), sobrenome: String::from("Gabriel")};
    let mut conta: Conta = Conta {
        titular,
        saldo: 100.0
    };

    conta.sacar(20.0);

    println!(
        "Dados da conta: titular = {} {}, Saldo = {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}
