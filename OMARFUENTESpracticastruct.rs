struct Persona {
    nombre: String,
    apellidos: String,
    edad: u8,
    peso: f32,
    altura: f32,
    curp: String,
    rfc: String,
}

impl Persona {
    fn info(&self) {
        println!("\n---Informacion del cliente:");
        println!("Nombre: {}", self.nombre);
        println!("Apellidos: {}", self.apellidos);
        println!("Edad: {}", self.edad);
        println!("Peso: {:.2} kg", self.peso);
        println!("Altura: {:.2} m", self.altura);
        println!("Curp: {}", self.curp);
        println!("RFC: {}", self.rfc);
        println!("IMC: {:.2}", self.calcular_imc());
    }

    fn calcular_imc(&self) -> f32 {
        self.peso / (self.altura * self.altura)
    }
}

fn main() {
    let persona = Persona {
        nombre: String::from("Omar Gerardo"),
        apellidos: String::from("Fuentes Huaira"),
        edad: 31,
        peso: 93.0,
        altura: 1.80,
        curp: String::from("FUHO931122HSLNERM02"),
        rfc: String::from("FUHO931122P62"),
    };

    persona.info();
    println!("IMC calculado: {:.2}", persona.calcular_imc());  
}