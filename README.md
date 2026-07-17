## viua — 终端图片查看器

将图片转换为 ASCII 字符画或直接显示原图到终端。默认使用半块字符/truecolor 渲染，可选启用 Sixel/iTerm2/Kitty 图像协议。

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
    │   ├── clahe.rs     # CLAHE 自适应直方图均衡化
    │   ├── sobel.rs     # Sobel 边缘检测算法
    │   └── charset.rs   # 默认字符集
    └── render/
        ├── mod.rs       # Renderer<T> trait
        └── ansi.rs      # ANSI truecolor 终端输出
```

### 命令行

```
viua [全局选项] [file]...              # 默认 image 模式
viua <ascii|image|halfblock> [选项] [file]...
```

#### 全局选项

| 参数 | 说明 |
|------|------|
| `-w, --width` | 输出宽度（0 = 自动适配终端，默认 0） |
| `-H, --height` | 输出高度（0 = 自动，默认 0） |
| `-m, --monochrome` | 全模式灰度输出 |
| `-i, --info` | 图片后显示文件信息页脚 |
| `-r, --recursive` | 递归遍历目录 |
| `-1, --once` | GIF 动画只播放一次 |

#### ASCII 子命令

| 参数 | 说明 |
|------|------|
| `-a, --algorithm` | 转换算法：`lum`（默认，亮度映射）/ `lum-clahe`（CLAHE 增强）/ `sobel`（边缘检测） |
| `-s, --charset` | 字符梯度（默认 ` .:-=+*#%@`） |

### 使用示例

```bash
# 原图显示（默认）
viua img.png

# ASCII 字符画
viua ascii img.png

# ASCII + CLAHE 增强
viua ascii -a lum-clahe img.png

# ASCII Sobel 边缘检测
viua ascii -a sobel img.png

# 半块字符，灰度
viua halfblock -m img.png

# 指定宽度 + 文件信息页脚
viua -w 60 -i img.png

# 多张图片
viua img1.jpg img2.png

# 自定义字符集
viua ascii -s " .-+*#" img.png

# 管道输入
find . -name '*.png' | viua

# 递归遍历目录
viua -r ./photos

# URL 输入
viua https://example.com/img.png

# GIF 动图播放（默认循环）
viua animation.gif

# GIF 动图只播放一次
viua --once animation.gif

# GIF + 自定义宽度 + 灰度
viua -w 60 -m --once animation.gif

# GIF 半块字符模式（无图像协议时使用）
viua halfblock animation.gif
```

### 核心算法（ASCII 模式）

| 算法 | 说明 |
|------|------|
| `lum` | Lanczos3 缩放 → NTSC 亮度 `0.299R+0.587G+0.114B` → 字符映射 |
| `lum-clahe` | CLAHE 预处理 → 同上 |
| `sobel` | 灰度 → Sobel Gx/Gy 边缘检测 → 梯度幅值反转 → 字符映射（轮廓风格） |

### 依赖

| crate | 用途 |
|-------|------|
| `image` | 图片解码与缩放 |
| `clap` | CLI 参数解析 |
| `crossterm` | 终端尺寸检测 / ANSI 颜色 |
| `viuer` | 原图直显 & 半块字符渲染（可选 feature `sixel` 启用 Sixel 协议） |
| `gif` | GIF 解码与帧延迟解析 |

### 编译安装

```bash
# 默认编译（半块字符 / truecolor 渲染）
cargo build --release

# 启用 Sixel 协议支持（需要 autotools 编译依赖）
cargo build --release --features sixel
```
