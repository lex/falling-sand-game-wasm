use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
    WebGlUniformLocation,
};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct ProgramInfo {
    pub a_vertex_position: i32,
    pub a_texture_coordinate: i32,
    pub u_sampler: Option<WebGlUniformLocation>,
    pub program: Option<WebGlProgram>,
}

pub struct Buffers {
    pub vertex_buffer: Option<WebGlBuffer>,
    pub index_buffer: Option<WebGlBuffer>,
    pub texture_coordinate_buffer: Option<WebGlBuffer>,
    pub texture: Option<WebGlTexture>,
}

pub struct Renderer {
    pub context: Option<WebGlRenderingContext>,
    pub program_info: Option<ProgramInfo>,
    pub buffers: Option<Buffers>,
}

impl Renderer {
    pub fn setup_webgl(&mut self) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        self.context = Some(context);

        let program_info = self.create_program().unwrap();
        self.program_info = Some(program_info);


        let buffers = self.create_buffers().unwrap();
        self.buffers = Some(buffers);

        // bind and shit

        // vertices
        log!("webgl initialized");
        Ok(())
    }

    fn create_buffers(&self) -> Result<Buffers, String> {
        let context = self.context.as_ref().unwrap();
        let vertices: [f32; 8] = [
            -1.0, 1.0, // upper left
            -1.0, -1.0, // lower left
            1.0, -1.0,  // lower right
            1.0, 1.0  // upper right
        ];

        let indices: [u8; 6] = [3, 2, 1, 3, 1, 0];

        let texture_coordinates = [
            0.0, 0.0, // lower left
            0.0, 1.0, // lower right
            1.0, 1.0, // upper right
            1.0, 0.0, // upper left
        ];

        // vertices
        let vertex_buffer = context.create_buffer().ok_or("failed to create buffer")?;

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        // indices
        let index_buffer = context.create_buffer().ok_or("failed to create buffer")?;

        context.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        unsafe {
            let index_array = js_sys::Uint8Array::view(&indices);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        // texture
        let texture_coordinate_buffer = context.create_buffer().ok_or("failed to create buffer")?;

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&texture_coordinate_buffer),
        );

        unsafe {
            let texture_coordinates_array = js_sys::Float32Array::view(&texture_coordinates);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &texture_coordinates_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let texture = context.create_texture().ok_or("failed to create texture")?;

        let buffers = Buffers {
            vertex_buffer: Some(vertex_buffer),
            index_buffer: Some(index_buffer),
            texture_coordinate_buffer: Some(texture_coordinate_buffer),
            texture: Some(texture),
        };
        log!("created buffers");
        return Ok(buffers);
    }

    // pub fn render(&self, framebuffer: &[u8]) {
    pub fn render(&self, framebuffer: &[u8], width: u32, height: u32) {
        let context = self.context.as_ref().unwrap();
        let buffers = self.buffers.as_ref().unwrap();
        let program_info = self.program_info.as_ref().unwrap();

        let vertex_buffer = buffers.vertex_buffer.as_ref().unwrap();
        let index_buffer = buffers.index_buffer.as_ref().unwrap();
        let texture_coordinate_buffer = buffers.texture_coordinate_buffer.as_ref().unwrap();
        let texture = buffers.texture.as_ref().unwrap();

        let vertex_position = program_info.a_vertex_position;
        let texture_coordinate = program_info.a_texture_coordinate;
        let u_sampler  = program_info.u_sampler.as_ref();

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        context.vertex_attrib_pointer_with_i32(
            vertex_position as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(vertex_position as u32);

        // indices
        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // texture coordinates
        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&texture_coordinate_buffer),
        );
        context.vertex_attrib_pointer_with_i32(
            texture_coordinate as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(texture_coordinate as u32);

        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

        let result = context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGB as i32,
            width as i32,
            height as i32,
            0,
            WebGlRenderingContext::RGB,
            WebGlRenderingContext::UNSIGNED_BYTE,
            Some(framebuffer),
        );

        context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_MIN_FILTER, WebGlRenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_MAG_FILTER, WebGlRenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_WRAP_S, WebGlRenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_WRAP_T, WebGlRenderingContext::CLAMP_TO_EDGE as i32);

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        context.active_texture(WebGlRenderingContext::TEXTURE0);
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

        if let Some(u) = &u_sampler {
            context.uniform1i(Some(u), 0);
        }

        let indices_len = 6;
        context.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, indices_len as i32, WebGlRenderingContext::UNSIGNED_BYTE, 0);
    }

    fn create_program(&self) -> Result<ProgramInfo, String> {
        let context = self.context.as_ref().unwrap();

        let vertex_shader = self.compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec2 aVertexPosition;
            attribute vec2 aTextureCoord;

            varying lowp vec4 vColor;
            varying highp vec2 vTextureCoord;
            varying highp vec2 vVertexPosition;

            void main() {
                gl_Position = vec4(aVertexPosition, 1.0, 1.0);
                vTextureCoord = aTextureCoord;
                vVertexPosition = aVertexPosition;
            }
        "#,
        )?;

        let fragment_shader = self.compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            varying highp vec2 vTextureCoord;
            varying highp vec2 vVertexPosition;
            uniform sampler2D uSampler;

            void main() {
                // gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
                // gl_FragColor = vec4(vTextureCoord, 0.0, 1.0);
                // gl_FragColor = vec4(vVertexPosition, 0.0, 1.0);
                // gl_FragColor = vec4(vTextureCoord.x, vTextureCoord.y, 0.0, 0.0);
                gl_FragColor = texture2D(uSampler, vTextureCoord);
            }
        "#,
        )?;

        let program = self.link_program(&context, &vertex_shader, &fragment_shader)?;

        context.use_program(Some(&program));

        let a_vertex_position = context.get_attrib_location(&program, "aVertexPosition");
        let a_texture_coordinate = context.get_attrib_location(&program, "aTextureCoord");
        let u_sampler = context.get_uniform_location(&program, "uSampler");

        let program_info = ProgramInfo {
            program: Some(program),
            a_vertex_position: a_vertex_position,
            a_texture_coordinate: a_texture_coordinate,
            u_sampler: u_sampler,
        };

        return Ok(program_info);
    }

    fn compile_shader(
        &self,
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        &self,
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}