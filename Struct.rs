//Camacho Quevedo Carolina Isabel
struct Perfil {
    nombre: String,
    edad: u16,
    genero: String,
    peso: f32,
    estatura: f32,
    nacionalidad: String,
    correo: String,
}

impl Perfil {
    fn imprimir(&self) {
        println!("--- Perfil ---");
        println!("Nombre: {}", self.nombre);
        println!("Edad: {} años", self.edad);
        println!("Género: {}", self.genero);
        println!("Peso: {} kg", self.peso);
        println!("Estatura: {} m", self.estatura);
        println!("Nacionalidad: {}", self.nacionalidad);
        println!("Correo: {}", self.correo);
    }
}

fn main() {
    let persona1 = Perfil {
        nombre: String::from("Juan Perez"),
        edad: 32,
        genero: String::from("Masculino"),
        peso: 67.2,
        estatura: 1.80,
        nacionalidad: String::from("Mexicana"),
        correo: String::from("JuanPerez@gmail.com"),
    };

    println!("Nombre: {}", persona1.nombre);
    persona1.imprimir();
}
