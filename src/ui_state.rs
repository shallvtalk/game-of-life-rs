/// UI状态管理模块
/// 负责管理用户界面的各种状态信息
use crate::game::CellState;

/// UI状态管理器
#[derive(Clone, Debug)]
pub struct UiStateManager {
    /// 每个细胞在屏幕上的显示大小（像素）
    cell_size: f32,
    /// 缩放级别（1.0为默认大小）
    zoom_level: f32,
    /// 是否显示网格线
    show_grid_lines: bool,
    /// 跟踪是否正在拖动绘制
    is_dragging: bool,
    /// 拖动时绘制的细胞状态（存活或死亡）
    drag_state: Option<CellState>,
    /// 保存/加载状态信息
    status_message: Option<String>,
    /// 状态信息显示的时间戳
    status_timestamp: Option<std::time::Instant>,
}

impl UiStateManager {
    /// 创建新的UI状态管理器
    pub fn new() -> Self {
        Self {
            cell_size: 10.0,
            zoom_level: 1.0,
            show_grid_lines: true,
            is_dragging: false,
            drag_state: None,
            status_message: None,
            status_timestamp: None,
        }
    }

    /// 获取细胞大小
    #[allow(dead_code)]
    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    /// 设置细胞大小
    #[allow(dead_code)]
    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size.max(1.0).min(50.0); // 限制在合理范围内
    }

    /// 获取缩放级别
    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }

    /// 设置缩放级别
    pub fn set_zoom_level(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.1, 5.0); // 限制缩放范围在0.1到5.0之间
    }

    /// 获取有效的细胞大小（考虑缩放）
    pub fn effective_cell_size(&self) -> f32 {
        self.cell_size * self.zoom_level
    }

    /// 处理缩放操作
    pub fn handle_zoom(&mut self, delta: f32, _mouse_pos: Option<egui::Pos2>) {
        let old_zoom = self.zoom_level;
        self.set_zoom_level(old_zoom + delta * 1.0);
        // 可以在这里添加基于鼠标位置的智能缩放中心点
    }

    /// 获取网格线显示状态
    pub fn show_grid_lines(&self) -> bool {
        self.show_grid_lines
    }

    /// 设置网格线显示状态
    pub fn set_show_grid_lines(&mut self, show: bool) {
        self.show_grid_lines = show;
    }

    /// 获取拖动状态
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// 设置拖动状态
    pub fn set_dragging(&mut self, dragging: bool) {
        self.is_dragging = dragging;
        if !dragging {
            self.drag_state = None;
        }
    }

    /// 获取拖动绘制状态
    pub fn drag_state(&self) -> Option<CellState> {
        self.drag_state
    }

    /// 设置拖动绘制状态
    pub fn set_drag_state(&mut self, state: CellState) {
        self.drag_state = Some(state);
    }

    /// 设置状态信息
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_timestamp = Some(std::time::Instant::now());
    }

    /// 清除状态信息
    pub fn clear_status(&mut self) {
        self.status_message = None;
        self.status_timestamp = None;
    }

    /// 获取当前状态信息
    pub fn status_message(&self) -> Option<&String> {
        self.status_message.as_ref()
    }

    /// 更新状态信息（清除过期的状态）
    pub fn update_status(&mut self) {
        if let Some(timestamp) = self.status_timestamp {
            if timestamp.elapsed() > std::time::Duration::from_secs(5) {
                self.clear_status();
            }
        }
    }
    /// 检查是否有活动状态信息
    #[allow(dead_code)]
    pub fn has_active_status(&self) -> bool {
        self.status_message.is_some()
    }
}

impl Default for UiStateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state_creation() {
        let ui_state = UiStateManager::new();
        assert_eq!(ui_state.cell_size(), 10.0);
        assert_eq!(ui_state.zoom_level(), 1.0);
        assert!(ui_state.show_grid_lines());
        assert!(!ui_state.is_dragging());
    }

    #[test]
    fn test_zoom_functionality() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_zoom_level(2.0);
        assert_eq!(ui_state.zoom_level(), 2.0);
        assert_eq!(ui_state.effective_cell_size(), 20.0);

        // 测试缩放限制
        ui_state.set_zoom_level(10.0);
        assert_eq!(ui_state.zoom_level(), 5.0); // 应该被限制到最大值

        ui_state.set_zoom_level(0.01);
        assert_eq!(ui_state.zoom_level(), 0.1); // 应该被限制到最小值
    }

    #[test]
    fn test_cell_size_limits() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_cell_size(100.0);
        assert_eq!(ui_state.cell_size(), 50.0); // 应该被限制到最大值

        ui_state.set_cell_size(0.5);
        assert_eq!(ui_state.cell_size(), 1.0); // 应该被限制到最小值
    }

    #[test]
    fn test_status_management() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_status("Test message".to_string());

        assert!(ui_state.has_active_status());
        assert_eq!(ui_state.status_message(), Some(&"Test message".to_string()));

        ui_state.clear_status();
        assert!(!ui_state.has_active_status());
        assert_eq!(ui_state.status_message(), None);
    }

    #[test]
    fn test_drag_state() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_dragging(true);
        ui_state.set_drag_state(CellState::Alive);

        assert!(ui_state.is_dragging());
        assert_eq!(ui_state.drag_state(), Some(CellState::Alive));

        ui_state.set_dragging(false);
        assert!(!ui_state.is_dragging());
        assert_eq!(ui_state.drag_state(), None);
    }
}
