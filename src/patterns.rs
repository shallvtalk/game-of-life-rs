/// 康威生命游戏预设图案模块
/// 包含各种经典的生命游戏图案
/// 图案数据结构
pub struct Pattern {
    pub name: &'static str,
    pub description: &'static str,
    pub data: &'static [&'static str],
}

/// 振荡器类图案 - 会周期性变化但保持在原位置
pub mod oscillators {
    use super::Pattern;

    /// 闪烁灯 - 最简单的振荡器，周期为2
    pub const BLINKER: Pattern = Pattern {
        name: "Blinker",
        description: "Simple oscillator, period 2",
        data: &["OOO"],
    };

    /// 蟾蜍 - 周期为2的振荡器
    pub const TOAD: Pattern = Pattern {
        name: "Toad",
        description: "Oscillator, period 2",
        data: &[" OOO", "OOO "],
    };

    /// 信标 - 周期为2的振荡器
    pub const BEACON: Pattern = Pattern {
        name: "Beacon",
        description: "Oscillator, period 2",
        data: &["OO  ", "O   ", "   O", "  OO"],
    };

    /// 脉冲星 - 周期为3的经典振荡器
    pub const PULSAR: Pattern = Pattern {
        name: "Pulsar",
        description: "Classic oscillator, period 3",
        data: &[
            "  OOO   OOO  ",
            "             ",
            "O    O O    O",
            "O    O O    O",
            "O    O O    O",
            "  OOO   OOO  ",
            "             ",
            "  OOO   OOO  ",
            "O    O O    O",
            "O    O O    O",
            "O    O O    O",
            "             ",
            "  OOO   OOO  ",
        ],
    };

    /// 十五项全能 - 周期为15的振荡器
    pub const PENTADECATHLON: Pattern = Pattern {
        name: "Pentadecathlon",
        description: "Oscillator, period 15",
        data: &[
            "  O    O  ",
            " OO    OO ",
            "O  OOOO  O",
            " OO    OO ",
            "  O    O  ",
        ],
    };
}

/// 飞船类图案 - 会在网格中移动
pub mod spaceships {
    use super::Pattern;

    /// 滑翔机 - 最小的飞船，周期为4，每4代向右下移动一格
    pub const GLIDER: Pattern = Pattern {
        name: "Glider",
        description: "Smallest spaceship, moves diagonally",
        data: &[" O ", "  O", "OOO"],
    };

    /// 轻量级飞船
    pub const LWSS: Pattern = Pattern {
        name: "LWSS",
        description: "Lightweight spaceship",
        data: &[" OOOO", "O   O", "    O", "O  O "],
    };

    /// 中量级飞船
    pub const MWSS: Pattern = Pattern {
        name: "MWSS",
        description: "Middleweight spaceship",
        data: &["  O   ", " OOOO ", "O    O", "     O", "O   O "],
    };

    /// 重量级飞船
    pub const HWSS: Pattern = Pattern {
        name: "HWSS",
        description: "Heavyweight spaceship",
        data: &["   O   ", "  OOOO ", " O    O", "      O", " O   O "],
    };
}

/// 枪类图案 - 会持续产生其他移动图案
pub mod guns {
    use super::Pattern;

    /// 高斯哈滑翔机生产线 - 最经典的枪，每30代产生一个滑翔机
    pub const GOSPER_GLIDER_GUN: Pattern = Pattern {
        name: "Gosper Gun",
        description: "Classic glider gun, period 30",
        data: &[
            "                        O           ",
            "                      O O           ",
            "            OO      OO            OO",
            "           O   O    OO            OO",
            "OO        O     O   OO              ",
            "OO        O   O OO    O O           ",
            "          O     O       O           ",
            "           O   O                    ",
            "            OO                      ",
        ],
    };

    /// 辛金滑翔机生产线 - 另一种经典的滑翔机枪
    pub const SIMKIN_GLIDER_GUN: Pattern = Pattern {
        name: "Simkin Gun",
        description: "Glider gun, period 120",
        data: &[
            "OO   OO                ",
            "OO   OO                ",
            "                       ",
            "    OO                 ",
            "    OO                 ",
            "                       ",
            "                       ",
            "                       ",
            "                       ",
            "                       ",
            "                OO  OO ",
            "                OO  OO ",
            "                       ",
            "                       ",
            "                   OOOO",
            "                 OO   O",
            "                 O     ",
            "                  O   O",
            "                   OOOO",
        ],
    };
}

/// 其他有趣的结构
pub mod miscellaneous {
    use super::Pattern;

    /// R-五连块 - 演化过程极其复杂的小结构
    pub const R_PENTOMINO: Pattern = Pattern {
        name: "R-Pentomino",
        description: "Complex evolution from simple start",
        data: &[" OO", "OO ", " O "],
    };

    /// 死硬 - 会存活一段时间后完全消失
    pub const DIEHARD: Pattern = Pattern {
        name: "Diehard",
        description: "Dies after 130 generations",
        data: &["      O ", "OO      ", " O   OOO"],
    };

    /// 橡实 - 从简单开始演化出复杂图形
    pub const ACORN: Pattern = Pattern {
        name: "Acorn",
        description: "Grows into complex pattern",
        data: &[" O     ", "   O   ", "OO  OOO"],
    };

    /// 简单的方块 - 静态不变的图案
    pub const BLOCK: Pattern = Pattern {
        name: "Block",
        description: "Still life - never changes",
        data: &["OO", "OO"],
    };
}

/// 获取所有预设图案的列表
pub fn get_all_patterns() -> Vec<(&'static str, Vec<&'static Pattern>)> {
    vec![
        (
            "Oscillators",
            vec![
                &oscillators::BLINKER,
                &oscillators::TOAD,
                &oscillators::BEACON,
                &oscillators::PULSAR,
                &oscillators::PENTADECATHLON,
            ],
        ),
        (
            "Spaceships",
            vec![
                &spaceships::GLIDER,
                &spaceships::LWSS,
                &spaceships::MWSS,
                &spaceships::HWSS,
            ],
        ),
        (
            "Guns",
            vec![&guns::GOSPER_GLIDER_GUN, &guns::SIMKIN_GLIDER_GUN],
        ),
        (
            "Miscellaneous",
            vec![
                &miscellaneous::R_PENTOMINO,
                &miscellaneous::DIEHARD,
                &miscellaneous::ACORN,
                &miscellaneous::BLOCK,
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_structure() {
        let blinker = &oscillators::BLINKER;
        assert_eq!(blinker.name, "Blinker");
        assert_eq!(blinker.description, "Simple oscillator, period 2");
        assert_eq!(blinker.data.len(), 1);
        assert_eq!(blinker.data[0], "OOO");
    }

    #[test]
    fn test_get_all_patterns() {
        let patterns = get_all_patterns();

        // Should have 4 categories
        assert_eq!(patterns.len(), 4);

        // Check category names
        let category_names: Vec<&str> = patterns.iter().map(|(name, _)| *name).collect();
        assert!(category_names.contains(&"Oscillators"));
        assert!(category_names.contains(&"Spaceships"));
        assert!(category_names.contains(&"Guns"));
        assert!(category_names.contains(&"Miscellaneous"));

        // Check that each category has patterns
        for (_, patterns_in_category) in patterns {
            assert!(!patterns_in_category.is_empty());
        }
    }

    #[test]
    fn test_oscillator_patterns() {
        // Test that blinker pattern has correct dimensions
        let blinker = &oscillators::BLINKER;
        assert_eq!(blinker.data.len(), 1);
        assert_eq!(blinker.data[0].len(), 3);

        // Test that pulsar pattern has correct dimensions
        let pulsar = &oscillators::PULSAR;
        assert_eq!(pulsar.data.len(), 13);
        assert_eq!(pulsar.data[0].len(), 13);
    }

    #[test]
    fn test_spaceship_patterns() {
        // Test glider pattern
        let glider = &spaceships::GLIDER;
        assert_eq!(glider.data.len(), 3);
        assert_eq!(glider.data[0], " O ");
        assert_eq!(glider.data[1], "  O");
        assert_eq!(glider.data[2], "OOO");

        // Test that spaceships have reasonable sizes
        let lwss = &spaceships::LWSS;
        assert_eq!(lwss.data.len(), 4);
        assert!(lwss.data[0].len() >= 4);
    }

    #[test]
    fn test_gun_patterns() {
        // Test that gun patterns are reasonably large
        let gosper_gun = &guns::GOSPER_GLIDER_GUN;
        assert!(gosper_gun.data.len() >= 9);
        assert!(gosper_gun.data[0].len() >= 30);

        let simkin_gun = &guns::SIMKIN_GLIDER_GUN;
        assert!(simkin_gun.data.len() >= 18);
        assert!(simkin_gun.data[0].len() >= 20);
    }

    #[test]
    fn test_miscellaneous_patterns() {
        // Test block pattern (still life)
        let block = &miscellaneous::BLOCK;
        assert_eq!(block.data.len(), 2);
        assert_eq!(block.data[0], "OO");
        assert_eq!(block.data[1], "OO");

        // Test R-pentomino
        let r_pentomino = &miscellaneous::R_PENTOMINO;
        assert_eq!(r_pentomino.data.len(), 3);
    }
}
