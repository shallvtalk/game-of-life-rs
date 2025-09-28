/// 细胞状态枚举
/// 在康威生命游戏中，每个细胞只有两种状态：存活或死亡
#[derive(Clone, Copy, PartialEq, Debug)]
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
    /// 一维向量存储所有细胞的状态
    /// cells[y * width + x] 表示位置(x,y)的细胞状态
    cells: Vec<CellState>,
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
        // 创建一个一维向量，所有细胞初始状态为死亡
        let cells = vec![CellState::Dead; width * height];
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
        &self.cells[y * self.width + x]
    }

    /// 设置指定位置细胞的状态
    ///
    /// # 参数
    /// * `x` - 细胞的x坐标（列）
    /// * `y` - 细胞的y坐标（行）
    /// * `state` - 要设置的细胞状态
    pub fn set_cell(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = state;
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
            let index = y * self.width + x;
            self.cells[index] = match self.cells[index] {
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
        
        // 定义8个邻居的相对位置偏移量
        const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];
        
        // 遍历所有邻居位置
        for (dx, dy) in NEIGHBOR_OFFSETS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            // 检查邻居是否在网格范围内
            if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                let index = (ny as usize) * self.width + (nx as usize);
                if self.cells[index] == CellState::Alive {
                    count += 1;
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
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let neighbors = self.count_neighbors(x, y);

                // 根据康威生命游戏规则决定细胞的下一代状态
                new_cells[index] = match (self.cells[index], neighbors) {
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
        for cell in &mut self.cells {
            *cell = CellState::Dead;
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
        for index in 0..self.cells.len() {
            // 生成下一个伪随机数（线性同余生成器）
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            // 将随机数转换为[0.0, 1.0]范围的浮点数
            let rand_val = (rng_state >> 32) as f32 / u32::MAX as f32;

            // 根据密度参数决定细胞状态
            self.cells[index] = if rand_val < density {
                CellState::Alive
            } else {
                CellState::Dead
            };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 8);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 8);

        // All cells should be dead initially
        for y in 0..8 {
            for x in 0..10 {
                assert_eq!(grid.get_cell(x, y), &CellState::Dead);
            }
        }
    }

    #[test]
    fn test_cell_manipulation() {
        let mut grid = Grid::new(5, 5);

        // Set a cell to alive
        grid.set_cell(2, 2, CellState::Alive);
        assert_eq!(grid.get_cell(2, 2), &CellState::Alive);

        // Toggle should make it dead
        grid.toggle_cell(2, 2);
        assert_eq!(grid.get_cell(2, 2), &CellState::Dead);

        // Toggle again should make it alive
        grid.toggle_cell(2, 2);
        assert_eq!(grid.get_cell(2, 2), &CellState::Alive);
    }

    #[test]
    fn test_clear_grid() {
        let mut grid = Grid::new(3, 3);

        // Set some cells alive
        grid.set_cell(0, 0, CellState::Alive);
        grid.set_cell(1, 1, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);

        // Clear the grid
        grid.clear();

        // All cells should be dead
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(grid.get_cell(x, y), &CellState::Dead);
            }
        }
    }

    #[test]
    fn test_neighbor_counting() {
        let mut grid = Grid::new(5, 5);

        // Create a simple pattern: vertical line in middle
        grid.set_cell(2, 1, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);
        grid.set_cell(2, 3, CellState::Alive);

        // Test neighbor counts
        assert_eq!(grid.count_neighbors(2, 0), 1); // Above the line
        assert_eq!(grid.count_neighbors(2, 1), 1); // Top of line
        assert_eq!(grid.count_neighbors(2, 2), 2); // Middle of line
        assert_eq!(grid.count_neighbors(2, 3), 1); // Bottom of line
        assert_eq!(grid.count_neighbors(2, 4), 1); // Below the line

        assert_eq!(grid.count_neighbors(1, 2), 3); // Left of middle
        assert_eq!(grid.count_neighbors(3, 2), 3); // Right of middle
    }

    #[test]
    fn test_blinker_pattern() {
        let mut grid = Grid::new(5, 5);

        // Create blinker pattern (horizontal line)
        grid.set_cell(1, 2, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);
        grid.set_cell(3, 2, CellState::Alive);

        // After one generation, should become vertical
        grid.next_generation();

        assert_eq!(grid.get_cell(1, 2), &CellState::Dead);
        assert_eq!(grid.get_cell(2, 1), &CellState::Alive);
        assert_eq!(grid.get_cell(2, 2), &CellState::Alive);
        assert_eq!(grid.get_cell(2, 3), &CellState::Alive);
        assert_eq!(grid.get_cell(3, 2), &CellState::Dead);

        // After another generation, should return to horizontal
        grid.next_generation();

        assert_eq!(grid.get_cell(1, 2), &CellState::Alive);
        assert_eq!(grid.get_cell(2, 1), &CellState::Dead);
        assert_eq!(grid.get_cell(2, 2), &CellState::Alive);
        assert_eq!(grid.get_cell(2, 3), &CellState::Dead);
        assert_eq!(grid.get_cell(3, 2), &CellState::Alive);
    }

    #[test]
    fn test_load_pattern() {
        let mut grid = Grid::new(10, 10);

        let pattern = &[" O ", "  O", "OOO"];

        grid.load_pattern(pattern, 3, 3);

        // Check that the pattern was loaded correctly
        assert_eq!(grid.get_cell(3, 3), &CellState::Dead); // space
        assert_eq!(grid.get_cell(4, 3), &CellState::Alive); // O
        assert_eq!(grid.get_cell(5, 3), &CellState::Dead); // space

        assert_eq!(grid.get_cell(3, 4), &CellState::Dead); // space
        assert_eq!(grid.get_cell(4, 4), &CellState::Dead); // space
        assert_eq!(grid.get_cell(5, 4), &CellState::Alive); // O

        assert_eq!(grid.get_cell(3, 5), &CellState::Alive); // O
        assert_eq!(grid.get_cell(4, 5), &CellState::Alive); // O
        assert_eq!(grid.get_cell(5, 5), &CellState::Alive); // O
    }
}
