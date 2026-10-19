# Laboratorio 4 - Renderizador de Modelos OBJ

## Autor
* Nombre: Pedro Caso
* Carné: 241286

## Descripción
Este proyecto consiste en un renderizador por software desarrollado en Rust para cargar y visualizar modelos tridimensionales en formato OBJ. El sistema recorre manualmente los vértices y las caras trianguladas del archivo, calculando las transformaciones necesarias para proyectar el modelo centrado y a escala adecuada en la pantalla.

## Demostración
#### Malla triangular y relleno
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/562e1aac-3ba8-4a5b-bf59-c2447e1755a9" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/6d931438-4bfd-4e3b-b79a-af10d78bcbf8" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/62d1ec04-8651-4f1d-a58c-df139df45161" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/0f01e4c5-dbe2-4b8c-8148-a0834da467af" />


#### Solamente malla triangular
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/4b86efd0-e675-4347-8a22-84fd84a433ef" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/91c5402b-e558-45b4-8eb7-2c42eb75d5a0" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/dc42a07c-0c78-469c-a065-c48d77f04611" />


#### Solamente relleno de los triangulos
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/9f110b0c-87f5-402f-917e-344d1f69d19f" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/0a0d2094-2f32-488d-b715-4e7ff178e6db" />
<img width="900" height="700" alt="render" src="https://github.com/user-attachments/assets/c4e2e24c-7ec9-458c-8730-5c04dfaad56b" />


## Características Técnicas
* Lectura y procesamiento manual de archivos OBJ para la extracción de vértices y caras trianguladas.
* Trazado de aristas implementando el algoritmo de Bresenham.
* Dibujo de triángulos tanto en modo malla de alambre como en modo relleno.
* Centrado automático del modelo mediante el cálculo de su caja delimitadora.
* Escalado proporcional según la resolución de la ventana.
* Transformaciones tridimensionales de rotación en los tres ejes espaciales.
* Exportación directa del búfer de píxeles a imagen PNG.

## Requisitos
* Entorno de desarrollo de Rust y Cargo instalados.

## Instrucciones de Ejecución
Para compilar y ejecutar el proyecto, ejecute el siguiente comando en la terminal:

```bash
cargo run --release
```

## Controles de Usuario
* Flechas direccionales o teclas W, A, S, D: Rotación tridimensional del modelo.
* Tecla E: Acercamiento de la cámara.
* Tecla Q: Alejamiento de la cámara.
* Tecla M: Alternar entre modo malla de alambre y modo sólido.
* Tecla P: Guardar la captura actual en el archivo render.png.
* Barra espaciadora: Restablecer la cámara a la orientación inicial.
* Tecla Escape: Cerrar la aplicación.
