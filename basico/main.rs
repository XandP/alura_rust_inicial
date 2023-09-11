const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;
static mut VARIAVEL_GLOBAL_INSEGURA:u8 = 2;

fn main() {
    println!("PI = {}", PI);
    println!("Variavel Global = {}", VARIAVEL_GLOBAL);

    unsafe{
        println!("Variavel Global Mutavel= {}", VARIAVEL_GLOBAL_INSEGURA);
    }



    let variavel:i32 = 300;

    println!("Variavel = {}, Tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("Decimal = {}", decimal);

    let mut booleana = false;
    booleana = true;
    println!("Booleana = {}, Tamanho = {} bytes", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho char = {}", std::mem::size_of_val(&letra));
}