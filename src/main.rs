mod framebuffer;
mod model;

use model::Model;

fn main() {
    println!("Fase 1: Prueba de Carga Manual de Modelo OBJ");
    let obj_path = "nave-lab3-PC.obj";

    match Model::load(obj_path) {
        Ok(model) => {
            println!("Modelo cargado exitosamente desde '{}'", obj_path);
            println!("* Vértices cargados: {}", model.vertices.len());
            println!("* Caras (triángulos): {}", model.faces.len());
            println!("* Bounding Box Min: {:?}", model.min_bound);
            println!("* Bounding Box Max: {:?}", model.max_bound);
            println!("* Centro geométrico: {:?}", model.center());
            println!("* Dimensiones (X, Y, Z): {:?}", model.dimensions());
            println!("* Extensión máxima: {:.4}", model.max_extent());
        }
        Err(e) => {
            eprintln!("Error al cargar el archivo OBJ: {}", e);
        }
    }
}