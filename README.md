## viua — 终端图片查看器

将图片转换为 ASCII 字符画或直接显示原图到终端。默认使用 Sixel/iTerm2 协议显示原图，也支持半块字符和 ASCII 模式。

### 项目结构

```
viua/
├── Cargo.toml
└── src/
    ├── main.rs          # 入口：CLI + 文件收集 + 错误处理
    ├── cli.rs           # CLI 参数定义 (clap)
    ├── config.rs        # 配置结构体 + ViewMode / Algorithm 枚举
    ├── app.rs           # 业务编排：遍历文件、分派渲染器
    ├── ascii/
    │   ├── mod.rs       # AsciiPixel, AsciiArt, convert() 按算法分派
    │   ├── lum.rs       # 亮度映射算法
    │   └── charset.rs   # 默认字符集
    └── render/
        ├── mod.rs       # Renderer<T> trait
        └── ansi.rs      # ANSI truecolor 终端输出
```

### 渲染模式

| 模式 | 说明 | 实现 |
|------|------|------|
| **Image**（默认）| 原图直显 | 通过 `viuer`（Sixel / iTerm2 / Kitty 自动检测） |
| **HalfBlock** | 半块字符 ▄▀ | `viuer` BlockPrinter，强制关闭协议 |
| **ASCII** | 字符画 | 自研：Lanczos3 缩放 → NTSC 亮度 → 字符映射 |

### CLI 选项

| 参数 | 说明 |
|------|------|
| `-M, --mode` | 渲染模式：`image`（默认）/ `ascii` / `halfblock` |
| `-w, --width` | 输出宽度（0 = 自动适配终端，默认 0） |
| `-m, --monochrome` | 全模式灰度输出 |
| `-s, --charset` | ASCII 字符梯度（默认 ` .:-=+*#%@`） |
| `-i, --info` | 图片后显示文件信息页脚 |

### 使用示例

```bash
# 原图显示（默认）
viua img.png

# ASCII 字符画
viua -M ascii img.png

# 半块字符，灰度
viua -M halfblock -m img.png

# 指定宽度 + 文件信息页脚
viua -w 60 -i img.png

# 多张图片
viua img1.jpg img2.png

# 自定义字符集
viua -M ascii -s " .-+*#" img.png
```

### 核心算法（ASCII 模式）

1. Lanczos3 缩放（考虑终端字符宽高比 2:1）
2. NTSC 亮度公式：`0.299R + 0.587G + 0.114B`
3. 亮度线性映射到字符梯度（暗→亮）
4. 通过 crossterm 输出 ANSI truecolor

### 依赖

| crate | 用途 |
|-------|------|
| `image` | 图片解码与缩放 |
| `clap` | CLI 参数解析 |
| `crossterm` | 终端尺寸检测 / ANSI 颜色 |
| `viuer` | 原图直显 & 半块字符渲染（Sixel / iTerm2 / Kitty） |
