// IFIHADMORETIME
// - Use standard numeric types like mint::Vec3 for Pos and mint::Vec2

use core::fmt::Debug;
use miniquad::graphics::FilterMode;
use miniquad::Bindings;
use miniquad::Buffer;
use miniquad::BufferLayout;
use miniquad::BufferType;
use miniquad::Comparison;
use miniquad::Context;
use miniquad::Pipeline;
use miniquad::PipelineParams;
use miniquad::Shader;
use miniquad::Texture;
use miniquad::TextureFormat;
use miniquad::TextureParams;
use miniquad::TextureWrap;
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

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::Less,
                depth_write: true,
                ..PipelineParams::default()
            },
        );

        Drawer {
            pipeline,
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
        }
    }

    pub fn draw<'a>(&self, ctx: &mut Context, imgs: impl Iterator<Item = (&'a Image, Pos)>) {
        ctx.apply_pipeline(&self.pipeline);
        for (img, pos) in imgs {
            ctx.apply_bindings(&Bindings {
                vertex_buffers: self.vertex_buffers.clone(),
                index_buffer: self.index_buffer,
                images: vec![img.tx],
            });
            ctx.apply_uniforms(&shader::Uniforms { offset: pos });
            ctx.draw(0, 6, 1);
        }
        ctx.end_render_pass();
    }
}

pub struct Image {
    tx: Texture,
}

impl Image {
    pub fn blank(ctx: &mut Context) -> Self {
        Self {
            tx: texture_from_rgb8(ctx, 1, 1, &[0x55, 0x55, 0x55]),
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
        Ok(Self {
            tx: texture_from_rgb8(ctx, metadata.width, metadata.height, &pixels),
        })
    }
}

fn texture_from_rgb8(ctx: &mut Context, width: u16, height: u16, bytes: &[u8]) -> Texture {
    Texture::from_data_and_format(
        ctx,
        bytes,
        TextureParams {
            width: width as u32,
            height: height as u32,
            format: TextureFormat::RGB8,
            wrap: TextureWrap::Repeat,
            filter: FilterMode::Linear,
        },
    )
}

impl Drop for Image {
    fn drop(&mut self) {
        self.tx.delete()
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Copy for Pos {}

impl core::ops::Sub<Self> for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl core::ops::Mul<f32> for Pos {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl core::ops::Add<Self> for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
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

    uniform vec3 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(vec3(pos * -offset.z, 0) + offset, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float3)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: super::Pos,
    }
}

fn dbug(t: impl Debug) -> String {
    format!("{:?}", t)
}
