use std::borrow::Borrow;
use std::ops::Deref;


mod rect;
pub use rect::*;

use ffi::DrawRectangleRoundedLines;
use raylib_ffi as ffi;
use raylib_ffi::enums::*;
pub use raylib_ffi::enums::KeyboardKey as Key;

pub use ffi::Color;
pub use raylib_ffi::colors;

pub struct Graphics(());
pub struct DrawHandle<'a>{
    pub g: &'a mut Graphics,
}
impl<'a> Deref for DrawHandle<'a> {
    type Target = &'a mut Graphics;

    fn deref(&self) -> &Self::Target {
        &self.g
    }
}
impl<'a> DrawHandle<'a> {
    pub fn time(&self)->f64{
        unsafe{
            ffi::GetTime()
        }
    }
    pub fn clear_background(&mut self, color: Color){
        unsafe{
            ffi::ClearBackground(color)
        }
    }
    pub fn draw_fps(&mut self, pos_x: i32, pos_y: i32){
        unsafe{
            ffi::DrawFPS(pos_x, pos_y)
        }
    }
    pub fn draw_circle(&mut self, pos_x: i32, pos_y: i32, radius: f32, color: Color){
        unsafe{
            ffi::DrawCircle(pos_x, pos_y, radius, color)
        }
    }
    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color){
        unsafe{
            ffi::DrawText(ffi::rl_str!(text), pos_x, pos_y, font_size, color)
        }
    }
    pub fn draw_text_ex(&mut self, font: PrettyFont, shader: &Shader,text: &str, pos: Vec2, scale: f32, color: Color){
        unsafe{
            ffi::BeginShaderMode(shader.0);
            ffi::DrawTextEx(font.font, ffi::rl_str!(text), pos.into(), scale, 0.0, color);
            ffi::EndShaderMode();
        }
    }
    pub fn draw_rect_rounded_lines(&mut self, rect: Rect, roundness: f32, segments: i32, line_thickness: f32, color: Color){
        unsafe{
            DrawRectangleRoundedLines(rect.into(), roundness, segments, line_thickness, color)
        }
    }
}

impl Graphics{
    pub fn window_should_close(&mut self)->bool{
        unsafe{
            ffi::WindowShouldClose()
        }
    }
    pub fn init(width: i32, height: i32, title: &str)->Self{
        unsafe{
            ffi::SetConfigFlags(ffi::enums::ConfigFlags::Msaa4xHint as u32);
            ffi::InitWindow(width, height, ffi::rl_str!(title));
            ffi::SetTargetFPS(60);
        }
        Graphics(())
    }
    pub fn draw_frame<F: FnMut(&mut DrawHandle)>(&mut self, mut f: F){
        unsafe{
            ffi::BeginDrawing();
        }
        f(&mut DrawHandle{
            g: self
        });
        unsafe{
            ffi::EndDrawing();
        }
    }
    pub fn is_key_pressed(&self, key: raylib_ffi::enums::KeyboardKey)->bool{
        unsafe{
            ffi::IsKeyPressed(key as i32)
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe{
            println!("DROP: Graphics");
            ffi::CloseWindow();
        }
    }
}

pub struct FileData{
    ptr: *mut u8,
    size: u32,
}
impl FileData {
    pub fn load(path: &str)->Option<FileData>{
        let mut size = 0;
        let ptr = unsafe {ffi::LoadFileData(ffi::rl_str!(path), &mut size)};
        if ptr.is_null(){
            None
        }else{
            Some(Self { ptr, size })
        }
    }
}
impl Drop for FileData {
    fn drop(&mut self) {
        println!("DROP: FileData");
        unsafe {ffi::UnloadFileData(self.ptr)}
    }
}

#[derive(Copy, Clone)]
pub struct PrettyFont{
    font: ffi::Font,
}
impl Graphics {
    pub fn font_from_file(&mut self, file: &FileData) -> PrettyFont{
        let base_size = 50;
        let font = unsafe {
            let mut sdf_font = ffi::Font{
                baseSize: base_size,
                glyphCount: 95,
                glyphPadding: 0,
                glyphs: ffi::LoadFontData(file.ptr, file.size as i32, base_size, 0 as *mut i32, 0, FontType::Sdf as i32),
                texture: ffi::Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0, },
                recs: 0 as *mut ffi::Rectangle,
            };
            let atlas = ffi::GenImageFontAtlas(sdf_font.glyphs, &mut sdf_font.recs, 95, base_size, 0, 1);
            sdf_font.texture = ffi::LoadTextureFromImage(atlas);
            ffi::SetTextureFilter(sdf_font.texture, TextureFilter::Bilinear as i32);    // Required for SDF font
            ffi::UnloadImage(atlas);
            sdf_font
        };
        PrettyFont { font }
    }

    pub fn draw_text(&mut self, font: &PrettyFont, text: &str, pos: ffi::Vector2, size: f32, color: ffi::Color){
        unsafe{ffi::DrawTextEx(font.font, 
                   ffi::rl_str!(text), 
                   pos,
                   size,
                   0.0,
                   color)};
    }

    pub fn measure(&self, font: impl Borrow<PrettyFont>, text: &str, size: f32) -> Vec2{
        unsafe{
            ffi::MeasureTextEx(font.borrow().font, ffi::rl_str!(text), size, 0.0)
        }.into()
    }
}

pub struct Shader(ffi::Shader);
impl Drop for Shader {
    fn drop(&mut self) {
        println!("DROP: Shader");
        unsafe{
            ffi::UnloadShader(self.0);
        }
    }
}

impl Graphics {
    pub fn load_shader(&mut self, vertex_shader_path: Option<&str>, frgment_shader_path: &str)-> Shader {
        unsafe{
            Shader(ffi::LoadShader(vertex_shader_path.map(|x|ffi::rl_str!(x)).unwrap_or(0 as *const i8), ffi::rl_str!(frgment_shader_path)))
        }
    }
    
}
