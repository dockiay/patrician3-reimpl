//! Generado por `reimpl/tools/ui_rt_hand.py`; no editar a mano.
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;

pub type HandFn = fn(&mut dyn Mem, &mut dyn OpsHost, u32, u32) -> u32;

fn w_401300(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::view_reset_scroll_and_scale(m, h, this, esp)
}
fn w_402310(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_screen_host_handle_input_code(m, h, this, esp)
}
fn w_423840(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_screen_update_hud_and_hotkeys(m, h, this, esp)
}
fn w_4247a0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_screen_on_key_down(m, h, this, esp)
}
fn w_428e00(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::game_screen_place_x_labels(m, h, this, esp)
}
fn w_428f50(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::game_screen_place_y_labels(m, h, this, esp)
}
fn w_4298f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_screen_on_mouse_up(m, h, this, esp)
}
fn w_429950(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_screen_on_mouse_down(m, h, this, esp)
}
fn w_42b920(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::game_screen_pop_notice(m, h, this, esp);
    0
}
fn w_42be40(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::game_screen_pop_notice_b(m, h, this, esp);
    0
}
fn w_42d230(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::game_screen_update_speed_text(m, h, this, esp);
    0
}
fn w_45c1f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::number_input_field_reset(m, h, this, esp);
    0
}
fn w_45c230(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::number_input_field_normalize_text(m, h, this, esp)
}
fn w_45c300(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::number_input_field_handle_key_input(m, h, this, esp)
}
fn w_45c930(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::number_input_field_set_value(m, h, this, esp)
}
fn w_461990(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::scroll_area_on_mouse_event(m, h, this, esp)
}
fn w_46cd10(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::slider_button_on_mouse_event(m, h, this, esp)
}
fn w_47d1f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::list_screen_reset(m, h, this, esp)
}
fn w_48d420(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::dialogs::confirm::ship_side_menu_confirm_icon_choice(m, h, this, esp);
    0
}
fn w_495bb0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::window_release_unless_active(m, h, this, esp)
}
fn w_4af740(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::anim_button_set_animation(m, h, this, esp)
}
fn w_4af910(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::anim_button_consume_click(m, h, this, esp)
}
fn w_4af920(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::anim_button_on_mouse_event(m, h, this, esp);
    0
}
fn w_4b1720(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::window_handle_mouse_event_mode(m, h, this, esp);
    0
}
fn w_4b2280(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::window_hit_test_point_copy(m, h, this, esp)
}
fn w_4b22b0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::window_hit_test_point(m, h, this, esp)
}
fn w_4b2320(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::window_on_mouse_press(m, h, this, esp)
}
fn w_4b2360(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::window_on_mouse_release(m, h, this, esp)
}
fn w_4b6650(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_on_mouse_move(m, h, this, esp)
}
fn w_4b6a90(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_on_mouse_wheel(m, h, this, esp)
}
fn w_4b6b70(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_on_mouse_event_noop(m, h, this, esp);
    0
}
fn w_4b6b80(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_on_mouse_leave(m, h, this, esp)
}
fn w_4b6cc0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_get_capture_child_window(m, h, this, esp)
}
fn w_4b7240(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_hit_test_children(m, h, this, esp)
}
fn w_4b72f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_set_dragged_child_by_window(m, h, this, esp)
}
fn w_4b7370(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_set_dragged_child(m, h, this, esp)
}
fn w_4b77f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_get_cursor_position(m, h, this, esp);
    0
}
fn w_4b7bf0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_begin_drag_child(m, h, this, esp)
}
fn w_4b7c80(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_window_end_drag_child(m, h, this, esp)
}
fn w_4b9430(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_list_get_last_cursor_position(m, h, this, esp);
    0
}
fn w_4b9540(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::screen_list_get_current_cursor(m, h, this, esp)
}
fn w_4b9d30(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::sgl_scrollbar_handle_scroll_keys(m, h, this, esp)
}
fn w_4babe0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::sgl2d_scrollbar_hit_test_region(m, h, this, esp)
}
fn w_4bacd0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::sgl2d_scrollbar_hit_test_xy(m, h, this, esp)
}
fn w_4bd340(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::clamp_cursor_to_screen(m, h, this, esp);
    0
}
fn w_4c0e80(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::game_window_on_set_cursor(m, h, this, esp)
}
fn w_4c0f50(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::virtual_key_to_ascii(m, h, this, esp)
}
fn w_4c49e0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::sgl2d_status_bar_on_mouse_event(m, h, this, esp)
}
fn w_4c5850(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::ptr_array_get_checked(m, h, this, esp)
}
fn w_4c6b90(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::button_base_on_mouse_event(m, h, this, esp);
    0
}
fn w_4c8000(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::combo_box_on_mouse_event(m, h, this, esp);
    0
}
fn w_4c9120(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::combo_box_close_dropdown_on_focus_lost(m, h, this, esp)
}
fn w_4c9f80(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::edit_box_on_mouse_event(m, h, this, esp)
}
fn w_4ca110(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::edit_box_handle_key(m, h, this, esp)
}
fn w_4ca640(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::edit_box_on_focus_gained(m, h, this, esp)
}
fn w_4ca6f0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::edit_box_on_focus_lost(m, h, this, esp)
}
fn w_4cb2a0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::list_box_on_mouse_event(m, h, this, esp)
}
fn w_4cb6c0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::list_box_handle_key(m, h, this, esp)
}
fn w_4cf890(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::slider_on_mouse_event(m, h, this, esp)
}
fn w_4cfb30(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::slider_handle_key(m, h, this, esp)
}
fn w_4d0b20(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::viper_slider_on_mouse_release(m, h, this, esp)
}
fn w_4d1610(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::state_switch_handle_mouse_event(m, h, this, esp);
    0
}
fn w_4d16a0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_input::state_switch_on_click_advance(m, h, this, esp);
    0
}
fn w_4f3fa0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_dtors::merchant_destructor(m, h, this, esp)
}
fn w_4f6050(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_dtors::merchant_base_destructor(m, h, this, esp)
}
fn w_549040(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::order_screen_refresh_slots(m, h, this, esp)
}
fn w_59a1e0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_hud::scene_refresh_clip_and_toggle(m, h, this, esp)
}
fn w_5f2e90(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::dialogs::confirm::siege_dialog_confirm_deliver_goods(m, h, this, esp);
    0
}
fn w_5f2f90(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::dialogs::confirm::siege_dialog_confirm_order_attack(m, h, this, esp);
    0
}
fn w_5f30b0(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::dialogs::confirm::siege_dialog_confirm_surrender(m, h, this, esp);
    0
}
fn w_633520(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    p3_core::ui_dtors::siege_record_destructor(m, h, this, esp)
}

pub static HAND: [(u32, HandFn); 71] = [
    (0x401300, w_401300),
    (0x402310, w_402310),
    (0x423840, w_423840),
    (0x4247a0, w_4247a0),
    (0x428e00, w_428e00),
    (0x428f50, w_428f50),
    (0x4298f0, w_4298f0),
    (0x429950, w_429950),
    (0x42b920, w_42b920),
    (0x42be40, w_42be40),
    (0x42d230, w_42d230),
    (0x45c1f0, w_45c1f0),
    (0x45c230, w_45c230),
    (0x45c300, w_45c300),
    (0x45c930, w_45c930),
    (0x461990, w_461990),
    (0x46cd10, w_46cd10),
    (0x47d1f0, w_47d1f0),
    (0x48d420, w_48d420),
    (0x495bb0, w_495bb0),
    (0x4af740, w_4af740),
    (0x4af910, w_4af910),
    (0x4af920, w_4af920),
    (0x4b1720, w_4b1720),
    (0x4b2280, w_4b2280),
    (0x4b22b0, w_4b22b0),
    (0x4b2320, w_4b2320),
    (0x4b2360, w_4b2360),
    (0x4b6650, w_4b6650),
    (0x4b6a90, w_4b6a90),
    (0x4b6b70, w_4b6b70),
    (0x4b6b80, w_4b6b80),
    (0x4b6cc0, w_4b6cc0),
    (0x4b7240, w_4b7240),
    (0x4b72f0, w_4b72f0),
    (0x4b7370, w_4b7370),
    (0x4b77f0, w_4b77f0),
    (0x4b7bf0, w_4b7bf0),
    (0x4b7c80, w_4b7c80),
    (0x4b9430, w_4b9430),
    (0x4b9540, w_4b9540),
    (0x4b9d30, w_4b9d30),
    (0x4babe0, w_4babe0),
    (0x4bacd0, w_4bacd0),
    (0x4bd340, w_4bd340),
    (0x4c0e80, w_4c0e80),
    (0x4c0f50, w_4c0f50),
    (0x4c49e0, w_4c49e0),
    (0x4c5850, w_4c5850),
    (0x4c6b90, w_4c6b90),
    (0x4c8000, w_4c8000),
    (0x4c9120, w_4c9120),
    (0x4c9f80, w_4c9f80),
    (0x4ca110, w_4ca110),
    (0x4ca640, w_4ca640),
    (0x4ca6f0, w_4ca6f0),
    (0x4cb2a0, w_4cb2a0),
    (0x4cb6c0, w_4cb6c0),
    (0x4cf890, w_4cf890),
    (0x4cfb30, w_4cfb30),
    (0x4d0b20, w_4d0b20),
    (0x4d1610, w_4d1610),
    (0x4d16a0, w_4d16a0),
    (0x4f3fa0, w_4f3fa0),
    (0x4f6050, w_4f6050),
    (0x549040, w_549040),
    (0x59a1e0, w_59a1e0),
    (0x5f2e90, w_5f2e90),
    (0x5f2f90, w_5f2f90),
    (0x5f30b0, w_5f30b0),
    (0x633520, w_633520),
];
