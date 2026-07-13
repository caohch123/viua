## asciiart — 终端 ASCII 画生成器

一个 Rust 命令行工具，将图片转换为 **彩色/单色 ASCII 字符画** 直接输出到终端。

### 核心流程

```
图片文件 → image::open → Lanczos3 缩放到指定宽度(保持宽高比×0.5) 
→ 灰度化(亮度公式) → 亮度值映射字符集索引 → crossterm ANSI truecolor 渲染输出
```

### 模块职责

| 文件 | 职责 |
|------|------|
| `main.rs` | clap CLI 解析，驱动主流程 |
| `config.rs` | 配置参数结构体 |
| `ascii.rs` | 图像加载→缩放→灰度→字符映射（核心） |
| `render.rs` | 逐行输出字符 + 可选 ANSI 颜色 |
| `charset.rs` | 字符集常量 |

### 依赖

- `image 0.24` — 图片解码、缩放
- `clap 4.4` — 命令行参数（源自 viu）
- `crossterm 0.27` — 终端 ANSI truecolor 输出

### 用法

```bash
# 基础
asciiart input.jpg

# 彩色 + 宽 100
asciiart -w 100 -c input.png

# 密集字符集 + 文件名
asciiart -s " .:-=+*#%@" -n input.jpg
```
