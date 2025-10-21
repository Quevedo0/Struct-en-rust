struct Persona {
    usuario: String,
    edad: u8,
    peso: u8,
    estatura: i16,
    sexo: String,
    nombre: String,
    vivo: bool,
}

fn print_persona(p: &Persona) {
    println!("usuario: {}", p.usuario);
    println!("edad: {}", p.edad);
    println!("peso: {}", p.peso);
    println!("estatura: {}", p.estatura);
    println!("sexo: {}", p.sexo);
    println!("nombre: {}", p.nombre);
    println!("vivo: {}", p.vivo);
}

fn main() {
    let usuario1 = Persona {
        usuario: String::from("dani123"),
        edad: 25,
        peso: 70,
        estatura: 175,
        sexo: String::from("masculino"),
        nombre: String::from("Daniel Enrique"),
        vivo: true,
    };

    print_persona(&usuario1);
}