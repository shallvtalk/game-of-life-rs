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
