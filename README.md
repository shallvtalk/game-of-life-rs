# Conway's Game of Life

A Rust implementation of Conway's Game of Life with a graphical user interface built using egui/eframe.

## Features

- **Interactive GUI**: Modern, responsive interface with real-time controls
- **Mouse Interaction**: Click and drag to draw living cells directly on the grid
- **Save/Load System**: ✅ **IMPLEMENTED** - Save and load game states to/from files
  - Support for .gol, .json, and .rle file formats
  - RLE (Run Length Encoded) format for standard Game of Life pattern sharing
  - Preserves grid state, generation count, and all settings
  - File dialog integration for easy file management
  - Comprehensive error handling and status feedback
- **Rich Presets**: Extensive collection of classic patterns including:
  - **Oscillators**: Blinker, Toad, Beacon, Pulsar, Pentadecathlon
  - **Spaceships**: Glider, LWSS, MWSS, HWSS
  - **Guns**: Gosper Glider Gun, Simkin Glider Gun
  - **Miscellaneous**: R-Pentomino, Diehard, Acorn, Block
- **Configurable Parameters**: Adjustable grid size, update speed, and cell density
- **Generation Tracking**: Real-time display of current generation count
- **Game Controls**: Play/pause, step-by-step execution, clear, and randomize

## Screenshots

The game features a clean interface with:

- Left sidebar for controls and presets
- Central grid area for the game visualization
- Real-time generation counter
- Organized preset categories with descriptions

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/yourusername/game_of_life.git
cd game_of_life
```

2. Build and run:

```bash
cargo run --release
```

## Usage

### Basic Controls

- **Start/Pause**: Begin or pause the automatic evolution
- **Step**: Advance the simulation by one generation
- **Clear**: Remove all living cells from the grid
- **Random**: Populate the grid with random living cells
- **Save**: Save current game state to a file (.gol or .json format)
- **Load**: Load a previously saved game state from file

### Mouse Interaction

- **Click**: Toggle individual cells between alive and dead
- **Click and Drag**: Draw continuous patterns by dragging across the grid

### Configuration

- **Update Speed**: Control simulation speed (1-30 FPS)
- **Grid Size**: Adjust grid dimensions (10-200 width, 10-150 height)
- **Random Density**: Set the probability of cells being alive when randomizing

### Save/Load Functionality

Preserve and share your game states:

- **Save**: Click the Save button to export your current game state
  - Choose between .gol (Game of Life), .json, or .rle file formats
  - .rle format is the standard format for sharing Game of Life patterns
  - Saves grid state, generation count, and all current settings
  - Default filename: `game_state.gol`
- **Load**: Click the Load button to import a previously saved game
  - Supports .gol, .json, and .rle file formats
  - RLE files can be downloaded from online pattern libraries
  - Automatically restores all game settings and grid configuration
  - Status messages confirm successful operations or report errors

### Presets

Browse organized categories of classic patterns:

- Select any preset to load it centered on the grid
- Each preset includes a description of its behavior
- Patterns are automatically positioned in the center of the current grid

## Project Structure

```
src/
├── main.rs         # Application entry point and core structure
├── game.rs         # Game logic and Conway's Game of Life rules
├── ui.rs           # User interface rendering and interaction
├── patterns.rs     # Preset pattern definitions
└── save_load.rs    # Save/load functionality with JSON serialization
```

## Technical Details

- **Framework**: Built with egui/eframe for cross-platform GUI
- **Architecture**: Modular design with clear separation of concerns
- **Performance**: Optimized for smooth real-time simulation
- **Memory**: Efficient grid representation using 1D vector for better cache performance
- **Serialization**: JSON-based save/load system using serde
- **File Management**: Native file dialogs with rfd crate
- **Dependencies**: serde, serde_json, rfd, chrono for enhanced functionality

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes following conventional commits
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Follow Rust standard formatting with `cargo fmt`
- Ensure code passes `cargo clippy` lints
- Add tests for new functionality
- Update documentation as needed

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- John Conway for creating the Game of Life
- The egui community for the excellent GUI framework
- Classic pattern contributors and the cellular automata community

## Conway's Game of Life Rules

The Game of Life follows simple rules:

1. **Survival**: A living cell with 2 or 3 neighbors survives
2. **Birth**: A dead cell with exactly 3 neighbors becomes alive
3. **Death**: All other living cells die (underpopulation or overpopulation)

These simple rules create surprisingly complex and beautiful patterns!

## Future Improvements / 未来改进建议

### Core Feature Enhancements / 核心功能增强

#### Save/Load Functionality / 保存/加载功能

- ✅ **COMPLETED** Save current grid state to file (JSON format) / 将当前网格状态保存为文件（JSON 格式）
- ✅ **COMPLETED** Load previously saved game states / 从文件加载之前保存的游戏状态
- ✅ **COMPLETED** Support RLE format import/export (Game of Life standard) / 支持 RLE 格式导入/导出（生命游戏标准格式）

#### History and Playback / 历史记录和回放

- Implement generation history with undo/redo capabilities / 实现代数历史记录，允许回退到之前的状态
- Add play/pause/reverse controls / 添加播放/暂停/倒放控制
- GIF animation export functionality / 生成 GIF 动画导出功能

#### Performance Optimization / 性能优化

- Implement HashLife algorithm for large-scale simulation / 实现 HashLife 算法用于大规模模拟
- Add multi-threading support for improved computation speed / 添加多线程支持以提升计算速度
- Implement boundary detection to compute only active regions / 实现边界检测，只计算活跃区域

### User Experience Improvements / 用户体验改进

#### Visual and Interface Enhancements / 视觉和界面增强

- ✅ **COMPLETED** Add color theme switching (dark/light mode) / 添加颜色主题切换（深色/浅色模式）
- ✅ **COMPLETED** Support zoom functionality (mouse wheel zooming) / 支持缩放功能（鼠标滚轮缩放）
- ✅ **COMPLETED** Add grid line display toggle / 添加网格线显示开关
- Implement cell age visualization (color gradient by survival time) / 实现细胞年龄可视化（颜色渐变显示存活时间）

#### Tools and Drawing Features / 工具和绘制功能

- Add brush tools (different brush sizes) / 添加画笔工具（不同大小的笔刷）
- Implement select/copy/paste region functionality / 实现选择/复制/粘贴区域功能
- Add shape drawing tools (lines, rectangles, circles) / 添加形状绘制工具（线条、矩形、圆形）
- Support pattern rotation and mirroring / 支持图案旋转和镜像

### Advanced Features / 高级功能

#### Statistics and Analysis / 统计和分析

- Real-time live cell count statistics / 实时显示活细胞数量统计
- Add population growth charts / 添加人口增长图表
- Detect stable states and periodic patterns / 检测稳定状态和周期性图案
- Add pattern recognition (auto-identify known patterns) / 添加模式识别（自动识别已知图案）

#### Extended Rule Support / 扩展规则支持

- Support other cellular automaton rules (e.g., Highlife, Day & Night) / 支持其他细胞自动机规则（如 Highlife、Day & Night）
- Allow users to define custom rules / 允许用户自定义规则
- Support larger neighborhoods (Moore/von Neumann) / 支持更大的邻域（如 Moore/von Neumann）

#### Network and Social Features / 网络和社交功能

- Online pattern library browsing and downloading / 在线图案库浏览和下载
- Share custom pattern functionality / 分享自定义图案功能
- Add more preset pattern categories / 添加更多预设图案分类

### Technical Improvements / 技术改进

#### Code Quality and Maintainability / 代码质量和维护性

- Add more unit tests and integration tests / 添加更多单元测试和集成测试
- Implement benchmark testing / 实现基准测试
- Add CLI mode for headless operation / 添加 CLI 模式支持无头运行
- Improve error handling and user feedback / 改进错误处理和用户反馈

#### Cross-platform and Deployment / 跨平台和部署

- Web version support (WASM compilation) / Web 版本支持（WASM 编译）
- Mobile adaptation / 移动端适配
- Application packaging and distribution optimization / 应用打包和分发优化

### Recommended Implementation Priority / 推荐实现优先级

1. ✅ **COMPLETED: Save/Load Functionality** - Essential for preserving work / 保存/加载功能 - 保存工作的基础需求
2. **Zoom and Visual Improvements** - Better user experience / 缩放和视觉改进 - 更好的用户体验
3. **History and Playback** - Practical utility / 历史记录回放 - 实用功能
4. **Statistics and Analysis** - Educational value / 统计和分析 - 教育价值
5. **More Preset Patterns** - Content enrichment / 更多预设图案 - 内容丰富

### Contributing to Improvements / 参与改进

If you're interested in implementing any of these features, please:

1. Check existing issues and pull requests
2. Create an issue to discuss the feature before starting
3. Follow the contributing guidelines
4. Ensure proper testing and documentation

如果您有兴趣实现任何这些功能，请：

1. 检查现有的问题和拉取请求
2. 在开始之前创建问题来讨论功能
3. 遵循贡献指南
4. 确保适当的测试和文档
