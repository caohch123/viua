## asciiart — 终端 ASCII 画生成器

一个 Rust 命令行工具，将图片转换为 **彩色/单色 ASCII 字符画** 直接输出到终端。

### 核心流程

```
图片文件 → image::open → Lanczos3 缩放到指定宽度(保持宽高比×0.5) 
→ 灰度化(亮度公式) → 亮度值映射字符集索引 → crossterm ANSI truecolor 渲染输出
```

项目已完成。总结如下：

---

### 新建项目 `asciiart` 位于 `C:\Users\asus\Desktop\RUSt大作业\asciiart`

**项目结构：**
```
asciiart/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI 入口 (clap 参数解析)
    ├── config.rs    # 配置结构体
    ├── ascii.rs     # ASCII 转换核心 (图像缩放 + 灰度映射)
    ├── render.rs    # 终端渲染 (crossterm ANSI color)
    └── charset.rs   # 字符集定义
```

**核心算法**（来自 shoalart 的简化 + viu 的 CLI 风格）：
1. 加载图像 → Lanczos3 缩放（考虑终端字符宽高比 2:1）
2. 灰度转换（`0.299R + 0.587G + 0.114B`）
3. 亮度映射到字符集（暗→亮），字符集可自定义
4. 通过 crossterm 输出 ANSI truecolor

**CLI 选项：**
| 参数 | 说明 |
|------|------|
| `-w, --width` | 输出宽度（字符数，默认80） |
| `-c, --color` | ANSI 真彩色输出 |
| `-s, --charset` | 自定义字符集（默认 ` .:-=+*#%@`） |
| `-n, --name` | 输出前打印文件名 |
| `-t, --caption` | 输出后打印文件名 |

**使用示例：**
```bash
cargo run -- image.jpg                              # 基础 ASCII
cargo run -- -w 120 -c image.png                    # 宽120 + 彩色
cargo run -- -s " .-+*#" --color image.png          # 自定义字符集
```
