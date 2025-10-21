struct User {
    nombre: String,
    sexo: String,
    edad: u16,
    altura: f32,
    peso: f32,
    color_piel: String,
    color_ojo: String,
    vivo: bool,
    pelo: bool,
    barba: bool,
}

impl User {
    fn mostrar(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Sexo: {}", self.sexo);
        println!("Edad: {}", self.edad);
        println!("Altura: {}", self.altura);
        println!("Peso: {}", self.peso);
        println!("Color de piel: {}", self.color_piel);
        println!("Color de ojos: {}", self.color_ojo);
        println!("Vivo: {}", self.vivo);
        println!("Pelo: {}", self.pelo);
        println!("Barba: {}", self.barba);
    }
}

fn main() {
    let user1 = User {
        nombre: String::from("Emilio Rafael Beltran Barraza"),
        sexo: String::from("hombre"),
        edad: 19,
        altura: 1.80,
        peso: 67.3,
        color_piel: String::from("duvalin"),
        color_ojo: String::from("cafe"),
        vivo: true,
        pelo: true,
        barba: false,
    };

    user1.mostrar();  
}