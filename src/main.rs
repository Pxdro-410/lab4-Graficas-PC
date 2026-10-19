mod framebuffer;
mod model;
mod triangle;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use model::Model;
use nalgebra_glm::{rotate_x, rotate_y, rotate_z, vec3, vec4, Mat4, Vec3};
use triangle::draw_triangle;

const WINDOW_WIDTH: usize = 900;
const WINDOW_HEIGHT: usize = 700;

/// Transforma los vértices 3D del modelo al espacio de pantalla:
/// 1. Centra el modelo en el origen restando su centro geométrico.
/// 2. Aplica rotación tridimensional.
/// 3. Escala y traslada al centro del Framebuffer.
fn render(
    framebuffer: &mut Framebuffer,
    model: &Model,
    rotation: &Vec3,
    scale: f32,
    wireframe: bool,
) {
    let center = model.center();
    let screen_w = framebuffer.width as f32;
    let screen_h = framebuffer.height as f32;

    // Matriz de rotación en los tres ejes
    let mut rot_matrix = Mat4::identity();
    rot_matrix = rotate_x(&rot_matrix, rotation.x);
    rot_matrix = rotate_y(&rot_matrix, rotation.y);
    rot_matrix = rotate_z(&rot_matrix, rotation.z);

    // Transformar todos los vértices a coordenadas de pantalla
    let transformed_vertices: Vec<Vec3> = model
        .vertices
        .iter()
        .map(|v| {
            // Centrar respecto al Bounding Box
            let centered = v - center;

            // Aplicar rotación
            let v4 = vec4(centered.x, centered.y, centered.z, 1.0);
            let rotated = rot_matrix * v4;

            // Proyectar y escalar al centro de la pantalla
            let screen_x = rotated.x * scale + (screen_w / 2.0);
            let screen_y = -rotated.y * scale + (screen_h / 2.0);
            let screen_z = rotated.z;

            vec3(screen_x, screen_y, screen_z)
        })
        .collect();

    // Recorrer manualmente las caras y dibujar cada triángulo
    for face in &model.faces {
        let v0 = &transformed_vertices[face[0]];
        let v1 = &transformed_vertices[face[1]];
        let v2 = &transformed_vertices[face[2]];

        draw_triangle(framebuffer, v0, v1, v2, wireframe);
    }
}

fn main() {
    println!("Fase 3: Proyección, Centrado, Escalado y Visualización en Pantalla");

    let obj_path = "nave-lab3-PC.obj";
    let model = match Model::load(obj_path) {
        Ok(m) => {
            println!("Modelo cargado exitosamente:");
            println!("* Vértices: {}", m.vertices.len());
            println!("* Caras: {}", m.faces.len());
            println!("* Centro: {:?}", m.center());
            println!("* Extensión máxima: {:.4}", m.max_extent());
            m
        }
        Err(e) => {
            eprintln!("Error al cargar el archivo OBJ '{}': {}", obj_path, e);
            return;
        }
    };

    // Crear la ventana de visualización con minifb
    let mut window = Window::new(
        "Laboratorio 4 - Renderizador OBJ - Pedro Caso",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana de minifb: {}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    framebuffer.set_background_color(0x0a0a15); 
    framebuffer.set_current_color(0xFF0000);  

    // Calcular factor de escala para que ocupe el 70% de la pantalla
    let target_screen_size = (WINDOW_HEIGHT as f32).min(WINDOW_WIDTH as f32) * 0.70;
    let base_scale = target_screen_size / model.max_extent();
    let mut current_scale = base_scale;

    // Ángulos iniciales de rotación
    let mut rotation = vec3(-0.35, 0.70, 0.0);
    let mut wireframe_mode = true;

    println!("\n Controles de la ventana");
    println!("* Flechas / A-D-W-S : Rotar la nave en 3D");
    println!("* Q / E             : Zoom (acercar / alejar)");
    println!("* M                 : Alternar entre modo Wireframe y Sólido");
    println!("* P                 : Guardar captura de pantalla en render.png");
    println!("* Espacio           : Restablecer vista inicial");
    println!("* ESC               : Salir");

    // Renderizar la pose inicial y generar automáticamente render.png para el laboratorio
    framebuffer.clear();
    render(&mut framebuffer, &model, &rotation, current_scale, wireframe_mode);
    if let Err(e) = framebuffer.save_to_file("render.png") {
        eprintln!("Error al generar render.png: {}", e);
    } else {
        println!("Imagen 'render.png' generada exitosamente.");
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejo de rotación
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            rotation.y -= 0.04;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            rotation.y += 0.04;
        }
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            rotation.x -= 0.04;
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            rotation.x += 0.04;
        }

        // Manejo de Zoom
        if window.is_key_down(Key::E) {
            current_scale *= 1.02;
        }
        if window.is_key_down(Key::Q) {
            current_scale *= 0.98;
        }

        // Restablecer vista
        if window.is_key_down(Key::Space) {
            rotation = vec3(-0.35, 0.70, 0.0);
            current_scale = base_scale;
        }

        // Alternar modo Wireframe / Sólido al presionar M
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            wireframe_mode = !wireframe_mode;
            println!("Modo cambiado a: {}", if wireframe_mode { "Wireframe" } else { "Sólido" });
        }

        // Guardar captura con la tecla P
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            if let Err(e) = framebuffer.save_to_file("render.png") {
                eprintln!("Error al guardar captura: {}", e);
            } else {
                println!("Nueva captura guardada como 'render.png'");
            }
        }

        // Limpiar buffer con color de fondo
        framebuffer.clear();

        // Renderizar el modelo en el centro de la pantalla
        render(&mut framebuffer, &model, &rotation, current_scale, wireframe_mode);

        // Volcar el framebuffer a la ventana
        window
            .update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}