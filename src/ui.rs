use crate::game::CellState;
/// UI组件模块
/// 包含所有用户界面相关的渲染和交互逻辑
use crate::{patterns, GameOfLifeApp, ColorTheme};
use eframe::egui;

/// 控制面板相关的UI渲染
impl GameOfLifeApp {
    /// 渲染左侧控制面板
    pub fn render_control_panel(&mut self, ui: &mut egui::Ui) {
        // 添加垂直滚动区域包装整个控制面板
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.heading("Conway's Game of Life");

                // 显示当前迭代次数
                ui.label(format!("Generation: {}", self.generation));
                
                // 显示控制提示
                ui.label(egui::RichText::new("🎮 Controls:")
                        .size(11.0)
                        .strong());
                ui.label(egui::RichText::new("Space: Play/Pause | S: Step | C: Clear | R: Random")
                        .size(9.0)
                        .color(egui::Color32::GRAY));
                ui.label(egui::RichText::new("T: Theme | Ctrl+S: Save | Ctrl+O: Load")
                        .size(9.0)
                        .color(egui::Color32::GRAY));
                ui.label(egui::RichText::new("Ctrl+Scroll: Zoom | Drag: Draw")
                        .size(9.0)
                        .color(egui::Color32::GRAY));
                ui.separator();

                // 游戏控制区域 默认展开
                egui::CollapsingHeader::new("Game Controls")
                    .default_open(true)
                    .show(ui, |ui| {
                        self.render_game_controls(ui);
                    });

                ui.add_space(5.0);

                // 视觉设置区域
                ui.collapsing("Visual Settings", |ui| {
                    self.render_visual_settings(ui);
                });

                ui.add_space(5.0);

                // 模拟设置区域
                ui.collapsing("Simulation Settings", |ui| {
                    self.render_simulation_settings(ui);
                });

                ui.add_space(5.0);

                // 预设图案区域
                ui.collapsing("Pattern Presets", |ui| {
                    self.render_presets_panel(ui);
                });

                ui.add_space(5.0);

                // 统计信息区域
                ui.collapsing("Statistics", |ui| {
                    self.render_statistics_controls(ui);
                });

                // 在面板底部添加一些额外空间
                ui.add_space(20.0);
            });
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
                self.update_population_history(); // 单步时也要更新统计
            }
        });

        ui.add_space(5.0);

        // 网格操作按钮（水平布局）
        ui.horizontal(|ui| {
            // 清空网格按钮
            if ui.button("Clear").clicked() {
                self.grid.clear();
                self.generation = 0; // 重置代数计数
                self.clear_population_history(); // 清除统计历史
            }

            // 随机化网格按钮
            if ui.button("Random").clicked() {
                self.grid.randomize(self.density);
                self.generation = 0; // 重置代数计数
                self.clear_population_history(); // 清除统计历史
                self.update_population_history(); // 记录初始人口
            }
        });

        ui.add_space(5.0);

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
        if let Some(status) = self.ui_state.status_message() {
            ui.add_space(5.0);
            ui.label(egui::RichText::new(status).small().color(egui::Color32::GRAY));
        }
    }

    /// 渲染视觉设置面板
    pub fn render_visual_settings(&mut self, ui: &mut egui::Ui) {
        // 颜色主题选择
        ui.label("Color Theme:");
        ui.horizontal(|ui| {
            let current_theme = self.theme_manager.current_theme();
            if ui.selectable_label(current_theme == ColorTheme::Light, "Light").clicked() {
                self.start_theme_transition(ColorTheme::Light);
            }
            if ui.selectable_label(current_theme == ColorTheme::Dark, "Dark").clicked() {
                self.start_theme_transition(ColorTheme::Dark);
            }
        });

        ui.add_space(5.0);

        // 网格线显示开关
        let mut show_grid_lines = self.ui_state.show_grid_lines();
        if ui.checkbox(&mut show_grid_lines, "Show Grid Lines").changed() {
            self.ui_state.set_show_grid_lines(show_grid_lines);
        }

        ui.add_space(5.0);

        // 缩放控制
        let zoom_level = self.ui_state.zoom_level();
        ui.label(format!("Zoom Level: {:.1}x", zoom_level));
        let mut new_zoom = zoom_level;
        if ui
            .add(egui::Slider::new(&mut new_zoom, 0.1..=5.0).text("Zoom"))
            .changed()
        {
            self.ui_state.set_zoom_level(new_zoom);
        }

        // 重置缩放按钮
        if ui.button("Reset Zoom").clicked() {
            self.ui_state.set_zoom_level(1.0);
        }
    }

    /// 渲染模拟设置面板
    pub fn render_simulation_settings(&mut self, ui: &mut egui::Ui) {
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

        ui.add_space(5.0);

        // 网格尺寸调节滑块
        ui.label("Grid Width:");
        ui.add(egui::Slider::new(&mut self.grid_width, 10..=200));

        ui.label("Grid Height:");
        ui.add(egui::Slider::new(&mut self.grid_height, 10..=150));

        ui.add_space(5.0);

        // 随机密度调节滑块
        ui.label("Random Density:");
        ui.add(egui::Slider::new(&mut self.density, 0.0..=1.0));

        ui.add_space(10.0);

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
        // 直接渲染预设列表，不需要单独的滚动区域
        // 因为整个控制面板已经有滚动了
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
                        self.clear_population_history(); // 清除统计历史
                        self.update_population_history(); // 记录初始人口
                    }
                    // 显示图案描述
                    ui.label(egui::RichText::new(pattern.description).small().italics());
                    ui.add_space(3.0);
                }
            });
        }
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
                    let drag_state = match current_state {
                        CellState::Alive => CellState::Dead, // 如果当前是存活，拖动时绘制死亡
                        CellState::Dead => CellState::Alive, // 如果当前是死亡，拖动时绘制存活
                    };
                    self.ui_state.set_drag_state(drag_state);
                    self.ui_state.set_dragging(true);
                    // 设置第一个细胞的状态
                    self.grid.set_cell(x, y, drag_state);
                }
            }
        }

        // 处理拖动过程中的事件
        if self.ui_state.is_dragging() && response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some((x, y)) = mouse_to_grid(pos) {
                    // 在拖动过程中，将经过的细胞设置为拖动状态
                    if let Some(state) = self.ui_state.drag_state() {
                        self.grid.set_cell(x, y, state);
                    }
                }
            }
        }

        // 处理鼠标释放事件（结束拖动）
        if response.drag_stopped() {
            self.ui_state.set_dragging(false);
        }

        // 处理简单点击事件（非拖动）
        if response.clicked() && !self.ui_state.is_dragging() {
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
        let (alive_color, dead_color, grid_line_color) = self.get_theme_colors();
        
        // 绘制网格中的每个细胞
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                // 计算每个细胞的绘制矩形
                let rect = egui::Rect::from_min_size(
                    response.rect.left_top()
                        + egui::Vec2::new(x as f32 * effective_cell_size, y as f32 * effective_cell_size),
                    egui::Vec2::splat(effective_cell_size),
                );

                // 根据细胞状态和主题选择颜色
                let color = match self.grid.get_cell(x, y) {
                    CellState::Alive => alive_color,
                    CellState::Dead => dead_color,
                };

                // 绘制填充的矩形（细胞）
                painter.rect_filled(rect, 0.0, color);
                
                // 根据设置决定是否绘制网格线
                if self.ui_state.show_grid_lines() {
                    let line_width = if effective_cell_size < 5.0 { 0.2 } else { 0.5 };
                    painter.rect_stroke(rect, 0.0, egui::Stroke::new(line_width, grid_line_color));
                }
            }
        }
    }

    /// 渲染统计控制（在左侧面板中）
    pub fn render_statistics_controls(&mut self, ui: &mut egui::Ui) {
        // 显示当前活细胞数量
        let current_population = self.get_current_population();
        ui.label(format!("Live Cells: {}", current_population));
        
        ui.add_space(5.0);
        
        // 显示统计开关
        let mut show_statistics = self.show_statistics();
        if ui.checkbox(&mut show_statistics, "Show Statistics Panel").changed() {
            self.set_show_statistics(show_statistics);
        }
        
        ui.add_space(5.0);
        
        // 清除历史按钮
        if ui.button("Clear History").clicked() {
            self.clear_population_history();
        }
    }

    /// 渲染统计信息面板（在右侧面板中）
    pub fn render_statistics_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Population Statistics");
        
        ui.add_space(10.0);
        
        // 显示当前活细胞数量
        let current_population = self.get_current_population();
        ui.label(format!("Current Live Cells: {}", current_population));
        
        // 显示历史记录长度
        ui.label(format!("Generations Recorded: {}", self.statistics.get_history_length()));
        
        // 显示最大和最小人口
        if self.statistics.has_data() {
            if let Some(max_pop) = self.statistics.get_max_population() {
                ui.label(format!("Max Population: {}", max_pop));
            }
            if let Some(min_pop) = self.statistics.get_min_population() {
                ui.label(format!("Min Population: {}", min_pop));
            }
            if let Some(avg_pop) = self.statistics.get_average_population() {
                ui.label(format!("Average Population: {:.1}", avg_pop));
            }
            
            // 显示趋势信息
            if let Some(trend) = self.statistics.get_population_trend(5) {
                let trend_text = match trend {
                    1 => "📈 Growing",
                    -1 => "📉 Declining", 
                    0 => "➡️ Stable",
                    _ => "❓ Unknown"
                };
                ui.label(format!("Trend: {}", trend_text));
            }
            
            // 显示稳定性
            if self.statistics.is_stable(10, 5) {
                ui.label("🔒 Population Stable");
            }
        }
        
        ui.add_space(15.0);
        
        // 如果有历史数据，则绘制图表
        if self.statistics.has_data() {
            ui.label("Population History:");
            ui.add_space(5.0);
            self.render_population_chart(ui);
        } else {
            ui.label("No population data yet. Start the simulation to see the chart.");
        }
    }

    /// 渲染人口增长图表
    pub fn render_population_chart(&self, ui: &mut egui::Ui) {
        use egui_plot::{Line, Plot, PlotPoints};
        
        let history = self.get_population_history();
        if history.is_empty() {
            return;
        }
        
        // 准备图表数据
        let points: PlotPoints = history
            .iter()
            .enumerate()
            .map(|(i, &population)| [i as f64, population as f64])
            .collect();
        
        let line = Line::new(points)
            .color(egui::Color32::from_rgb(100, 200, 100))
            .name("Population");
        
        // 创建图表
        Plot::new("population_chart")
            .view_aspect(1.5)
            .height(200.0)
            .allow_zoom(true)
            .allow_drag(true)
            .show_axes([true, true])
            .x_axis_label("Generation")
            .y_axis_label("Population")
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}
