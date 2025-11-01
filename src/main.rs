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
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Static shaders")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    
    let mut camera = Camera::new(
        Vector3::new(0.0, 8.0, 20.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(Vector3::new(0.0, 0.0, 0.0));

    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(5, 5, 15, 255));

    let mut time: f32 = 0.0;

    while !window.window_should_close() {
        camera.process_input(&window);
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        time += 0.005;

        let sun_scale = 1.5;
        let sun_rotation = Vector3::new(0.0, time * 0.3, 0.0); // Slow rotation
        let sun_translation = Vector3::new(0.0, 0.0, 0.0);
        let sun_model_matrix = create_model_matrix(sun_translation, sun_scale, sun_rotation);
        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &sun_uniforms, &vertex_array, &light, "sun");

        let earth_orbit_radius = 4.0;
        let earth_orbit_speed = 1.0;
        let earth_angle = time * earth_orbit_speed;
        let earth_x = earth_orbit_radius * earth_angle.cos();
        let earth_z = earth_orbit_radius * earth_angle.sin();
        let earth_translation = Vector3::new(earth_x, 0.0, earth_z);
        let earth_scale = 0.8;
        let earth_rotation = Vector3::new(0.0, time * 2.0, 0.0); // Self rotation
        let earth_model_matrix = create_model_matrix(earth_translation, earth_scale, earth_rotation);
        let earth_uniforms = Uniforms {
            model_matrix: earth_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &earth_uniforms, &vertex_array, &light, "earth");

        let moon_orbit_radius = 1.5;
        let moon_orbit_speed = 3.0;
        let moon_angle = time * moon_orbit_speed;
        let moon_x = earth_x + moon_orbit_radius * moon_angle.cos();
        let moon_y = moon_orbit_radius * 0.3 * moon_angle.sin(); // Vertical component
        let moon_z = earth_z + moon_orbit_radius * moon_angle.sin();
        let moon_translation = Vector3::new(moon_x, moon_y, moon_z);
        let moon_scale = 0.2;
        let moon_rotation = Vector3::new(0.0, time * 1.0, 0.0);
        let moon_model_matrix = create_model_matrix(moon_translation, moon_scale, moon_rotation);
        let moon_uniforms = Uniforms {
            model_matrix: moon_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &moon_uniforms, &vertex_array, &light, "moon");

        let namek_orbit_radius = 7.0;
        let namek_orbit_speed = 0.7;
        let namek_angle = time * namek_orbit_speed;
        let namek_x = namek_orbit_radius * namek_angle.cos();
        let namek_z = namek_orbit_radius * namek_angle.sin();
        let namek_translation = Vector3::new(namek_x, 0.0, namek_z);
        let namek_scale = 0.75;
        let namek_rotation = Vector3::new(0.0, time * 1.8, 0.0);
        let namek_model_matrix = create_model_matrix(namek_translation, namek_scale, namek_rotation);
        let namek_uniforms = Uniforms {
            model_matrix: namek_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &namek_uniforms, &vertex_array, &light, "namek");

        let jupiter_orbit_radius = 10.5;
        let jupiter_orbit_speed = 0.4;
        let jupiter_angle = time * jupiter_orbit_speed;
        let jupiter_x = jupiter_orbit_radius * jupiter_angle.cos();
        let jupiter_z = jupiter_orbit_radius * jupiter_angle.sin();
        let jupiter_translation = Vector3::new(jupiter_x, 0.0, jupiter_z);
        let jupiter_scale = 1.3;
        let jupiter_rotation = Vector3::new(0.0, time * 3.0, 0.0)
        let jupiter_model_matrix = create_model_matrix(jupiter_translation, jupiter_scale, jupiter_rotation);
        let jupiter_uniforms = Uniforms {
            model_matrix: jupiter_model_matrix,
            view_matrix: view_matrix.clone(),
            projection_matrix: projection_matrix.clone(),
            viewport_matrix: viewport_matrix.clone(),
        };
        render(&mut framebuffer, &jupiter_uniforms, &vertex_array, &light, "jupiter");

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}
