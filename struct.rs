fn main() {
    struct DatosPersona {
        edad: i8,
        sexo: char,
        nombres: String,
        apellidos: String,
        altura: f32,
        num_celular: String,
        correo: String,
    }

    impl DatosPersona {
        fn imprimir(&self) {
            println!("--- DATOS DE LA PERSONA ---");
            println!("Nombre: {} {}", self.nombres, self.apellidos);
            println!("Edad: {}", self.edad);
            println!("Sexo: {}", self.sexo);
            println!("Altura: {} m", self.altura);
            println!("Número de celular: {}", self.num_celular);
            println!("Correo electrónico: {}", self.correo);
        }
    }

    let persona1 = DatosPersona {
        edad: 18,
        sexo: 'H',
        nombres: String::from("Kevin"),
        apellidos: String::from("Salas"),
        altura: 1.70,
        num_celular: String::from("66742241184"),
        correo: String::from("kev@gmail.com"),
    };

    persona1.imprimir();
}
