#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入游戏逻辑模块
mod game;
mod patterns;
mod save_load;
mod ui;
mod statistics;
mod theme;
mod ui_state;

// 导入所需的外部crate
use eframe::egui;
use game::{CellState, Grid};
use statistics::PopulationStatistics;
use theme::{ColorTheme, ThemeManager};
use ui_state::UiStateManager;

/// 康威生命游戏应用程序的主结构体
/// 包含游戏状态、UI设置和控制参数
struct GameOfLifeApp {
    /// 游戏网格，存储细胞状态
    grid: Grid,
    /// 游戏是否正在运行（自动更新）
    is_running: bool,
    /// 上次更新的时间戳，用于控制更新频率
    last_update: std::time::Instant,
    /// 更新间隔时间
    update_interval: std::time::Duration,
    /// 网格宽度设置（用于UI调节）
    grid_width: usize,
    /// 网格高度设置（用于UI调节）
    grid_height: usize,
    /// 更新速度设置（FPS）
    update_speed: f32,
    /// 随机化时的细胞密度
    density: f32,
    /// 当前迭代次数（代数）
    generation: usize,
    
    /// 人口统计管理器
    statistics: PopulationStatistics,
    /// 主题管理器
    theme_manager: ThemeManager,
    /// UI状态管理器
    ui_state: UiStateManager,
}

/// 为GameOfLifeApp实现Default trait
/// 提供应用程序的默认配置
impl Default for GameOfLifeApp {
    fn default() -> Self {
        // 设置默认网格尺寸
        let grid_width = 60;
        let grid_height = 40;

        // 创建网格并进行随机初始化
        let mut grid = Grid::new(grid_width, grid_height);
        let density = 0.3;
        grid.randomize(density);

        // 初始化人口统计
        let mut statistics = PopulationStatistics::new(200);
        let initial_population = grid.count_alive_cells();
        statistics.add_population(initial_population);

        Self {
            grid,
            is_running: false,                      // 初始状态为暂停
            last_update: std::time::Instant::now(), // 记录当前时间
            update_interval: std::time::Duration::from_millis(100), // 默认100ms更新一次（10 FPS）
            grid_width,
            grid_height,
            update_speed: 10.0, // 默认10 FPS
            density,
            generation: 0,      // 初始代数为0
            
            statistics,
            theme_manager: ThemeManager::new(ColorTheme::Dark),
            ui_state: UiStateManager::new(),
        }
    }
}

impl GameOfLifeApp {
    /// 设置状态信息
    fn set_status(&mut self, message: String) {
        self.ui_state.set_status(message);
    }

    /// 检查并清除过期的状态信息
    fn update_status(&mut self) {
        self.ui_state.update_status();
    }


    /// 获取当前有效的细胞大小（考虑缩放）
    fn effective_cell_size(&self) -> f32 {
        self.ui_state.effective_cell_size()
    }

    /// 处理缩放操作
    fn handle_zoom(&mut self, delta: f32, mouse_pos: Option<egui::Pos2>) {
        self.ui_state.handle_zoom(delta, mouse_pos);
    }

    /// 获取当前主题的颜色配置（支持动画过渡）
    fn get_theme_colors(&self) -> (egui::Color32, egui::Color32, egui::Color32) {
        self.theme_manager.get_theme_colors()
    }

    /// 开始主题切换动画
    pub fn start_theme_transition(&mut self, new_theme: ColorTheme) {
        self.theme_manager.start_theme_transition(new_theme);
    }

    /// 更新主题切换动画
    fn update_theme_transition(&mut self) {
        self.theme_manager.update_theme_transition();
    }

    /// 设置UI主题（支持动画过渡）
    fn set_ui_theme(&self, ctx: &egui::Context) {
        self.theme_manager.apply_ui_theme(ctx);
    }

    /// 保存游戏状态到文件
    fn save_game(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("RLE Files", &["rle"])
            .add_filter("Game of Life Files", &["gol"])
            .add_filter("JSON Files", &["json"])
            .set_file_name("game_state.gol")
            .save_file()
        {
            match save_load::save_file(
                &path,
                &self.grid,
                self.generation,
                self.update_speed,
                self.ui_state.cell_size(),
                self.density,
            ) {
                Ok(_) => {
                    let format = path.extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("unknown");
                    self.set_status(format!("File saved as {} format to: {:?}", format, path));
                }
                Err(e) => {
                    self.set_status(format!("Save failed: {}", e));
                }
            }
        }
    }

    /// 从文件加载游戏状态
    fn load_game(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("RLE Files", &["rle"])
            .add_filter("Game of Life Files", &["gol"])
            .add_filter("JSON Files", &["json"])
            .pick_file()
        {
            match save_load::load_file(&path) {
                Ok(save_load::LoadResult::GameState(game_state)) => {
                    match game_state.to_grid() {
                        Ok(grid) => {
                            self.grid = grid;
                            self.generation = game_state.generation;
                            self.update_speed = game_state.settings.update_speed;
                            self.ui_state.set_cell_size(game_state.settings.cell_size);
                            self.density = game_state.settings.density;
                            self.grid_width = game_state.width;
                            self.grid_height = game_state.height;

                            // 更新更新间隔
                            self.update_interval = std::time::Duration::from_millis(
                                (1000.0 / self.update_speed) as u64,
                            );

                            self.set_status(format!("Game state loaded from: {:?}", path));
                        }
                        Err(e) => {
                            self.set_status(format!("Load failed: {}", e));
                        }
                    }
                }
                Ok(save_load::LoadResult::RlePattern(pattern)) => {
                    self.load_rle_pattern(pattern, &path);
                }
                Err(e) => {
                    self.set_status(format!("File read failed: {}", e));
                }
            }
        }
    }

    /// 加载RLE图案
    fn load_rle_pattern(&mut self, pattern: save_load::rle::RlePattern, path: &std::path::Path) {
        // 创建新网格以适应RLE图案大小
        let new_width = pattern.width.max(self.grid_width);
        let new_height = pattern.height.max(self.grid_height);

        let mut new_grid = crate::game::Grid::new(new_width, new_height);

        // 计算居中位置
        let start_x = (new_width.saturating_sub(pattern.width)) / 2;
        let start_y = (new_height.saturating_sub(pattern.height)) / 2;

        // 将RLE图案加载到网格中
        for (y, row) in pattern.data.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let grid_x = start_x + x;
                    let grid_y = start_y + y;
                    if grid_x < new_width && grid_y < new_height {
                        new_grid.set_cell(grid_x, grid_y, CellState::Alive);
                    }
                }
            }
        }

        self.grid = new_grid;
        self.grid_width = new_width;
        self.grid_height = new_height;
        self.generation = 0;

        let info = if pattern.name.is_empty() {
            format!("RLE pattern loaded from: {:?}", path)
        } else {
            format!("RLE pattern '{}' loaded from: {:?}", pattern.name, path)
        };
        self.set_status(info);
    }

    /// 更新人口统计历史
    fn update_population_history(&mut self) {
        let current_population = self.grid.count_alive_cells();
        self.statistics.add_population(current_population);
    }

    /// 清除人口统计历史
    pub fn clear_population_history(&mut self) {
        self.statistics.clear_history();
    }

    /// 获取当前活细胞数量
    pub fn get_current_population(&self) -> usize {
        self.grid.count_alive_cells()
    }

    /// 获取人口历史记录的引用
    pub fn get_population_history(&self) -> &Vec<usize> {
        self.statistics.get_history()
    }

    /// 获取统计显示状态
    pub fn show_statistics(&self) -> bool {
        self.statistics.is_statistics_visible()
    }

    /// 设置统计显示状态
    pub fn set_show_statistics(&mut self, show: bool) {
        self.statistics.set_statistics_visible(show);
    }

}

/// 为GameOfLifeApp实现eframe::App trait
/// 这是egui应用程序的核心接口
impl eframe::App for GameOfLifeApp {
    /// 应用程序的主更新函数，每帧都会被调用
    ///
    /// # 参数
    /// * `ctx` - egui上下文，用于创建UI和控制重绘
    /// * `_frame` - 窗口框架信息（本例中未使用）
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 更新主题切换动画
        self.update_theme_transition();

        // 应用UI主题
        self.set_ui_theme(ctx);

        // 更新状态信息（清除过期的状态）
        self.update_status();

        // 处理键盘快捷键
        ctx.input(|i| {
            // T - 切换主题
            if i.key_pressed(egui::Key::T) {
                self.theme_manager.toggle_theme();
            }
            
            // Space - 开始/暂停
            if i.key_pressed(egui::Key::Space) {
                self.is_running = !self.is_running;
                self.last_update = std::time::Instant::now();
            }
            
            // S - 单步执行
            if i.key_pressed(egui::Key::S) {
                self.grid.next_generation();
                self.generation += 1;
                self.update_population_history();
            }
            
            // C - 清空网格
            if i.key_pressed(egui::Key::C) {
                self.grid.clear();
                self.generation = 0;
                self.clear_population_history();
            }
            
            // R - 随机化
            if i.key_pressed(egui::Key::R) {
                self.grid.randomize(self.density);
                self.generation = 0;
                self.clear_population_history();
                self.update_population_history();
            }
            
            // Ctrl+S - 保存
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save_game();
            }
            
            // Ctrl+O - 加载
            if i.modifiers.ctrl && i.key_pressed(egui::Key::O) {
                self.load_game();
            }
        });

        // 检查是否需要自动更新游戏状态
        if self.is_running && self.last_update.elapsed() >= self.update_interval {
            self.grid.next_generation(); // 计算下一代
            self.generation += 1; // 增加代数计数
            self.update_population_history(); // 更新人口统计
            self.last_update = std::time::Instant::now(); // 更新时间戳
            ctx.request_repaint(); // 请求重绘界面
        }

        // 创建左侧控制面板
        egui::SidePanel::left("controls").show(ctx, |ui| {
            self.render_control_panel(ui);
        });

        // 创建右侧统计面板（仅在显示统计时）
        if self.show_statistics() {
            egui::SidePanel::right("statistics").show(ctx, |ui| {
                self.render_statistics_panel(ui);
            });
        }

        // 创建中央面板用于显示游戏网格
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_game_grid(ui);
        });

        // 如果游戏正在运行，请求在下一个更新间隔后重绘
        if self.is_running {
            ctx.request_repaint_after(self.update_interval);
        }

        // 如果正在进行主题切换动画，请求持续重绘
        if self.theme_manager.is_transitioning() {
            ctx.request_repaint();
        }
    }
}

/// 程序主入口函数
/// 创建并运行康威生命游戏应用程序
fn main() -> Result<(), eframe::Error> {
    // 配置应用程序窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]) // 设置窗口初始大小
            .with_title("Conway's Game of Life"), // 设置窗口标题
        ..Default::default()
    };

    // 启动native应用程序
    eframe::run_native(
        "Conway's Game of Life",                                // 应用程序名称
        options,                                                // 窗口配置选项
        Box::new(|_cc| Ok(Box::new(GameOfLifeApp::default()))), // 创建应用程序实例的闭包
    )
}
