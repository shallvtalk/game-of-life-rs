/// 细胞状态枚举
/// 在康威生命游戏中，每个细胞只有两种状态：存活或死亡
#[derive(Clone, PartialEq)]
pub enum CellState {
    /// 细胞存活状态
    Alive,
    /// 细胞死亡状态
    Dead,
}

/// 游戏网格结构体
/// 包含游戏的核心状态和逻辑
pub struct Grid {
    /// 网格宽度（列数）
    width: usize,
    /// 网格高度（行数）
    height: usize,
    /// 二维向量存储所有细胞的状态
    /// cells[y][x] 表示位置(x,y)的细胞状态
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    /// 创建一个新的游戏网格
    ///
    /// # 参数
    /// * `width` - 网格宽度（列数）
    /// * `height` - 网格高度（行数）
    ///
    /// # 返回值
    /// 返回一个所有细胞都处于死亡状态的新网格
    pub fn new(width: usize, height: usize) -> Self {
        // 创建一个height行width列的二维向量，所有细胞初始状态为死亡
        let cells = vec![vec![CellState::Dead; width]; height];
        Self {
            width,
            height,
            cells,
        }
    }

    /// 获取网格宽度
    pub fn width(&self) -> usize {
        self.width
    }

    /// 获取网格高度
    pub fn height(&self) -> usize {
        self.height
    }

    /// 获取指定位置细胞的状态
    ///
    /// # 参数
    /// * `x` - 细胞的x坐标（列）
    /// * `y` - 细胞的y坐标（行）
    ///
    /// # 返回值
    /// 返回该位置细胞状态的引用
    pub fn get_cell(&self, x: usize, y: usize) -> &CellState {
        &self.cells[y][x]
    }

    /// 设置指定位置细胞的状态
    ///
    /// # 参数
    /// * `x` - 细胞的x坐标（列）
    /// * `y` - 细胞的y坐标（行）
    /// * `state` - 要设置的细胞状态
    pub fn set_cell(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.width && y < self.height {
            self.cells[y][x] = state;
        }
    }

    /// 切换指定位置细胞的状态
    /// 如果细胞是存活的，则变为死亡；如果是死亡的，则变为存活
    ///
    /// # 参数
    /// * `x` - 细胞的x坐标（列）
    /// * `y` - 细胞的y坐标（行）
    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y][x] = match self.cells[y][x] {
                CellState::Alive => CellState::Dead,
                CellState::Dead => CellState::Alive,
            };
        }
    }

    /// 计算指定位置细胞的存活邻居数量
    ///
    /// 在康威生命游戏中，每个细胞有8个邻居（包括对角线方向）
    ///
    /// # 参数
    /// * `x` - 细胞的x坐标（列）
    /// * `y` - 细胞的y坐标（行）
    ///
    /// # 返回值
    /// 返回该细胞周围存活邻居的数量
    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        // 遍历该细胞周围的3x3区域
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                // 跳过中心细胞本身
                if dx == 0 && dy == 0 {
                    continue;
                }

                // 计算邻居的坐标
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // 检查邻居是否在网格范围内
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    // 如果邻居是存活的，计数加一
                    if self.cells[ny as usize][nx as usize] == CellState::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// 计算并更新到下一代
    ///
    /// 根据康威生命游戏的规则更新所有细胞：
    /// 1. 存活细胞有2-3个存活邻居时继续存活，否则死亡
    /// 2. 死亡细胞有恰好3个存活邻居时复活
    /// 3. 其他情况保持死亡状态
    pub fn next_generation(&mut self) {
        // 克隆当前状态，避免在计算过程中修改原数据
        let mut new_cells = self.cells.clone();

        // 遍历网格中的每个细胞
        for (y, row) in new_cells.iter_mut().enumerate().take(self.height) {
            for (x, cell) in row.iter_mut().enumerate().take(self.width) {
                let neighbors = self.count_neighbors(x, y);

                // 根据康威生命游戏规则决定细胞的下一代状态
                *cell = match (self.cells[y][x].clone(), neighbors) {
                    // 存活细胞有2或3个邻居时继续存活
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // 死亡细胞有恰好3个邻居时复活
                    (CellState::Dead, 3) => CellState::Alive,
                    // 其他情况都是死亡
                    _ => CellState::Dead,
                };
            }
        }

        // 用新计算的状态替换当前状态
        self.cells = new_cells;
    }

    /// 清空网格，将所有细胞设置为死亡状态
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = CellState::Dead;
            }
        }
    }

    /// 随机化网格中的细胞状态
    ///
    /// # 参数
    /// * `density` - 细胞存活的概率，范围[0.0, 1.0]
    ///   - 0.0 表示所有细胞都死亡
    ///   - 1.0 表示所有细胞都存活
    ///   - 0.3 表示大约30%的细胞会存活
    pub fn randomize(&mut self, density: f32) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // 使用当前时间作为随机种子
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // 创建基于时间种子的伪随机数生成器
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let mut rng_state = hasher.finish();

        // 遍历网格中的每个细胞
        for y in 0..self.height {
            for x in 0..self.width {
                // 生成下一个伪随机数（线性同余生成器）
                rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
                // 将随机数转换为[0.0, 1.0]范围的浮点数
                let rand_val = (rng_state >> 32) as f32 / u32::MAX as f32;

                // 根据密度参数决定细胞状态
                self.cells[y][x] = if rand_val < density {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
    }

    /// 从字符串图案加载预设的细胞配置
    ///
    /// # 参数
    /// * `pattern` - 字符串数组，每个字符串代表一行
    ///   - 'O', '*', '#' 字符表示存活的细胞
    ///   - 其他字符表示死亡的细胞
    /// * `x_offset` - 图案在网格中的x偏移量
    /// * `y_offset` - 图案在网格中的y偏移量
    pub fn load_pattern(&mut self, pattern: &[&str], x_offset: usize, y_offset: usize) {
        // 首先清空整个网格
        self.clear();

        // 逐行处理图案
        for (dy, line) in pattern.iter().enumerate() {
            // 逐字符处理每一行
            for (dx, ch) in line.chars().enumerate() {
                let x = x_offset + dx;
                let y = y_offset + dy;

                // 确保坐标在网格范围内
                if x < self.width && y < self.height {
                    match ch {
                        // 这些字符表示存活的细胞
                        '*' | '#' | 'O' => self.set_cell(x, y, CellState::Alive),
                        // 其他字符表示死亡的细胞
                        _ => self.set_cell(x, y, CellState::Dead),
                    }
                }
            }
        }
    }
}

/// 为Grid实现Default trait
/// 提供默认的网格配置：50x50大小
impl Default for Grid {
    fn default() -> Self {
        Self::new(50, 50)
    }
}
