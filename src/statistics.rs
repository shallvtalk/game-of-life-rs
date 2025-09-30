/// 人口统计模块
/// 负责跟踪和分析生命游戏的人口变化

/// 人口统计数据结构
#[derive(Clone, Debug)]
pub struct PopulationStatistics {
    /// 人口历史记录（最近的活细胞数量）
    history: Vec<usize>,
    /// 人口历史记录的最大长度
    max_history_length: usize,
    /// 是否显示统计信息
    show_statistics: bool,
}

impl PopulationStatistics {
    /// 创建新的人口统计实例
    pub fn new(max_history_length: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history_length,
            show_statistics: true,
        }
    }

    /// 添加新的人口数据点
    pub fn add_population(&mut self, population: usize) {
        self.history.push(population);

        // 保持历史记录在指定长度内
        if self.history.len() > self.max_history_length {
            self.history.remove(0);
        }
    }

    /// 清除人口统计历史
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// 获取人口历史记录的引用
    pub fn get_history(&self) -> &Vec<usize> {
        &self.history
    }

    /// 获取当前历史记录长度
    pub fn get_history_length(&self) -> usize {
        self.history.len()
    }
    /// 获取当前人口数（最后一个数据点）
    #[allow(dead_code)]
    pub fn get_current_population(&self) -> Option<usize> {
        self.history.last().copied()
    }
    /// 获取最大人口数
    pub fn get_max_population(&self) -> Option<usize> {
        self.history.iter().max().copied()
    }

    /// 获取最小人口数
    pub fn get_min_population(&self) -> Option<usize> {
        self.history.iter().min().copied()
    }

    /// 获取平均人口
    pub fn get_average_population(&self) -> Option<f64> {
        if self.history.is_empty() {
            return None;
        }

        let sum: usize = self.history.iter().sum();
        Some(sum as f64 / self.history.len() as f64)
    }

    /// 检查是否有历史数据
    pub fn has_data(&self) -> bool {
        !self.history.is_empty()
    }

    /// 获取统计显示状态
    pub fn is_statistics_visible(&self) -> bool {
        self.show_statistics
    }

    /// 设置统计显示状态
    pub fn set_statistics_visible(&mut self, visible: bool) {
        self.show_statistics = visible;
    }

    /// 获取人口变化趋势
    /// 返回值：正数表示增长，负数表示下降，0表示稳定
    pub fn get_population_trend(&self, window_size: usize) -> Option<i32> {
        if self.history.len() < window_size * 2 {
            return None;
        }

        let len = self.history.len();
        let recent_avg: f64 =
            self.history[len - window_size..].iter().sum::<usize>() as f64 / window_size as f64;

        let previous_avg: f64 = self.history[len - window_size * 2..len - window_size]
            .iter()
            .sum::<usize>() as f64
            / window_size as f64;

        let diff = recent_avg - previous_avg;
        if diff > 1.0 {
            Some(1) // 增长
        } else if diff < -1.0 {
            Some(-1) // 下降
        } else {
            Some(0) // 稳定
        }
    }

    /// 检测是否达到稳定状态
    /// 如果最近N代的人口变化很小，则认为达到稳定状态
    pub fn is_stable(&self, window_size: usize, threshold: usize) -> bool {
        if self.history.len() < window_size {
            return false;
        }

        let recent_history = &self.history[self.history.len() - window_size..];
        let max = *recent_history.iter().max().unwrap_or(&0);
        let min = *recent_history.iter().min().unwrap_or(&0);

        max - min <= threshold
    }
}

impl Default for PopulationStatistics {
    fn default() -> Self {
        Self::new(200) // 默认保存200代历史
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_statistics_creation() {
        let stats = PopulationStatistics::new(100);
        assert_eq!(stats.max_history_length, 100);
        assert!(stats.show_statistics);
        assert!(!stats.has_data());
    }

    #[test]
    fn test_add_population_data() {
        let mut stats = PopulationStatistics::new(3);
        stats.add_population(10);
        stats.add_population(15);
        stats.add_population(12);

        assert_eq!(stats.get_history_length(), 3);
        assert_eq!(stats.get_current_population(), Some(12));
        assert_eq!(stats.get_max_population(), Some(15));
        assert_eq!(stats.get_min_population(), Some(10));
    }

    #[test]
    fn test_history_length_limit() {
        let mut stats = PopulationStatistics::new(2);
        stats.add_population(10);
        stats.add_population(15);
        stats.add_population(12);
        stats.add_population(8);

        assert_eq!(stats.get_history_length(), 2);
        assert_eq!(*stats.get_history(), vec![12, 8]);
    }

    #[test]
    fn test_clear_history() {
        let mut stats = PopulationStatistics::new(10);
        stats.add_population(10);
        stats.add_population(15);

        assert!(stats.has_data());
        stats.clear_history();
        assert!(!stats.has_data());
    }

    #[test]
    fn test_average_population() {
        let mut stats = PopulationStatistics::new(10);
        stats.add_population(10);
        stats.add_population(20);
        stats.add_population(30);

        assert_eq!(stats.get_average_population(), Some(20.0));
    }

    #[test]
    fn test_stability_detection() {
        let mut stats = PopulationStatistics::new(10);
        // 添加稳定的数据
        for i in 0..5 {
            stats.add_population(100 + i % 2); // 100, 101, 100, 101, 100
        }

        assert!(stats.is_stable(5, 2));

        // 添加不稳定的数据
        stats.add_population(200);
        assert!(!stats.is_stable(6, 2));
    }
}
