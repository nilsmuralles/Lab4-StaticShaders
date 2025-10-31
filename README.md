# 🪐 Laboratorio 4: Shaders Estáticos

Este proyecto de gráficos por computadora crea una escena de un sistema solar utilizando shaders estáticos para renderizar un sol y tres planetas: la Tierra, Namek y Júpiter. Cada cuerpo celeste se representa como una esfera y su apariencia se genera de forma procedural en los fragment shaders.

## 🖼️ Imágenes

<table>
  <tr>
    <td><img src="" alt="Imagen 1" width="400"/></td>
    <td><img src="" alt="Imagen 2" width="400"/></td>
  </tr>
  <tr>
    <td><img src="" alt="Imagen 3" width="400"/></td>
    <td><img src="" alt="Imagen 4" width="400"/></td>
  </tr>
</table>

## 🚀 Cómo ejecutar el proyecto

Para compilar y ejecutar este proyecto, necesitarás tener [Rust](https://www.rust-lang.org/tools/install) instalado. Una vez que tengas Rust, sigue estos pasos:

1.  Clona el repositorio:
    ```bash
    git clone https://github.com/nilsmuralles/Lab4-StaticShaders.git
    ```
2.  Navega al directorio del proyecto:
    ```bash
    cd Lab4-StaticShaders
    ```
3.  Ejecuta el proyecto con Cargo:
    ```bash
    cargo run --release
    ```

## 🪐 Planetas y Shaders

A continuación se detallan los shaders y uniforms utilizados para cada planeta en la escena.

### ☀️ Sol

-   **Vertex Shader**: Se utiliza un vertex shader estándar para transformar los vértices del modelo de la esfera al espacio de la pantalla.
-   **Fragment Shader**: `sun_shader`
    -   Este shader genera una apariencia de estrella con turbulencia. Utiliza una combinación de funciones de seno y coseno para crear un patrón de ruido que simula la superficie del sol.
    -   Los colores varían entre amarillo brillante, naranja y naranja oscuro para dar la impresión de una superficie solar activa.
-   **Uniforms**:
    -   `model_matrix`: Matriz para escalar, rotar y trasladar la esfera del sol.
    -   `view_matrix`: Matriz de la cámara.
    -   `projection_matrix`: Matriz de proyección.
    -   `viewport_matrix`: Matriz para transformar las coordenadas a la pantalla.

### 🌍 Tierra

-   **Vertex Shader**: El mismo vertex shader que el sol.
-   **Fragment Shader**: `earth_shader`
    -   Este shader genera una apariencia similar a la de la Tierra con océanos, tierra, montañas y nubes.
    -   Utiliza patrones de ruido para diferenciar entre agua, tierra y montañas.
    -   Se añade una capa adicional de ruido para simular las nubes.
-   **Uniforms**:
    -   `model_matrix`: Matriz para la órbita y rotación de la Tierra.
    -   `view_matrix`: Matriz de la cámara.
    -   `projection_matrix`: Matriz de proyección.
    -   `viewport_matrix`: Matriz para transformar las coordenadas a la pantalla.

### 🪐 Namek

-   **Vertex Shader**: El mismo vertex shader que el sol.
-   **Fragment Shader**: `namek_shader`
    -   Este shader crea un planeta con un aspecto similar al planeta Namek de Dragon Ball Z.
    -   La superficie se genera con patrones de ruido que distinguen entre agua, hierba y bosques.
    -   Los colores predominantes son verdes y azules para simular la apariencia de Namek.
-   **Uniforms**:
    -   `model_matrix`: Matriz para la órbita y rotación de Namek.
    -   `view_matrix`: Matriz de la cámara.
    -   `projection_matrix`: Matriz de proyección.
    -   `viewport_matrix`: Matriz para transformar las coordenadas a la pantalla.

### 🪐 Júpiter

-   **Vertex Shader**: El mismo vertex shader que el sol.
-   **Fragment Shader**: `jupiter_shader`
    -   Este shader simula la apariencia de Júpiter, incluyendo sus características bandas de nubes y la Gran Mancha Roja.
    -   Las bandas se generan con una función de seno y se añade turbulencia para un aspecto más realista.
    -   La Gran Mancha Roja se crea como una elipse en una posición específica del planeta.
-   **Uniforms**:
    -   `model_matrix`: Matriz para la órbita y rotación de Júpiter.
    -   `view_matrix`: Matriz de la cámara.
    -   `projection_matrix`: Matriz de proyección.
    -   `viewport_matrix`: Matriz para transformar las coordenadas a la pantalla.
