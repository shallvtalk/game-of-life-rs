/// RLE (Run Length Encoded) 格式处理模块
/// 用于导入和导出标准的生命游戏RLE格式文件
use crate::game::{Grid, CellState};
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

/// RLE图案数据结构
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
            pattern.width = value.parse().map_err(|_| {
                RleError::ParseError(format!("Invalid width: {}", value))
            })?;
        } else if let Some(value) = part.strip_prefix("y=") {
            pattern.height = value.parse().map_err(|_| {
                RleError::ParseError(format!("Invalid height: {}", value))
            })?;
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

                let count: usize = num_str.parse().map_err(|_| {
                    RleError::ParseError(format!("Invalid number: {}", num_str))
                })?;

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

/// 导出RLE文件
pub fn export_rle_file<P: AsRef<Path>>(
    path: P,
    pattern: &RlePattern,
) -> Result<(), RleError> {
    let rle_string = export_to_rle_string(pattern);
    fs::write(path, rle_string)?;
    Ok(())
}

/// 导入RLE文件
pub fn import_rle_file<P: AsRef<Path>>(path: P) -> Result<RlePattern, RleError> {
    let content = fs::read_to_string(path)?;
    import_from_rle_string(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_export_import() {
        // 创建一个简单的3x3图案
        let mut pattern = RlePattern::new("Test Pattern".to_string(), 3, 3);
        pattern.data[1][1] = true; // 中心点
        pattern.comment = "Test comment".to_string();

        // 导出为RLE字符串
        let rle_string = export_to_rle_string(&pattern);
        println!("Exported RLE:\n{}", rle_string);

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
}