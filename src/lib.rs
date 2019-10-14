
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation, WebGlBuffer};
use js_sys::{Function};
use tuple::TupleElements;

#[wasm_bindgen]
pub struct Plotter {
    context: WebGl2RenderingContext,
    vertex_loca: u32,
    viewport_loca: Option<WebGlUniformLocation>,
    linewidth_loca: Option<WebGlUniformLocation>,
    color_loca: Option<WebGlUniformLocation>,
    transform_loca: Option<WebGlUniformLocation>,
    blendfactor_loca: Option<WebGlUniformLocation>,
    buffer_data: Vec<f32>,
    buffer: WebGlBuffer,
    function: Function,
    x_samples: u32
}

#[wasm_bindgen]
impl Plotter {

    #[wasm_bindgen(constructor)]
    pub fn new(f: Function) -> Result<Plotter, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("shader.vs")
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("shader.fs")
        )?;
        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let vertex_loca = context.get_attrib_location(&program, "aVertexPosition") as u32;
        let viewport_loca = context.get_uniform_location(&program, "uViewPort");
        let linewidth_loca = context.get_uniform_location(&program, "uLineWidth");
        let color_loca = context.get_uniform_location(&program, "uColor");
        let blendfactor_loca = context.get_uniform_location(&program, "uBlendFactor");
        let transform_loca = context.get_uniform_location(&program, "uTransform");
        
        //context.enable(WebGl2RenderingContext::BLEND);
        //context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        context.depth_mask(false);
        context.clear_color(1.0, 1.0, 0.9, 1.0);

        let buffer = context.create_buffer().ok_or("failed to create buffer")?;

        Ok(Plotter {
            context,
            vertex_loca,
            viewport_loca,
            linewidth_loca,
            color_loca,
            transform_loca,
            blendfactor_loca,
            buffer,
            buffer_data: vec![],
            function: f,
            x_samples: 100
        })
    }
    pub fn set_func(&mut self, f: Function) {
        self.function = f;
    }
    pub fn set_x_samples(&mut self, x_samples: u32) {
        self.x_samples = x_samples;
    }
    pub fn frame(&mut self, time: f64) -> Result<(), JsValue> {
        self.buffer_data.clear();

        let a = 0.1;
        let t = 0.001 * time;
        let dx = 1.0 / self.x_samples as f64;
        for n in 0 ..= self.x_samples {
            let x = dx * n as f64;
            let y = self.function.call2(&JsValue::NULL, &x.into(), &t.into())?;
            self.buffer_data.push(x as f32);
            self.buffer_data.push(y.as_f64().unwrap_or(0.) as f32);
            self.buffer_data.push(0.0);
        }

        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&self.buffer_data);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STREAM_DRAW,
            );
        }
        
        let plot_origin = (0.0, -1.0);
        let plot_size = (1.0, 2.0);

        self.context.vertex_attrib_pointer_with_i32(self.vertex_loca, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(self.vertex_loca);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.context.uniform1f(self.blendfactor_loca.as_ref(), 1.5);
        self.context.uniform2f(self.viewport_loca.as_ref(), 600.0, 400.0);
        self.context.uniform1f(self.linewidth_loca.as_ref(), 0.5);
        self.context.uniform4f(self.color_loca.as_ref(), 0.0, 0.0, 0.0, 1.0);
        self.context.uniform_matrix4fv_with_f32_array(self.transform_loca.as_ref(), false, &[
            2.0 / plot_size.0, 0.0, 0.0, 0.0,
            0.0, 2.0 / plot_size.1, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -2.0 * plot_origin.0 / plot_size.0 - 1.0,
            -2.0 * plot_origin.1 / plot_size.1 - 1.0, 0.0, 1.0
        ]);

        self.context.line_width(4.0);
        self.context.draw_arrays(
            WebGl2RenderingContext::LINE_STRIP,
            0,
            (self.buffer_data.len() / 3) as i32,
        );

        Ok(())
    }
}
pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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

pub fn link_program(
    context: &WebGl2RenderingContext,
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
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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

#[wasm_bindgen]
pub fn compile_expr(expr: String, args: String) -> Result<Vec<u8>, JsValue> {
    use bullet::builder::Builder;
    use bullet::vm::wasm::Wasm;
    
    let builder = Builder::new();
    let root = builder.parse(&expr).map_err(|e| format!("{:?}", e))?;

    let args: Vec<_> = args.split(" ").collect();
    let code = Wasm::compile(&root, &args).map_err(|e| format!("{:?}", e))?;
    Ok(code)
}
