//Diego Santos Treviño Camacho

struct caracteristicas{
        cabello : String,
        estatura : f32,
        peso : i8,
        ojos : String,
        piel : String,
        nombre : String,
        apellido : String,
    }
    
impl caracteristicas{
    fn imprimir(&self){
    println!("El nombre es: {}",self.nombre);
    println!("El apellido es: {}",self.apellido);
    println!("El color de cabello es: {}",self.cabello);
    println!("La estatura en metros es: {}",self.estatura);
    println!("El peso en kg es: {}",self.peso);
    println!("El color de ojos es: {}",self.ojos);
    println!("El color de piel es: {}",self.piel);
    }
}

fn main(){
    let caracteristicas2 : caracteristicas = caracteristicas{
        cabello : String::from("castaño"),
        estatura : 1.87,
        peso : 91,
        ojos : String::from("cafes"),
        piel : String::from("morena"),
        nombre : String::from("Diego"),
        apellido : String::from("Treviño"),
    };
    
    caracteristicas2.imprimir();

}