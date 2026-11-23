//! Capa gráfica SGL (`sgl_*`, `ddraw_*`): los 46 envoltorios del EXE son `jmp [slot]` a punteros que `sgl_load_driver_dll`
//! rellena con las exportaciones de `ddraw_Dll.dll`. El cliente **no** transcribe esas rutinas: las sustituye este crate.
//! Esta tabla recoge para cada envoltorio su ranura y la rutina de p3-render que la cubre (`rust`; las 46 la tienen:
//! dibujo y texto, recortes `sgl_clip`, texturas `sgl_tex` y pantalla/Direct3D `sgl_screen`). Ver `reimpl/SGL_SUSTITUCION.md`.
//! Los envoltorios sí están transcritos y probados contra el x86 en `p3_core::lifted::sgl` (llaman al anfitrión por la ranura).

pub struct Slot {
    pub thunk: u32,
    pub name: &'static str,
    pub slot: u32,
    pub rust: Option<&'static str>,
}

pub const SLOTS: &[Slot] = &[
    Slot { thunk: 0x4BB130, name: "sgl_AddClipRect_4i", slot: 0x6DAA90, rust: Some("sgl_clip::SglClip::add_clip_rect") },
    Slot { thunk: 0x4BB200, name: "sgl_Begin3D", slot: 0x6DAA74, rust: Some("sgl_screen::begin_3d") },
    Slot { thunk: 0x4BB210, name: "sgl_ChangePalette", slot: 0x6DAA70, rust: Some("sgl_tex::TexStore::change_palette") },
    Slot { thunk: 0x4BB220, name: "sgl_ChangeTextureFlags", slot: 0x6DAA6C, rust: Some("sgl_tex::TexStore::change_flags") },
    Slot { thunk: 0x4BB240, name: "sgl_CleanUp", slot: 0x6DAA64, rust: Some("sgl_screen::SglScreen::clean_up") },
    Slot { thunk: 0x4BB280, name: "ddraw_copy_clipper", slot: 0x6DAA54, rust: Some("sgl_clip::SglClip::copy_clipper") },
    Slot { thunk: 0x4BB2A0, name: "sgl_CreateMemoryTexture", slot: 0x6DAA4C, rust: Some("sgl_tex::TexStore::create_memory_texture") },
    Slot { thunk: 0x4BB350, name: "sgl_Blt", slot: 0x6DAA20, rust: Some("sgl::draw_bitmap_rect / draw_palette_rect") },
    Slot { thunk: 0x4BB360, name: "sgl_Blt_RC", slot: 0x6DAA1C, rust: Some("sgl::draw_bitmap_rect / draw_palette_rect") },
    Slot { thunk: 0x4BB3C0, name: "sgl_DrawScene", slot: 0x6DAA10, rust: Some("sgl_screen::draw_scene_without_world") },
    Slot { thunk: 0x4BB3E0, name: "ui_render_text_at", slot: 0x6DAA08, rust: Some("sgl_font::Font::draw") },
    Slot { thunk: 0x4BB3F0, name: "sgl_DrawText_Rect", slot: 0x6DAA04, rust: Some("sgl_font::Font::draw") },
    Slot { thunk: 0x4BB440, name: "sgl_Finish3D", slot: 0x6DA9F0, rust: Some("sgl_screen::finish_3d") },
    Slot { thunk: 0x4BB460, name: "sgl_SwapBuffer", slot: 0x6DA9E8, rust: Some("sgl_screen::SglScreen::swap_buffer") },
    Slot { thunk: 0x4BB480, name: "sgl_FreeFont", slot: 0x6DA9E0, rust: Some("sgl_font::Font (se libera al soltar)") },
    Slot { thunk: 0x4BB4C0, name: "ddraw_free_rect_clipper", slot: 0x6DA9D0, rust: Some("sgl_clip::SglClip::free_clipper") },
    Slot { thunk: 0x4BB4D0, name: "sgl_FreeRectClipper", slot: 0x6DA9D0, rust: Some("sgl_clip::SglClip::free_clipper") },
    Slot { thunk: 0x4BB510, name: "sgl_GetProjectionData", slot: 0x6DA9BC, rust: Some("sgl_screen::projection_data") },
    Slot { thunk: 0x4BB520, name: "ddraw_get_rect_clipper", slot: 0x6DA9B8, rust: Some("sgl_clip::SglClip::create_clipper") },
    Slot { thunk: 0x4BB530, name: "sgl_CreateClipper", slot: 0x6DA9B4, rust: Some("sgl_clip::SglClip::create_clipper") },
    Slot { thunk: 0x4BB550, name: "sgl_GrabScreen", slot: 0x6DA9AC, rust: Some("sgl_screen::SglScreen::grab_screen") },
    Slot { thunk: 0x4BB570, name: "sgl_InitHardware", slot: 0x6DA9A0, rust: Some("sgl_screen::init_hardware") },
    Slot { thunk: 0x4BB590, name: "sgl_InitScreen", slot: 0x6DA99C, rust: Some("sgl_screen::SglScreen::init_screen") },
    Slot { thunk: 0x4BB5C0, name: "sgl_LoadMemoryTexture", slot: 0x6DA990, rust: Some("sgl_tex::TexStore::load_memory_texture") },
    Slot { thunk: 0x4BB6F0, name: "sgl_ReLoadMemoryTexture", slot: 0x6DA94C, rust: Some("sgl_tex::TexStore::reload_memory_texture") },
    Slot { thunk: 0x4BB740, name: "sgl_ResetClipper", slot: 0x6DA938, rust: Some("sgl_clip::SglClip::reset_clipper") },
    Slot { thunk: 0x4BB760, name: "ddraw_set_2d_clipping_mode", slot: 0x6DA930, rust: Some("sgl_clip::SglClip::set_mode") },
    Slot { thunk: 0x4BB770, name: "sgl_Get2dClippingMode", slot: 0x6DA92C, rust: Some("sgl_clip::SglClip::mode") },
    Slot { thunk: 0x4BB790, name: "sgl_SetArchive", slot: 0x6DA924, rust: Some("sgl_screen::SglScreen::set_archive") },
    Slot { thunk: 0x4BB7C0, name: "sgl_IsRectInClippingRegion", slot: 0x6DA904, rust: Some("sgl_clip::SglClip::is_rect_in") },
    Slot { thunk: 0x4BB800, name: "ddraw_set_clip_rect", slot: 0x6DA918, rust: Some("sgl_clip::SglClip::set_clip_rect") },
    Slot { thunk: 0x4BB810, name: "sgl_SetClipRect_4i", slot: 0x6DA914, rust: Some("sgl_clip::SglClip::set_clip_rect") },
    Slot { thunk: 0x4BB820, name: "ddraw_set_clip_rect_r", slot: 0x6DA910, rust: Some("sgl_clip::SglClip::set_clip_rect") },
    Slot { thunk: 0x4BB840, name: "sgl_GetClipRect_r", slot: 0x6DA908, rust: Some("sgl_clip::SglClip::clip") },
    Slot { thunk: 0x4BB850, name: "sgl_SetColorKey", slot: 0x6DA8F4, rust: Some("sgl_tex::TexStore::set_color_key") },
    Slot { thunk: 0x4BB860, name: "sgl_GetColorKey", slot: 0x6DA8F0, rust: Some("sgl_tex::TexStore::get_color_key") },
    Slot { thunk: 0x4BB880, name: "sgl_GetConstantColor", slot: 0x6DA8E8, rust: Some("sgl::BlitState::color") },
    Slot { thunk: 0x4BBA80, name: "sgl_StretchBitmapRect", slot: 0x6DA880, rust: Some("sgl::draw_scaled_rect") },
    Slot { thunk: 0x4BBA90, name: "sgl_StrLen", slot: 0x6DA87C, rust: Some("sgl_font::Font::text_width") },
    Slot { thunk: 0x4BBAA0, name: "sgl_SubClipRect_4i", slot: 0x6DA878, rust: Some("sgl_clip::SglClip::sub_clip_rect") },
    Slot { thunk: 0x4BBAB0, name: "sgl_SubClipRect_r", slot: 0x6DA874, rust: Some("sgl_clip::SglClip::sub_clip_rect") },
    Slot { thunk: 0x4BBAC0, name: "sgl_SwitchScreenMode", slot: 0x6DA86C, rust: Some("sgl_screen::SglScreen::switch_screen_mode") },
    Slot { thunk: 0x4BBAF0, name: "sgl_PrefetchTexture", slot: 0x6DA860, rust: Some("sgl_tex::TexStore::prefetch") },
    Slot { thunk: 0x4BBB00, name: "sgl_LockTexture", slot: 0x6DA85C, rust: Some("sgl_tex::TexStore::lock") },
    Slot { thunk: 0x4BBB10, name: "sgl_UnlockTexture", slot: 0x6DA864, rust: Some("sgl_tex::TexStore::unlock") },
    Slot { thunk: 0x4BBB30, name: "sgl_MessageBox", slot: 0x6DA854, rust: Some("sgl_screen::SglScreen::message_box") },
];

/// Rutina de sustitución del envoltorio en `thunk`, si existe.
pub fn replacement(thunk: u32) -> Option<&'static str> {
    SLOTS.iter().find(|s| s.thunk == thunk).and_then(|s| s.rust)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_is_consistent() {
        assert_eq!(SLOTS.len(), 46);
        let mut t: Vec<u32> = SLOTS.iter().map(|s| s.thunk).collect();
        t.sort();
        t.dedup();
        assert_eq!(t.len(), 46);
        assert_eq!(replacement(0x4BB350), Some("sgl::draw_bitmap_rect / draw_palette_rect"));
        assert_eq!(replacement(0x4BB200), Some("sgl_screen::begin_3d"));
        assert!(SLOTS.iter().all(|s| s.rust.is_some()), "ranura sin sustitución");
    }
}
