/// 保存和加载游戏状态的模块
/// 支持JSON格式的文件保存和加载功能

use crate::game::{CellState, Grid};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod rle;

/// 错误类型定义
#[derive(Debug)]
pub enum SaveLoadError {
    /// 文件IO错误
    IoError(std::io::Error),
    /// JSON序列化/反序列化错误
    SerializationError(serde_json::Error),
    /// 无效的游戏状态
    InvalidGameState(String),
}

impl From<std::io::Error> for SaveLoadError {
    fn from(error: std::io::Error) -> Self {
        SaveLoadError::IoError(error)
    }
}

impl From<serde_json::Error> for SaveLoadError {
    fn from(error: serde_json::Error) -> Self {
        SaveLoadError::SerializationError(error)
    }
}

impl std::fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveLoadError::IoError(e) => write!(f, "File operation error: {}", e),
            SaveLoadError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            SaveLoadError::InvalidGameState(msg) => write!(f, "Invalid game state: {}", msg),
        }
    }
}

impl std::error::Error for SaveLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SaveLoadError::IoError(e) => Some(e),
            SaveLoadError::SerializationError(e) => Some(e),
            SaveLoadError::InvalidGameState(_) => None,
        }
    }
}

/// 可序列化的游戏状态结构
/// 包含游戏的完整状态信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    /// 游戏版本信息
    pub version: String,
    /// 创建时间戳
    pub created_at: String,
    /// 网格宽度
    pub width: usize,
    /// 网格高度
    pub height: usize,
    /// 当前代数
    pub generation: usize,
    /// 细胞状态数据（压缩格式：只存储活细胞的坐标）
    pub alive_cells: Vec<(usize, usize)>,
    /// 游戏设置
    pub settings: GameSettings,
}

/// 游戏设置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    /// 更新速度(FPS)
    pub update_speed: f32,
    /// 细胞大小
    pub cell_size: f32,
    /// 随机密度
    pub density: f32,
}

impl GameState {
    /// 从Grid和相关设置创建GameState
    pub fn from_grid(
        grid: &Grid,
        generation: usize,
        update_speed: f32,
        cell_size: f32,
        density: f32,
    ) -> Self {
        // 收集所有活细胞的坐标
        let mut alive_cells = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if *grid.get_cell(x, y) == CellState::Alive {
                    alive_cells.push((x, y));
                }
            }
        }

        let settings = GameSettings {
            update_speed,
            cell_size,
            density,
        };

        GameState {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            width: grid.width(),
            height: grid.height(),
            generation,
            alive_cells,
            settings,
        }
    }

    /// 将GameState转换为Grid
    pub fn to_grid(&self) -> Result<Grid, SaveLoadError> {
        if self.width == 0 || self.height == 0 {
            return Err(SaveLoadError::InvalidGameState(
                "Grid dimensions cannot be zero".to_string(),
            ));
        }

        let mut grid = Grid::new(self.width, self.height);

        // 设置活细胞
        for &(x, y) in &self.alive_cells {
            if x >= self.width || y >= self.height {
                return Err(SaveLoadError::InvalidGameState(format!(
                    "Cell coordinates ({}, {}) exceed grid bounds ({}, {})",
                    x, y, self.width, self.height
                )));
            }
            grid.set_cell(x, y, CellState::Alive);
        }

        Ok(grid)
    }

    /// 验证游戏状态的有效性
    pub fn validate(&self) -> Result<(), SaveLoadError> {
        if self.width == 0 || self.height == 0 {
            return Err(SaveLoadError::InvalidGameState(
                "Grid dimensions cannot be zero".to_string(),
            ));
        }

        if self.settings.update_speed <= 0.0 || self.settings.update_speed > 100.0 {
            return Err(SaveLoadError::InvalidGameState(
                "Update speed must be between 1-100".to_string(),
            ));
        }

        if self.settings.cell_size <= 0.0 || self.settings.cell_size > 100.0 {
            return Err(SaveLoadError::InvalidGameState(
                "Cell size must be between 0-100".to_string(),
            ));
        }

        if self.settings.density < 0.0 || self.settings.density > 1.0 {
            return Err(SaveLoadError::InvalidGameState(
                "Density must be between 0-1".to_string(),
            ));
        }

        // 检查所有活细胞坐标是否在有效范围内
        for &(x, y) in &self.alive_cells {
            if x >= self.width || y >= self.height {
                return Err(SaveLoadError::InvalidGameState(format!(
                    "Cell coordinates ({}, {}) exceed grid bounds",
                    x, y
                )));
            }
        }

        Ok(())
    }
}

/// 保存游戏状态到文件
pub fn save_game_state<P: AsRef<Path>>(
    path: P,
    grid: &Grid,
    generation: usize,
    update_speed: f32,
    cell_size: f32,
    density: f32,
) -> Result<(), SaveLoadError> {
    let game_state = GameState::from_grid(grid, generation, update_speed, cell_size, density);
    game_state.validate()?;

    let json_content = serde_json::to_string_pretty(&game_state)?;
    fs::write(path, json_content)?;

    Ok(())
}

/// 从文件加载游戏状态
pub fn load_game_state<P: AsRef<Path>>(path: P) -> Result<GameState, SaveLoadError> {
    let json_content = fs::read_to_string(path)?;
    let game_state: GameState = serde_json::from_str(&json_content)?;
    game_state.validate()?;

    Ok(game_state)
}

/// 统一的文件保存接口 - 根据文件扩展名自动选择格式
pub fn save_file<P: AsRef<Path>>(
    path: P,
    grid: &Grid,
    generation: usize,
    update_speed: f32,
    cell_size: f32,
    density: f32,
) -> Result<(), SaveLoadError> {
    let path = path.as_ref();
    
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "rle" => {
                let pattern = rle::RlePattern::from_grid(grid, "Exported Pattern".to_string());
                rle::export_rle_file(path, &pattern)
                    .map_err(|e| SaveLoadError::InvalidGameState(e.to_string()))
            }
            _ => {
                // 默认使用JSON/GOL格式
                save_game_state(path, grid, generation, update_speed, cell_size, density)
            }
        }
    } else {
        // 没有扩展名，默认使用JSON格式
        save_game_state(path, grid, generation, update_speed, cell_size, density)
    }
}

/// 加载结果枚举
pub enum LoadResult {
    /// JSON/GOL格式的游戏状态
    GameState(GameState),
    /// RLE格式的图案
    RlePattern(rle::RlePattern),
}

/// 统一的文件加载接口 - 根据文件扩展名自动选择格式
pub fn load_file<P: AsRef<Path>>(path: P) -> Result<LoadResult, SaveLoadError> {
    let path = path.as_ref();
    
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "rle" => {
                let pattern = rle::import_rle_file(path)
                    .map_err(|e| SaveLoadError::InvalidGameState(e.to_string()))?;
                Ok(LoadResult::RlePattern(pattern))
            }
            _ => {
                // 默认使用JSON/GOL格式
                let game_state = load_game_state(path)?;
                Ok(LoadResult::GameState(game_state))
            }
        }
    } else {
        // 没有扩展名，尝试JSON格式
        let game_state = load_game_state(path)?;
        Ok(LoadResult::GameState(game_state))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_game_state_creation() {
        let mut grid = Grid::new(5, 5);
        grid.set_cell(1, 1, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);

        let game_state = GameState::from_grid(&grid, 10, 15.0, 10.0, 0.3);

        assert_eq!(game_state.width, 5);
        assert_eq!(game_state.height, 5);
        assert_eq!(game_state.generation, 10);
        assert_eq!(game_state.alive_cells.len(), 2);
        assert!(game_state.alive_cells.contains(&(1, 1)));
        assert!(game_state.alive_cells.contains(&(2, 2)));
    }

    #[test]
    fn test_game_state_to_grid() {
        let game_state = GameState {
            version: "0.1.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            width: 3,
            height: 3,
            generation: 5,
            alive_cells: vec![(0, 0), (1, 1), (2, 2)],
            settings: GameSettings {
                update_speed: 10.0,
                cell_size: 8.0,
                density: 0.2,
            },
        };

        let grid = game_state.to_grid().unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(*grid.get_cell(0, 0), CellState::Alive);
        assert_eq!(*grid.get_cell(1, 1), CellState::Alive);
        assert_eq!(*grid.get_cell(2, 2), CellState::Alive);
        assert_eq!(*grid.get_cell(0, 1), CellState::Dead);
    }

    #[test]
    fn test_save_and_load() -> Result<(), Box<dyn std::error::Error>> {
        let mut grid = Grid::new(4, 4);
        grid.set_cell(1, 1, CellState::Alive);
        grid.set_cell(2, 1, CellState::Alive);
        grid.set_cell(3, 1, CellState::Alive);

        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path();

        // 保存
        save_game_state(temp_path, &grid, 7, 12.0, 9.0, 0.4)?;

        // 加载
        let loaded_state = load_game_state(temp_path)?;

        assert_eq!(loaded_state.width, 4);
        assert_eq!(loaded_state.height, 4);
        assert_eq!(loaded_state.generation, 7);
        assert_eq!(loaded_state.settings.update_speed, 12.0);
        assert_eq!(loaded_state.alive_cells.len(), 3);

        let loaded_grid = loaded_state.to_grid()?;
        assert_eq!(*loaded_grid.get_cell(1, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(2, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(3, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(0, 0), CellState::Dead);

        Ok(())
    }

    #[test]
    fn test_validation() {
        let invalid_state = GameState {
            version: "0.1.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            width: 0, // 无效的宽度
            height: 5,
            generation: 0,
            alive_cells: vec![],
            settings: GameSettings {
                update_speed: 10.0,
                cell_size: 8.0,
                density: 0.2,
            },
        };

        assert!(invalid_state.validate().is_err());

        let invalid_coords_state = GameState {
            version: "0.1.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            width: 3,
            height: 3,
            generation: 0,
            alive_cells: vec![(5, 5)], // 超出范围的坐标
            settings: GameSettings {
                update_speed: 10.0,
                cell_size: 8.0,
                density: 0.2,
            },
        };

        assert!(invalid_coords_state.validate().is_err());
    }
}
