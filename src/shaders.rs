use crate::Uniforms;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::prelude::*;

// Manually multiply a 4x4 matrix with a 4D vector (in homogeneous coordinates)
#[inline]
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

// Transform a normal vector using the model matrix
#[inline]
fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );

    let length = (transformed_normal.x * transformed_normal.x
        + transformed_normal.y * transformed_normal.y
        + transformed_normal.z * transformed_normal.z)
        .sqrt();

    if length > 0.0 {
        transformed_normal.x /= length;
        transformed_normal.y /= length;
        transformed_normal.z /= length;
    }

    transformed_normal
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position_vec4 = Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    let world_position_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);
    let world_position = Vector3::new(
        world_position_vec4.x,
        world_position_vec4.y,
        world_position_vec4.z,
    );

    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position_vec4);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position =
        Vector3::new(screen_position.x, screen_position.y, screen_position.z);

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
        world_position,
    }
}

pub fn earth_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let uv = fragment.position * 0.015;
    
    let noise1 = (uv.x * 3.0).sin() * (uv.y * 2.5).cos();
    let noise2 = (uv.x * 5.0 + 100.0).cos() * (uv.y * 4.0 + 50.0).sin();
    let pattern = (noise1 + noise2 * 0.5).abs();
    
    let ocean = Vector3::new(0.05, 0.3, 0.55);
    let land = Vector3::new(0.25, 0.5, 0.2);
    let mountain = Vector3::new(0.4, 0.35, 0.25);
    
    let base_color = if pattern > 0.7 {
        mountain
    } else if pattern > 0.4 {
        land
    } else {
        ocean
    };
    
    let cloud_pattern = ((uv.x * 7.0 + 200.0).sin() * (uv.y * 6.0 + 150.0).cos()).abs();
    let clouds = Vector3::new(1.0, 1.0, 1.0) * 0.3;
    
    let final_color = if cloud_pattern > 0.75 {
        base_color * 0.7 + clouds
    } else {
        base_color
    };
    
    final_color * 1.2
}

pub fn jupiter_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let uv = fragment.position * 0.01;
    
    let band_pos = uv.y * 15.0;
    let band = band_pos.sin() * 0.5 + 0.5;
    
    let turbulence = (uv.x * 10.0 + uv.y * 3.0).sin() * 
                     (uv.x * 7.0 - uv.y * 5.0).cos() * 0.3;
    
    let spot_x = uv.x - 300.0;
    let spot_y = uv.y - 250.0;
    let spot_dist = (spot_x * spot_x + spot_y * spot_y * 4.0).sqrt();
    let red_spot = if spot_dist < 50.0 {
        Vector3::new(0.4, 0.1, 0.05) * (1.0 - spot_dist / 50.0)
    } else {
        Vector3::new(0.0, 0.0, 0.0)
    };
    
    let color1 = Vector3::new(0.8, 0.6, 0.4);
    let color2 = Vector3::new(0.7, 0.4, 0.2);
    let color3 = Vector3::new(0.6, 0.35, 0.15);
    
    let band_value = band + turbulence;
    let base_color = if band_value > 0.66 {
        color1
    } else if band_value > 0.33 {
        color2
    } else {
        color3
    };
    
    (base_color + red_spot) * 1.1
}

pub fn namek_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let uv = fragment.position * 0.012;
    
    let noise1 = (uv.x * 4.0).sin() * (uv.y * 3.5).cos();
    let noise2 = (uv.x * 6.0 + 50.0).cos() * (uv.y * 5.5 + 30.0).sin();
    let pattern = (noise1 + noise2 * 0.6).abs();
    
    let water = Vector3::new(0.1, 0.5, 0.4);
    let grass = Vector3::new(0.4, 0.85, 0.4);
    let forest = Vector3::new(0.2, 0.65, 0.2);
    
    let base_color = if pattern > 0.65 {
        forest
    } else if pattern > 0.35 {
        grass
    } else {
        water
    };
    
    let glow = Vector3::new(0.3, 0.5, 0.3) * 0.2;
    
    (base_color + glow) * 1.4
}

pub fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let uv = fragment.position * 0.008;
    
    let noise1 = (uv.x * 3.0).sin() * (uv.y * 2.8).cos();
    let noise2 = (uv.x * 5.5 + 100.0).cos() * (uv.y * 4.5 + 80.0).sin();
    let noise3 = (uv.x * 8.0 - uv.y * 6.0).sin();
    
    let turbulence = (noise1 + noise2 * 0.5 + noise3 * 0.3).abs();
    
    let bright_yellow = Vector3::new(1.0, 1.0, 0.6);
    let orange = Vector3::new(1.0, 0.7, 0.2);
    let deep_orange = Vector3::new(1.0, 0.5, 0.1);
    
    let base_color = if turbulence > 0.7 {
        bright_yellow
    } else if turbulence > 0.4 {
        orange
    } else {
        deep_orange
    };
    
    base_color * 2.5
}

pub fn fragment_shaders(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_type: &str,
) -> Vector3 {
    match shader_type {
        "earth" => earth_shader(fragment, uniforms),
        "jupiter" => jupiter_shader(fragment, uniforms),
        "namek" => namek_shader(fragment, uniforms),
        "sun" => sun_shader(fragment, uniforms),
        _ => earth_shader(fragment, uniforms),
    }
}
