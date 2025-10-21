struct Persona {
    nombre: String,
    apellido: String,
    edad: i8,
    sexo: String,
    altura: i16,
    peso: i16,
    color_de_piel: String,
    color_de_ojos: String,
    color_de_cabello: String,
}

impl Persona {
    fn caracteristicas(&self) {
        println!(
            "Mi nombre:{} {}, edad:{} años, sexo:{}, altura:{} cm, peso:{} kg,
            color de piel:{}, ojos de color:{} y mi cabello es de color {}.",
            self.nombre,
            self.apellido,
            self.edad,
            self.sexo,
            self.altura,
            self.peso,
            self.color_de_piel,
            self.color_de_ojos,
            self.color_de_cabello
        );
    }
}

fn main() {
    let persona_n = Persona {
        nombre: String::from("Jose David"),
        apellido: String::from("Sicairos Fonseca"),
        edad: 19,
        sexo: String::from("Masculino"),
        altura: 179,
        peso: 81,
        color_de_piel: String::from("moreno"),
        color_de_ojos: String::from("Marrones"),
        color_de_cabello: String::from("Negro"),
    };

    persona_n.caracteristicas();
}
