//elias cuevas herrera
fn main() {
    let persona1= Persona::new(String::from("Elias"), 20, 1.79, 128.45, String::from("Masculino"));

    struct Persona {
        nombre: String,
        edad: u32,
        altura: f32,
        peso: f32,
        genero: String,
    }
    impl Persona {
        fn new(nombre: String, edad: u32, altura: f32, peso: f32, genero: String) -> Persona {
            Persona {
                nombre,
                edad,
                altura,
                peso,
                genero,
            }
        }
    }
    println!("nombre: {}", persona1.nombre);
    println!("edad: {}", persona1.edad);
    println!("altura: {}", persona1.altura);
    println!("peso: {}", persona1.peso);
    println!("genero: {}", persona1.genero);
}