use eframe::glow::{self, HasContext};
use glam::Mat4;

pub struct Renderer {
    program: glow::Program,
    vao: glow::VertexArray,
    buffer: glow::Buffer,
    count: i32,
}

impl Renderer {
    // All GL calls run on eframe's context-owning UI thread, including callbacks.
    pub fn new(gl: &glow::Context) -> Result<Self, String> {
        unsafe {
            let program = gl.create_program()?;
            for (kind, source) in [
                (glow::VERTEX_SHADER, include_str!("shaders/mesh.vert")),
                (glow::FRAGMENT_SHADER, include_str!("shaders/mesh.frag")),
            ] {
                let shader = gl.create_shader(kind)?;
                gl.shader_source(shader, source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    let error = gl.get_shader_info_log(shader);
                    gl.delete_shader(shader);
                    gl.delete_program(program);
                    return Err(format!("Rendering shader failed: {error}"));
                }
                gl.attach_shader(program, shader);
                gl.delete_shader(shader);
            }
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                let error = gl.get_program_info_log(program);
                gl.delete_program(program);
                return Err(format!("Rendering program failed: {error}"));
            }
            let vao = gl.create_vertex_array()?;
            let buffer = gl.create_buffer()?;
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            for index in 0..3 {
                gl.enable_vertex_attrib_array(index);
                gl.vertex_attrib_pointer_f32(index, 3, glow::FLOAT, false, 36, index as i32 * 12);
            }
            gl.bind_vertex_array(None);
            Ok(Self {
                program,
                vao,
                buffer,
                count: 0,
            })
        }
    }
    pub fn upload(&mut self, gl: &glow::Context, vertices: &[f32]) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            self.count = (vertices.len() / 9) as i32;
        }
    }
    pub fn paint(&self, gl: &glow::Context, mvp: Mat4, mode: i32, material: [f32; 3]) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.depth_mask(true);
            gl.clear_depth_f32(1.0);
            gl.clear(glow::DEPTH_BUFFER_BIT);
            gl.disable(glow::CULL_FACE);
            gl.disable(glow::BLEND);
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "mvp").as_ref(),
                false,
                &mvp.to_cols_array(),
            );
            gl.uniform_1_i32(gl.get_uniform_location(self.program, "mode").as_ref(), mode);
            gl.uniform_3_f32_slice(
                gl.get_uniform_location(self.program, "material").as_ref(),
                &material,
            );
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, self.count);
            gl.bind_vertex_array(None);
            gl.disable(glow::DEPTH_TEST);
            gl.enable(glow::BLEND);
        }
    }
    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.buffer);
            gl.delete_vertex_array(self.vao);
            gl.delete_program(self.program);
        }
    }
}
