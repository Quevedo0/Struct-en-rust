struct Usuario {
    nombre: String,
    edad: u8,
    correo: String,
    altura: f32,
    color_ojos: String,
    peso: f32,
    color_favorito: String,
}

fn imprimir_usuario(u: &Usuario) {
    println!("Nombre: {}", u.nombre);
    println!("Edad: {}", u.edad);
    println!("Correo: {}", u.correo);
    println!("Altura: {} m", u.altura);
    println!("Color de ojos: {}", u.color_ojos);
    println!("Peso: {} kg", u.peso);
    println!("Color favorito: {}", u.color_favorito);
}

fn main() {
    let usuario1 = Usuario {
        nombre: String::from("Arredondo"),
        edad: 23,
        correo: String::from("ft69120@gmail.com"),
        altura: 1.83,
        color_ojos: String::from("Marrones"),
        peso: 85.0,
        color_favorito: String::from("negro"),

    };

    imprimir_usuario(&usuario1);
}
