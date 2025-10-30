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
    let uv = fragment.position * 0.02;

    let pattern = (uv.x.sin() * uv.y.cos()).abs();

    let base_color = if pattern > 0.3 {
        Vector3::new(0.2, 0.7, 0.3)
    } else {
        Vector3::new(0.0, 0.3, 0.6)
    };

    let day_factor = ((uv.x * 0.5).cos() + 1.0) * 0.5;
    let shaded = base_color * (0.3 + 0.7 * day_factor);

    shaded
}

pub fn fragment_shaders(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_type: &str,
) -> Vector3 {
    match shader_type {
        "earth" => earth_shader(fragment, uniforms),
        _ => earth_shader(fragment, uniforms),
    }
}
