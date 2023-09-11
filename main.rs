const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;
static mut VARIAVEL_GLOBAL_INSEGURA:u8 = 2;

fn main() {
    testes_var();
    sombra();

    println!("Soma = {}", soma(2, 2));

    condicionais();
    repeticoes();
}


fn repeticoes() {

    let multiplicador:u8 = 5;

    let mut i:u8 = 0;

    while i < 10 {
        i += 1;

        if i == 5 {
            continue;
        }

        println!("{} X {} = {}", multiplicador, i, multiplicador * i);
    }


    i = 0;

    loop {
        i += 1;
        println!("{} X {} = {}", multiplicador, i, multiplicador * i);

        if i >= 10 {
            break;
        }
    }
}


fn condicionais() {

    let idade:u8 = 18;
    let responsavel_autorizou = true;
    let bool_maior = idade >= 18;

    if bool_maior {
        println!("Pode entrar na balada");
    }

    else if idade >= 16 && responsavel_autorizou {
        println!("Pode entrar na balada com permissão do responsável");
    }

    else {
        println!("Não pode entrar na balada!");
    }

    let condicao = if bool_maior { "maior" } else { "menor" };

    println!("É {} de idade!", condicao);
}


fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b , a + b);

    a + b
}


fn sombra() {
    let a = 123;

    // SHADOWING
    {
        let b = 456;
        println!("B interior = {}", b);

        let a = 789;
        println!("A interior = {}", a);

    }

    println!("a = {}", a);
    // println!("b = {}", b);
}


fn testes_var() {

    println!("PI = {}", PI);
    println!("Variavel Global = {}", VARIAVEL_GLOBAL);

    unsafe{
        println!("Variavel Global Mutavel= {}", VARIAVEL_GLOBAL_INSEGURA);
    }

    let variavel:i32 = 300;

    println!("Variavel = {}, Tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let variavel:i32 = 301;
    println!("Variavel redeclarada = {}, Tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("Decimal = {}", decimal);

    let booleana = false;
    // booleana = true;
    println!("Booleana = {}, Tamanho = {} bytes", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho char = {}", std::mem::size_of_val(&letra));
}