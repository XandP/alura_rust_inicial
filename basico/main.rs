const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;
static mut VARIAVEL_GLOBAL_INSEGURA:u8 = 2;

fn main() {
    // testes_var();
    // sombra();

    println!("Soma = {}", soma(2, 2));
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

    let mut booleana = false;
    booleana = true;
    println!("Booleana = {}, Tamanho = {} bytes", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho char = {}", std::mem::size_of_val(&letra));
}