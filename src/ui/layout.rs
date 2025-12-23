pub struct LayoutState {
    /// 是否展示左侧栏
    pub show_left_panel: bool,
    /// 左侧栏宽度
    pub left_panel_width: f32,
    /// 终端是否可见
    pub terminal_visible: bool,
    /// 终端高度
    pub terminal_height: f32,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            show_left_panel: true,
            left_panel_width: 220.0,
            terminal_visible: false,
            terminal_height: 200.0,
        }
    }
}
