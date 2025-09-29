// 导入游戏逻辑模块
mod game;
mod patterns;
mod save_load;
mod ui;

// 导入所需的外部crate
use eframe::egui;
use game::{CellState, Grid};

/// 颜色主题枚举
#[derive(Clone, Copy, PartialEq)]
enum ColorTheme {
    Light,
    Dark,
}

/// 康威生命游戏应用程序的主结构体
/// 包含游戏状态、UI设置和控制参数
struct GameOfLifeApp {
    /// 游戏网格，存储细胞状态
    grid: Grid,
    /// 游戏是否正在运行（自动更新）
    is_running: bool,
    /// 每个细胞在屏幕上的显示大小（像素）
    cell_size: f32,
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
    /// 跟踪是否正在拖动绘制
    is_dragging: bool,
    /// 拖动时绘制的细胞状态（存活或死亡）
    drag_state: Option<CellState>,
    /// 当前迭代次数（代数）
    generation: usize,
    /// 保存/加载状态信息
    save_load_status: Option<String>,
    /// 状态信息显示的时间戳
    status_timestamp: Option<std::time::Instant>,
    /// 缩放级别（1.0为默认大小）
    zoom_level: f32,
    /// 当前颜色主题
    color_theme: ColorTheme,
    /// 是否显示网格线
    show_grid_lines: bool,
    /// 主题切换动画进度 (0.0 到 1.0)
    theme_transition_progress: f32,
    /// 主题切换开始时间
    theme_transition_start: Option<std::time::Instant>,
    /// 目标主题（用于动画过渡）
    target_theme: ColorTheme,
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

        Self {
            grid,
            is_running: false,                      // 初始状态为暂停
            cell_size: 10.0,                        // 每个细胞10像素大小
            last_update: std::time::Instant::now(), // 记录当前时间
            update_interval: std::time::Duration::from_millis(100), // 默认100ms更新一次（10 FPS）
            grid_width,
            grid_height,
            update_speed: 10.0, // 默认10 FPS
            density,
            is_dragging: false, // 初始状态为非拖动
            drag_state: None,   // 初始拖动状态为None
            generation: 0,      // 初始代数为0
            save_load_status: None, // 初始状态无保存/加载信息
            status_timestamp: None, // 初始状态无时间戳
            zoom_level: 1.0,    // 默认缩放级别
            color_theme: ColorTheme::Dark, // 默认浅色主题
            show_grid_lines: true, // 默认显示网格线
            theme_transition_progress: 1.0, // 初始无动画
            theme_transition_start: None,   // 初始无动画
            target_theme: ColorTheme::Dark, // 初始目标主题与当前主题相同
        }
    }
}

impl GameOfLifeApp {
    /// 设置状态信息
    fn set_status(&mut self, message: String) {
        self.save_load_status = Some(message);
        self.status_timestamp = Some(std::time::Instant::now());
    }

    /// 检查并清除过期的状态信息
    fn update_status(&mut self) {
        if let Some(timestamp) = self.status_timestamp {
            if timestamp.elapsed() > std::time::Duration::from_secs(5) {
                self.save_load_status = None;
                self.status_timestamp = None;
            }
        }
    }

    /// 调整缩放级别
    fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom_level = new_zoom.clamp(0.1, 5.0); // 限制缩放范围在0.1到5.0之间
    }

    /// 获取当前有效的细胞大小（考虑缩放）
    fn effective_cell_size(&self) -> f32 {
        self.cell_size * self.zoom_level
    }

    /// 处理缩放操作
    fn handle_zoom(&mut self, delta: f32, _mouse_pos: Option<egui::Pos2>) {
        let old_zoom = self.zoom_level;
        self.set_zoom(old_zoom + delta * 1.0);

        // 可以在这里添加基于鼠标位置的智能缩放中心点
        // 暂时保持简单的实现
    }

    /// 颜色插值函数
    fn lerp_color(from: egui::Color32, to: egui::Color32, t: f32) -> egui::Color32 {
        let t = t.clamp(0.0, 1.0);
        egui::Color32::from_rgb(
            (from.r() as f32 * (1.0 - t) + to.r() as f32 * t) as u8,
            (from.g() as f32 * (1.0 - t) + to.g() as f32 * t) as u8,
            (from.b() as f32 * (1.0 - t) + to.b() as f32 * t) as u8,
        )
    }

    /// 获取当前主题的颜色配置（支持动画过渡）
    fn get_theme_colors(&self) -> (egui::Color32, egui::Color32, egui::Color32) {
        let light_colors = (
            egui::Color32::BLACK, // 存活细胞
            egui::Color32::WHITE, // 死亡细胞
            egui::Color32::GRAY,  // 网格线
        );
        let dark_colors = (
            egui::Color32::WHITE,           // 存活细胞
            egui::Color32::from_rgb(30, 30, 30), // 死亡细胞
            egui::Color32::from_rgb(60, 60, 60), // 网格线
        );

        // 如果正在进行主题切换动画
        if self.theme_transition_progress < 1.0 {
            let (from_colors, to_colors) = match self.target_theme {
                ColorTheme::Dark => (light_colors, dark_colors),
                ColorTheme::Light => (dark_colors, light_colors),
            };

            (
                Self::lerp_color(from_colors.0, to_colors.0, self.theme_transition_progress),
                Self::lerp_color(from_colors.1, to_colors.1, self.theme_transition_progress),
                Self::lerp_color(from_colors.2, to_colors.2, self.theme_transition_progress),
            )
        } else {
            match self.color_theme {
                ColorTheme::Light => light_colors,
                ColorTheme::Dark => dark_colors,
            }
        }
    }

    /// 开始主题切换动画
    pub fn start_theme_transition(&mut self, new_theme: ColorTheme) {
        if new_theme != self.color_theme {
            self.target_theme = new_theme;
            self.theme_transition_start = Some(std::time::Instant::now());
            self.theme_transition_progress = 0.0;
        }
    }

    /// 更新主题切换动画
    fn update_theme_transition(&mut self) {
        if let Some(start_time) = self.theme_transition_start {
            const TRANSITION_DURATION: f32 = 0.3; // 300ms 动画时长
            let elapsed = start_time.elapsed().as_secs_f32();
            self.theme_transition_progress = (elapsed / TRANSITION_DURATION).min(1.0);

            // 使用缓动函数让动画更自然
            let eased_progress = self.ease_in_out_cubic(self.theme_transition_progress);
            self.theme_transition_progress = eased_progress;

            // 动画完成后更新主题
            if self.theme_transition_progress >= 1.0 {
                self.color_theme = self.target_theme;
                self.theme_transition_start = None;
                self.theme_transition_progress = 1.0;
            }
        }
    }

    /// 缓动函数：缓入缓出立方
    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    /// 设置UI主题（支持动画过渡）
    fn set_ui_theme(&self, ctx: &egui::Context) {
        // 在动画过程中，根据进度选择UI主题
        let ui_theme = if self.theme_transition_progress < 1.0 {
            if self.theme_transition_progress < 0.5 {
                // 前半段使用当前主题
                self.color_theme
            } else {
                // 后半段使用目标主题
                self.target_theme
            }
        } else {
            self.color_theme
        };

        match ui_theme {
            ColorTheme::Light => {
                ctx.set_visuals(egui::Visuals::light());
            }
            ColorTheme::Dark => {
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
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
                self.cell_size,
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
                            self.cell_size = game_state.settings.cell_size;
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

        // 检查是否需要自动更新游戏状态
        if self.is_running && self.last_update.elapsed() >= self.update_interval {
            self.grid.next_generation(); // 计算下一代
            self.generation += 1; // 增加代数计数
            self.last_update = std::time::Instant::now(); // 更新时间戳
            ctx.request_repaint(); // 请求重绘界面
        }

        // 创建左侧控制面板
        egui::SidePanel::left("controls").show(ctx, |ui| {
            self.render_control_panel(ui);
        });

        // 创建中央面板用于显示游戏网格
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_game_grid(ui);
        });

        // 如果游戏正在运行，请求在下一个更新间隔后重绘
        if self.is_running {
            ctx.request_repaint_after(self.update_interval);
        }

        // 如果正在进行主题切换动画，请求持续重绘
        if self.theme_transition_progress < 1.0 {
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
