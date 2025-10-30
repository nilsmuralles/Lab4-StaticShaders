mod framebuffer;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use vertex::Vertex;
use camera::Camera;
use shaders::{vertex_shader, fragment_shaders};
use light::Light;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light, shader_type: &str) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let final_color = fragment_shaders(&fragment, uniforms, shader_type);
            
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            final_color,
        );
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Static Shaders")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(Vector3::new(5.0, 5.0, 5.0));

    let scale = 0.5;
    let rotation = Vector3::new(0.0, 0.0, 0.0);

    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    
    let vertex_array = obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(0, 0, 0, 255));

    while !window.window_should_close() {
        camera.process_input(&window);
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        let earth_translation = Vector3::new(0.0, 0.0, 0.0);
        let earth_model_matrix = create_model_matrix(earth_translation, scale, rotation);
        let earth_uniforms = Uniforms {
            model_matrix: earth_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &earth_uniforms, &vertex_array, &light, "earth");

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}
