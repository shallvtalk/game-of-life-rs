// 导入游戏逻辑模块
mod game;
mod patterns;
mod ui;

// 导入所需的外部crate
use eframe::egui;
use game::{Grid, CellState};

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
            is_running: false,                                          // 初始状态为暂停
            cell_size: 10.0,                                           // 每个细胞10像素大小
            last_update: std::time::Instant::now(),                   // 记录当前时间
            update_interval: std::time::Duration::from_millis(100),   // 默认100ms更新一次（10 FPS）
            grid_width,
            grid_height,
            update_speed: 10.0,                                        // 默认10 FPS
            density,
            is_dragging: false,                                        // 初始状态为非拖动
            drag_state: None,                                          // 初始拖动状态为None
            generation: 0,                                             // 初始代数为0
        }
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
        // 检查是否需要自动更新游戏状态
        if self.is_running && self.last_update.elapsed() >= self.update_interval {
            self.grid.next_generation();                    // 计算下一代
            self.generation += 1;                           // 增加代数计数
            self.last_update = std::time::Instant::now();  // 更新时间戳
            ctx.request_repaint();                          // 请求重绘界面
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
    }

}

/// 程序主入口函数
/// 创建并运行康威生命游戏应用程序
fn main() -> Result<(), eframe::Error> {
    // 配置应用程序窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])                    // 设置窗口初始大小
            .with_title("Conway's Game of Life"),               // 设置窗口标题
        ..Default::default()
    };

    // 启动native应用程序
    eframe::run_native(
        "Conway's Game of Life",                                // 应用程序名称
        options,                                                // 窗口配置选项
        Box::new(|_cc| Ok(Box::new(GameOfLifeApp::default()))), // 创建应用程序实例的闭包
    )
}
