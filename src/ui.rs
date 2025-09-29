use crate::game::CellState;
/// UI组件模块
/// 包含所有用户界面相关的渲染和交互逻辑
use crate::{patterns, GameOfLifeApp};
use eframe::egui;

/// 控制面板相关的UI渲染
impl GameOfLifeApp {
    /// 渲染左侧控制面板
    pub fn render_control_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Controls");

        // 显示当前迭代次数
        ui.label(format!("Generation: {}", self.generation));
        
        // 显示控制提示
        ui.label(egui::RichText::new("💡 Ctrl+Scroll: zoom | Drag: draw")
                .size(10.0)
                .color(egui::Color32::GRAY));
        ui.separator();

        self.render_game_controls(ui);
        self.render_settings_panel(ui);
        self.render_presets_panel(ui);
    }

    /// 渲染游戏控制按钮
    pub fn render_game_controls(&mut self, ui: &mut egui::Ui) {
        // 游戏控制按钮（水平布局）
        ui.horizontal(|ui| {
            // 开始/暂停按钮
            if ui
                .button(if self.is_running { "Pause" } else { "Start" })
                .clicked()
            {
                self.is_running = !self.is_running;
                self.last_update = std::time::Instant::now();
            }

            // 单步执行按钮
            if ui.button("Step").clicked() {
                self.grid.next_generation();
                self.generation += 1; // 单步时也要增加代数
            }
        });

        // 网格操作按钮（水平布局）
        ui.horizontal(|ui| {
            // 清空网格按钮
            if ui.button("Clear").clicked() {
                self.grid.clear();
                self.generation = 0; // 重置代数计数
            }

            // 随机化网格按钮
            if ui.button("Random").clicked() {
                self.grid.randomize(self.density);
                self.generation = 0; // 重置代数计数
            }
        });

        // 文件操作按钮（水平布局）
        ui.horizontal(|ui| {
            // 保存按钮
            if ui.button("Save").clicked() {
                self.save_game();
            }

            // 加载按钮
            if ui.button("Load").clicked() {
                self.load_game();
            }
        });

        // 显示保存/加载状态信息
        if let Some(status) = &self.save_load_status {
            ui.separator();
            ui.label(egui::RichText::new(status).small().color(egui::Color32::GRAY));
        }
    }

    /// 渲染设置面板
    pub fn render_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Settings");

        // 更新速度调节滑块
        ui.label("Update Speed (FPS):");
        if ui
            .add(egui::Slider::new(&mut self.update_speed, 1.0..=30.0))
            .changed()
        {
            // 当速度改变时，重新计算更新间隔
            self.update_interval =
                std::time::Duration::from_millis((1000.0 / self.update_speed) as u64);
        }

        // 网格尺寸调节滑块
        ui.label("Grid Width:");
        ui.add(egui::Slider::new(&mut self.grid_width, 10..=200));

        ui.label("Grid Height:");
        ui.add(egui::Slider::new(&mut self.grid_height, 10..=150));

        // 随机密度调节滑块
        ui.label("Random Density:");
        ui.add(egui::Slider::new(&mut self.density, 0.0..=1.0));

        ui.separator();

        // 缩放控制
        ui.label(format!("Zoom Level: {:.1}x", self.zoom_level));
        if ui
            .add(egui::Slider::new(&mut self.zoom_level, 0.1..=5.0).text("Zoom"))
            .changed()
        {
            // 确保缩放级别在有效范围内
            self.zoom_level = self.zoom_level.clamp(0.1, 5.0);
        }

        // 重置缩放按钮
        if ui.button("Reset Zoom").clicked() {
            self.zoom_level = 1.0;
        }

        ui.separator();

        // 应用网格设置按钮
        if ui.button("Apply Grid Settings").clicked() {
            // 创建新的网格并随机化
            self.grid = crate::game::Grid::new(self.grid_width, self.grid_height);
            self.grid.randomize(self.density);
            self.generation = 0; // 重置代数计数
        }
    }

    /// 渲染预设面板
    pub fn render_presets_panel(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Presets");

        // 使用新的预设系统
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for (category_name, patterns) in patterns::get_all_patterns() {
                    ui.collapsing(category_name, |ui| {
                        for pattern in patterns {
                            if ui.button(pattern.name).clicked() {
                                // 计算居中位置
                                let center_x =
                                    (self.grid.width().saturating_sub(pattern.data[0].len())) / 2;
                                let center_y =
                                    (self.grid.height().saturating_sub(pattern.data.len())) / 2;
                                self.grid.load_pattern(pattern.data, center_x, center_y);
                                self.generation = 0; // 重置代数计数
                            }
                            // 显示图案描述
                            ui.label(egui::RichText::new(pattern.description).small().italics());
                            ui.separator();
                        }
                    });
                }
            });
    }
}

/// 游戏网格相关的UI渲染
impl GameOfLifeApp {
    /// 渲染游戏网格并处理鼠标交互
    pub fn render_game_grid(&mut self, ui: &mut egui::Ui) {
        // 计算有效的细胞大小（考虑缩放）
        let effective_cell_size = self.effective_cell_size();
        
        // 计算总的网格大小
        let total_grid_size = egui::Vec2::new(
            self.grid.width() as f32 * effective_cell_size,
            self.grid.height() as f32 * effective_cell_size,
        );

        // 创建滚动区域
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // 分配绘图区域
                let (response, painter) = ui.allocate_painter(
                    total_grid_size,
                    egui::Sense::click_and_drag(), // 允许鼠标点击和拖动交互
                );

                // 处理缩放（Ctrl + 鼠标滚轮）
                if response.hovered() {
                    let ctrl_pressed = ui.input(|i| i.modifiers.ctrl);
                    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                    if ctrl_pressed && scroll_delta != 0.0 {
                        let mouse_pos = response.interact_pointer_pos();
                        self.handle_zoom(scroll_delta * 0.001, mouse_pos);
                    }
                }

                // 处理鼠标交互
                self.handle_mouse_interaction(&response);

                // 绘制网格
                self.draw_grid(&response, &painter);
            });
    }

    /// 处理鼠标交互事件
    pub fn handle_mouse_interaction(&mut self, response: &egui::Response) {
        // 处理鼠标事件的辅助函数：将鼠标坐标转换为网格坐标
        let grid_width = self.grid.width();
        let grid_height = self.grid.height();
        let effective_cell_size = self.effective_cell_size();
        let mouse_to_grid = |pos: egui::Pos2| -> Option<(usize, usize)> {
            let rect = response.rect;
            let x = ((pos.x - rect.left()) / effective_cell_size) as usize;
            let y = ((pos.y - rect.top()) / effective_cell_size) as usize;
            if x < grid_width && y < grid_height {
                Some((x, y))
            } else {
                None
            }
        };

        // 处理鼠标按下事件（开始拖动）
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some((x, y)) = mouse_to_grid(pos) {
                    // 开始拖动时，记住当前细胞的状态，并决定拖动时要绘制的状态
                    let current_state = self.grid.get_cell(x, y).clone();
                    self.drag_state = Some(match current_state {
                        CellState::Alive => CellState::Dead, // 如果当前是存活，拖动时绘制死亡
                        CellState::Dead => CellState::Alive, // 如果当前是死亡，拖动时绘制存活
                    });
                    self.is_dragging = true;
                    // 设置第一个细胞的状态
                    self.grid.set_cell(x, y, self.drag_state.clone().unwrap());
                }
            }
        }

        // 处理拖动过程中的事件
        if self.is_dragging && response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some((x, y)) = mouse_to_grid(pos) {
                    // 在拖动过程中，将经过的细胞设置为拖动状态
                    if let Some(state) = &self.drag_state {
                        self.grid.set_cell(x, y, state.clone());
                    }
                }
            }
        }

        // 处理鼠标释放事件（结束拖动）
        if response.drag_stopped() {
            self.is_dragging = false;
            self.drag_state = None;
        }

        // 处理简单点击事件（非拖动）
        if response.clicked() && !self.is_dragging {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some((x, y)) = mouse_to_grid(pos) {
                    // 简单点击时切换细胞状态
                    self.grid.toggle_cell(x, y);
                }
            }
        }
    }

    /// 绘制游戏网格
    pub fn draw_grid(&self, response: &egui::Response, painter: &egui::Painter) {
        let effective_cell_size = self.effective_cell_size();
        
        // 绘制网格中的每个细胞
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                // 计算每个细胞的绘制矩形
                let rect = egui::Rect::from_min_size(
                    response.rect.left_top()
                        + egui::Vec2::new(x as f32 * effective_cell_size, y as f32 * effective_cell_size),
                    egui::Vec2::splat(effective_cell_size),
                );

                // 根据细胞状态选择颜色
                let color = match self.grid.get_cell(x, y) {
                    CellState::Alive => egui::Color32::BLACK, // 存活细胞显示为黑色
                    CellState::Dead => egui::Color32::WHITE,  // 死亡细胞显示为白色
                };

                // 绘制填充的矩形（细胞）
                painter.rect_filled(rect, 0.0, color);
                // 绘制边框线（根据缩放级别调整线条粗细）
                let line_width = if effective_cell_size < 5.0 { 0.2 } else { 0.5 };
                painter.rect_stroke(rect, 0.0, egui::Stroke::new(line_width, egui::Color32::GRAY));
            }
        }
    }
}
