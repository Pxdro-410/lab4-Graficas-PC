# Laboratorio 4 - Renderizador de Modelos OBJ

## Autor
* Nombre: Pedro Caso
* Carné: 241286

## Descripción
Este proyecto consiste en un renderizador por software desarrollado en Rust para cargar y visualizar modelos tridimensionales en formato OBJ. El sistema recorre manualmente los vértices y las caras trianguladas del archivo, calculando las transformaciones necesarias para proyectar el modelo centrado y a escala adecuada en la pantalla.

## Demostración

pendiente

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
