# Laboratorio 4 - Renderizador de Modelos OBJ

## Autor
* Nombre: Pedro Caso
* Carné: 241286

## Descripción
Este proyecto consiste en un renderizador por software desarrollado en Rust para cargar y visualizar modelos tridimensionales en formato OBJ. El sistema recorre manualmente los vértices y las caras trianguladas del archivo, calculando las transformaciones necesarias para proyectar el modelo centrado y a escala adecuada en la pantalla.

## Demostración
#### Malla triangular y relleno
<img width="900" height="700" alt="2af5a58f-a6c0-4c78-975f-e5d8563a4941" src="https://github.com/user-attachments/assets/64d63756-6699-4aa1-a69f-69aeb6f7b9a2" />
<img width="900" height="700" alt="2e075523-7df1-4a14-8224-68355bec28b9" src="https://github.com/user-attachments/assets/b26bcb80-af5b-4104-9d58-1df7cdb819da" />
<img width="900" height="700" alt="6539af7c-f1ed-4607-bf3c-6a0043a1c41d" src="https://github.com/user-attachments/assets/d50a41cd-da4c-4588-9743-61ea9ce47949" />
<img width="900" height="700" alt="89d7dc05-dd35-403d-be9e-4e88153c7bac" src="https://github.com/user-attachments/assets/2bcd3009-cf29-4ccd-aead-004b4cb9ffbe" />

#### Solamente malla triangular
<img width="900" height="700" alt="5fa62708-c507-4e1d-95ac-2e7c2ac509ba" src="https://github.com/user-attachments/assets/e94ec161-c94b-4acf-9acb-2b8e037a6471" />
<img width="900" height="700" alt="798eaa3d-06a1-4910-bc80-c3f7bbc2b3a3" src="https://github.com/user-attachments/assets/ae095ee1-20fe-4e2a-a13e-5df7df50e67e" />

#### Solamente relleno de los triangulos
<img width="900" height="700" alt="8e341549-d3d6-4461-8bd9-0050677391b0" src="https://github.com/user-attachments/assets/68b4303b-6ab0-4d9d-bbdf-79dc17bd9dcc" />
<img width="900" height="700" alt="5f5c4aeb-ff38-44b2-ba40-dbf44c07cf1a" src="https://github.com/user-attachments/assets/0dfdfc13-ee49-4484-9905-d7deaa3e0656" />
<img width="900" height="700" alt="abd55b58-e1a2-44b0-a66e-845b44a11886" src="https://github.com/user-attachments/assets/57830a35-3b01-44f6-856a-0100708e37f7" />


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
