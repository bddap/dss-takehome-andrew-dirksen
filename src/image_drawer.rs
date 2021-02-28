use core::fmt::Debug;
use miniquad::Bindings;
use miniquad::Buffer;
use miniquad::BufferLayout;
use miniquad::BufferType;
use miniquad::Context;
use miniquad::Pipeline;
use miniquad::Shader;
use miniquad::Texture;
use miniquad::VertexAttribute;
use miniquad::VertexFormat;

pub struct Drawer {
    pipeline: Pipeline,
    vertex_buffers: Vec<Buffer>,
    index_buffer: Buffer,
}

impl Drawer {
    pub fn new(ctx: &mut Context) -> Self {
        let s = 0.15;
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos: Vec2 { x: -s, y: -s }, uv: Vec2 { x: 0., y: 1. } },
            Vertex { pos: Vec2 { x:  s, y: -s }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos: Vec2 { x:  s, y:  s }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos: Vec2 { x: -s, y:  s }, uv: Vec2 { x: 0., y: 0. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::META);

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Drawer {
            pipeline,
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
        }
    }

    pub fn draw<'a>(&self, ctx: &mut Context, imgs: impl Iterator<Item = (&'a Image, Pos, Scale)>) {
        ctx.apply_pipeline(&self.pipeline);
        for (img, pos, scale) in imgs {
            ctx.apply_bindings(&Bindings {
                vertex_buffers: self.vertex_buffers.clone(),
                index_buffer: self.index_buffer,
                images: vec![img.tx],
            });
            ctx.apply_uniforms(&shader::Uniforms {
                offset: (pos.x, pos.y),
            });
            ctx.draw(0, 6, 1);
        }
        ctx.end_render_pass();
    }

    pub fn draw_single(&self, ctx: &mut Context, img: &Image, pos: Pos, scale: Scale) {
        self.draw(ctx, Some((img, pos, scale)).into_iter())
    }
}

pub struct Image {
    tx: Texture,
}

impl Image {
    pub fn from_rgba8(ctx: &mut Context, width: u16, height: u16, bytes: &[u8]) -> Self {
        Self {
            tx: Texture::from_rgba8(ctx, width, height, &bytes),
        }
    }

    pub fn decode_jpg(ctx: &mut Context, bs: &[u8]) -> Result<Self, String> {
        use jpeg_decoder::{Decoder, PixelFormat};
        let mut decoder = Decoder::new(std::io::Cursor::new(bs));
        let pixels = decoder.decode().map_err(dbug)?;
        let metadata = decoder
            .info()
            .expect("metadata should be available after calling decode");
        if metadata.pixel_format != PixelFormat::RGB24 {
            return Err("unsupported image format".to_string());
        }

        let mut rgba_px: Vec<u8> = Vec::with_capacity(pixels.len() / 3 * 4);
        for chunk in pixels.chunks(3) {
            rgba_px.extend(chunk);
            rgba_px.push(0); // add the alpha channel
        }
        Ok(Self::from_rgba8(
            ctx,
            metadata.width,
            metadata.height,
            &rgba_px,
        ))
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        self.tx.delete()
    }
}

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub struct Scale {
    pub big: bool,
}

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const META: ShaderMeta = ShaderMeta {
        images: &["tex"],
        uniforms: UniformBlockLayout {
            uniforms: &[("offset", UniformType::Float2)],
        },
    };

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}

fn dbug(t: impl Debug) -> String {
    format!("{:?}", t)
}
