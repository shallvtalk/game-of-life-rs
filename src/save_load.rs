/// RLE格式保存和加载模块
/// 专门支持RLE (Run Length Encoded) 格式的文件保存和加载功能
use crate::game::{CellState, Grid};
use std::fs;
use std::path::Path;

/// RLE格式的错误类型
#[derive(Debug)]
pub enum RleError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidFormat(String),
}

impl From<std::io::Error> for RleError {
    fn from(error: std::io::Error) -> Self {
        RleError::IoError(error)
    }
}

impl std::fmt::Display for RleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RleError::IoError(err) => write!(f, "IO error: {}", err),
            RleError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RleError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for RleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RleError::IoError(e) => Some(e),
            RleError::ParseError(_) => None,
            RleError::InvalidFormat(_) => None,
        }
    }
}

/// RLE图案数据结构
#[derive(Debug, Clone)]
pub struct RlePattern {
    pub name: String,
    pub comment: String,
    pub author: String,
    pub width: usize,
    pub height: usize,
    pub rule: String,
    pub data: Vec<Vec<bool>>,
}

impl RlePattern {
    /// 创建新的RLE图案
    pub fn new(name: String, width: usize, height: usize) -> Self {
        Self {
            name,
            comment: String::new(),
            author: String::new(),
            width,
            height,
            rule: "B3/S23".to_string(), // 康威生命游戏标准规则
            data: vec![vec![false; width]; height],
        }
    }

    /// 从网格创建RLE图案
    pub fn from_grid(grid: &Grid, name: String) -> Self {
        let width = grid.width();
        let height = grid.height();
        let mut data = vec![vec![false; width]; height];

        for y in 0..height {
            for x in 0..width {
                data[y][x] = matches!(grid.get_cell(x, y), CellState::Alive);
            }
        }

        Self {
            name,
            comment: String::new(),
            author: String::new(),
            width,
            height,
            rule: "B3/S23".to_string(),
            data,
        }
    }

    /// 将RLE图案转换为Grid
    #[allow(dead_code)]
    pub fn to_grid(&self) -> Result<Grid, RleError> {
        if self.width == 0 || self.height == 0 {
            return Err(RleError::InvalidFormat(
                "Grid dimensions cannot be zero".to_string(),
            ));
        }

        let mut grid = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y][x] {
                    grid.set_cell(x, y, CellState::Alive);
                }
            }
        }

        Ok(grid)
    }
}

/// 将RLE图案导出为RLE格式字符串
pub fn export_to_rle_string(pattern: &RlePattern) -> String {
    let mut result = String::new();

    // 添加注释行
    if !pattern.name.is_empty() {
        result.push_str(&format!("#N {}\n", pattern.name));
    }
    if !pattern.comment.is_empty() {
        result.push_str(&format!("#C {}\n", pattern.comment));
    }
    if !pattern.author.is_empty() {
        result.push_str(&format!("#O {}\n", pattern.author));
    }

    // 添加头部行 (x = width, y = height, rule = rule)
    result.push_str(&format!(
        "x = {}, y = {}, rule = {}\n",
        pattern.width, pattern.height, pattern.rule
    ));

    // 编码图案数据
    let mut encoded_lines = Vec::new();

    for row in &pattern.data {
        let mut line = String::new();
        let mut count = 0;
        let mut last_cell = false;

        for &cell in row {
            if cell == last_cell {
                count += 1;
            } else {
                if count > 0 {
                    append_run(&mut line, count, last_cell);
                }
                count = 1;
                last_cell = cell;
            }
        }

        // 添加最后一个连续段
        if count > 0 {
            append_run(&mut line, count, last_cell);
        }

        encoded_lines.push(line);
    }

    // 移除末尾的空行
    while encoded_lines.last().map_or(false, |line| line.is_empty()) {
        encoded_lines.pop();
    }

    // 将所有行连接起来，用$分隔行，最后以!结束
    let pattern_data = encoded_lines.join("$");
    result.push_str(&pattern_data);
    if !pattern_data.is_empty() {
        result.push('!');
    }
    result.push('\n');

    result
}

/// 添加连续段到编码字符串
fn append_run(line: &mut String, count: usize, is_alive: bool) {
    if is_alive {
        if count == 1 {
            line.push('o');
        } else {
            line.push_str(&format!("{}o", count));
        }
    } else {
        if count == 1 {
            line.push('b');
        } else {
            line.push_str(&format!("{}b", count));
        }
    }
}

/// 保存RLE图案到文件
pub fn save_rle_file<P: AsRef<Path>>(
    path: P,
    grid: &Grid,
    name: Option<String>,
) -> Result<(), RleError> {
    let pattern_name = name.unwrap_or_else(|| "Exported Pattern".to_string());
    let pattern = RlePattern::from_grid(grid, pattern_name);
    let rle_string = export_to_rle_string(&pattern);
    fs::write(path, rle_string)?;
    Ok(())
}

/// 从RLE格式字符串导入图案
pub fn import_from_rle_string(rle_data: &str) -> Result<RlePattern, RleError> {
    let lines: Vec<&str> = rle_data.lines().collect();
    let mut pattern = RlePattern::new("Imported Pattern".to_string(), 0, 0);
    let mut header_line = None;
    let mut pattern_lines = Vec::new();

    // 解析头部信息和图案数据
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('#') {
            parse_comment_line(line, &mut pattern);
        } else if line.starts_with("x =") || line.starts_with("x=") {
            header_line = Some(line);
        } else {
            pattern_lines.push(line);
        }
    }

    // 解析头部行
    if let Some(header) = header_line {
        parse_header_line(header, &mut pattern)?;
    } else {
        return Err(RleError::InvalidFormat(
            "Missing header line (x = ..., y = ...)".to_string(),
        ));
    }

    // 解析图案数据
    let pattern_data = pattern_lines.join("");
    parse_pattern_data(&pattern_data, &mut pattern)?;

    Ok(pattern)
}

/// 解析注释行
fn parse_comment_line(line: &str, pattern: &mut RlePattern) {
    if let Some(rest) = line.strip_prefix("#N ") {
        pattern.name = rest.to_string();
    } else if let Some(rest) = line.strip_prefix("#C ") {
        if pattern.comment.is_empty() {
            pattern.comment = rest.to_string();
        } else {
            pattern.comment.push('\n');
            pattern.comment.push_str(rest);
        }
    } else if let Some(rest) = line.strip_prefix("#O ") {
        pattern.author = rest.to_string();
    }
}

/// 解析头部行
fn parse_header_line(line: &str, pattern: &mut RlePattern) -> Result<(), RleError> {
    // 移除空格并解析 "x=width,y=height,rule=rule" 格式
    let cleaned = line.replace(' ', "");
    let parts: Vec<&str> = cleaned.split(',').collect();

    for part in parts {
        if let Some(value) = part.strip_prefix("x=") {
            pattern.width = value
                .parse()
                .map_err(|_| RleError::ParseError(format!("Invalid width: {}", value)))?;
        } else if let Some(value) = part.strip_prefix("y=") {
            pattern.height = value
                .parse()
                .map_err(|_| RleError::ParseError(format!("Invalid height: {}", value)))?;
        } else if let Some(value) = part.strip_prefix("rule=") {
            pattern.rule = value.to_string();
        }
    }

    if pattern.width == 0 || pattern.height == 0 {
        return Err(RleError::InvalidFormat(
            "Width and height must be greater than 0".to_string(),
        ));
    }

    // 初始化数据数组
    pattern.data = vec![vec![false; pattern.width]; pattern.height];

    Ok(())
}

/// 解析图案数据
fn parse_pattern_data(data: &str, pattern: &mut RlePattern) -> Result<(), RleError> {
    // 移除结束符号!
    let data = data.trim_end_matches('!').trim();

    let mut x = 0;
    let mut y = 0;
    let mut chars = data.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                // 读取数字
                let mut num_str = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let count: usize = num_str
                    .parse()
                    .map_err(|_| RleError::ParseError(format!("Invalid number: {}", num_str)))?;

                // 读取下一个字符来确定类型
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'b' => {
                            // 死细胞
                            x += count;
                        }
                        'o' => {
                            // 活细胞
                            for _ in 0..count {
                                if y < pattern.height && x < pattern.width {
                                    pattern.data[y][x] = true;
                                }
                                x += 1;
                            }
                        }
                        '$' => {
                            // 换行
                            y += count;
                            x = 0;
                        }
                        _ => {
                            return Err(RleError::ParseError(format!(
                                "Invalid character after number: {}",
                                next_ch
                            )));
                        }
                    }
                } else {
                    return Err(RleError::ParseError(
                        "Number not followed by character".to_string(),
                    ));
                }
            }
            'b' => {
                // 单个死细胞
                x += 1;
            }
            'o' => {
                // 单个活细胞
                if y < pattern.height && x < pattern.width {
                    pattern.data[y][x] = true;
                }
                x += 1;
            }
            '$' => {
                // 换行
                y += 1;
                x = 0;
            }
            '!' => {
                // 结束符号
                break;
            }
            ' ' | '\t' | '\n' | '\r' => {
                // 忽略空白字符
            }
            _ => {
                return Err(RleError::ParseError(format!("Invalid character: {}", ch)));
            }
        }
    }

    Ok(())
}

/// 从RLE文件加载图案
pub fn load_rle_file<P: AsRef<Path>>(path: P) -> Result<RlePattern, RleError> {
    let content = fs::read_to_string(path)?;
    import_from_rle_string(&content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_rle_pattern_creation() {
        let mut grid = Grid::new(5, 5);
        grid.set_cell(1, 1, CellState::Alive);
        grid.set_cell(2, 2, CellState::Alive);

        let pattern = RlePattern::from_grid(&grid, "Test Pattern".to_string());

        assert_eq!(pattern.width, 5);
        assert_eq!(pattern.height, 5);
        assert_eq!(pattern.name, "Test Pattern");
        assert!(pattern.data[1][1]);
        assert!(pattern.data[2][2]);
        assert!(!pattern.data[0][0]);
    }

    #[test]
    fn test_rle_pattern_to_grid() {
        let mut pattern = RlePattern::new("Test".to_string(), 3, 3);
        pattern.data[0][0] = true;
        pattern.data[1][1] = true;
        pattern.data[2][2] = true;

        let grid = pattern.to_grid().unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(*grid.get_cell(0, 0), CellState::Alive);
        assert_eq!(*grid.get_cell(1, 1), CellState::Alive);
        assert_eq!(*grid.get_cell(2, 2), CellState::Alive);
        assert_eq!(*grid.get_cell(0, 1), CellState::Dead);
    }

    #[test]
    fn test_rle_export_import() {
        // 创建一个简单的3x3图案
        let mut pattern = RlePattern::new("Test Pattern".to_string(), 3, 3);
        pattern.data[1][1] = true; // 中心点
        pattern.comment = "Test comment".to_string();

        // 导出为RLE字符串
        let rle_string = export_to_rle_string(&pattern);

        // 重新导入
        let imported = import_from_rle_string(&rle_string).unwrap();

        // 验证
        assert_eq!(imported.width, 3);
        assert_eq!(imported.height, 3);
        assert_eq!(imported.name, "Test Pattern");
        assert_eq!(imported.comment, "Test comment");
        assert!(imported.data[1][1]); // 中心点应该是活的
        assert!(!imported.data[0][0]); // 其他点应该是死的
    }

    #[test]
    fn test_save_and_load_rle() -> Result<(), Box<dyn std::error::Error>> {
        let mut grid = Grid::new(4, 4);
        grid.set_cell(1, 1, CellState::Alive);
        grid.set_cell(2, 1, CellState::Alive);
        grid.set_cell(3, 1, CellState::Alive);

        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path();

        // 保存
        save_rle_file(temp_path, &grid, Some("Line Pattern".to_string()))?;

        // 加载
        let loaded_pattern = load_rle_file(temp_path)?;

        assert_eq!(loaded_pattern.width, 4);
        assert_eq!(loaded_pattern.height, 4);
        assert_eq!(loaded_pattern.name, "Line Pattern");

        let loaded_grid = loaded_pattern.to_grid()?;
        assert_eq!(*loaded_grid.get_cell(1, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(2, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(3, 1), CellState::Alive);
        assert_eq!(*loaded_grid.get_cell(0, 0), CellState::Dead);

        Ok(())
    }

    #[test]
    fn test_rle_validation() {
        let invalid_pattern = RlePattern::new("Invalid".to_string(), 0, 5);
        assert!(invalid_pattern.to_grid().is_err());

        let valid_pattern = RlePattern::new("Valid".to_string(), 3, 3);
        assert!(valid_pattern.to_grid().is_ok());
    }
}
