mod framebuffer;
mod model;
mod triangle;

use framebuffer::Framebuffer;
use model::Model;
use nalgebra_glm::Vec3;
use triangle::{draw_line, draw_triangle_wireframe, draw_triangle_filled};

fn main() {
    println!("Fase 2: Algoritmo de Bresenham y Función de Pintado de Triángulos");

    // 1. Verificar carga de modelo
    let obj_path = "nave-lab3-PC.obj";
    match Model::load(obj_path) {
        Ok(model) => {
            println!("Modelo '{}' listo ({} vertices, {} caras)", obj_path, model.vertices.len(), model.faces.len());
        }
        Err(e) => {
            eprintln!("Error al cargar el archivo OBJ: {}", e);
            return;
        }
    }

    // 2. Prueba del algoritmo de Bresenham y pintado de triángulos
    let mut fb = Framebuffer::new(100, 100);
    fb.set_current_color(0x00FF00); // Verde

    // Línea de prueba con Bresenham
    draw_line(&mut fb, (10, 10), (90, 10));

    // Triángulo wireframe
    let v0 = Vec3::new(20.0, 30.0, 0.0);
    let v1 = Vec3::new(80.0, 30.0, 0.0);
    let v2 = Vec3::new(50.0, 80.0, 0.0);
    draw_triangle_wireframe(&mut fb, &v0, &v1, &v2);

    // Triángulo relleno
    fb.set_current_color(0xFF0000); // Rojo
    let vf0 = Vec3::new(35.0, 45.0, 0.0);
    let vf1 = Vec3::new(65.0, 45.0, 0.0);
    let vf2 = Vec3::new(50.0, 70.0, 0.0);
    draw_triangle_filled(&mut fb, &vf0, &vf1, &vf2);

    println!("Línea y triángulos (wireframe y relleno) dibujados exitosamente en el Framebuffer.");
}