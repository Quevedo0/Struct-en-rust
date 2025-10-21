fn main() {
  
    struct Usuario {
        
        nombre: String,
        estudio: String,
        edad: u8,
        sexo: String,
        estado: String,
        altura:f32,
        peso:f32,
    }

 
    let persona = Usuario {
        
        nombre: String::from("Wendy"),
        estudio: String::from("Universitario"),
        edad: 19,
        sexo: String::from("Femenino"),
        estado: String::from("solter1a"),
        altura: 64.5,
        peso:50.6,
    };

    println!("Nombre: {}", persona.nombre);
    println!("Estudios: {}", persona.estudio);
    println!("Edad: {}", persona.edad);
    println!("sexo: {}", persona.sexo);
    println!("Estado: {}", persona.estado);
    println!("Altura: {} M", persona.altura);
    println!("peso: {} kg ", persona.peso);
    
}

