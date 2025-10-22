//Kevin Omar Hernandez Martinez

struct Persona {
    nombre: String,
    email: String,
    edad: u8,
    altura: f32,
    sexo: String,
    ciudad: String,
    cumpleaños: String,
}


impl Persona {
    fn mostrar(&self) {
        println!("El nombre es: {}", self.nombre);
        println!("El email es: {}", self.email);
        println!("La edad es: {}", self.edad);
        println!("La altura es: {} m", self.altura);
        println!("El sexo es: {}", self.sexo);
        println!("la ciudad es: {}", self.ciudad);
        println!("El cumpleaños es: {}", self.cumpleaños);
    }
}

fn main() {

    let alumno = Persona {
        nombre: String::from("Kevin Hernandez"),
        email: String::from("kevin.hdz@ejemplo.com"),
        edad: 23,
        altura: 1.75,
        sexo: String::from("Masculino"),
        ciudad: String::from("Culiacán"),
        cumpleaños: String::from("28 de Agosto"),
    };

    alumno.mostrar();
}
