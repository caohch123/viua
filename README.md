## viuva — 终端 ASCII 画生成器

一个 Rust 命令行工具，将图片转换为 **彩色/单色 ASCII 字符画** 直接输出到终端。


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

**核心算法：**
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
**效果图：**
<img width="1550" height="1572" alt="435b97b0dcc56f0dc46db3bb990593b6" src="https://github.com/user-attachments/assets/98780d8f-3ee2-4142-b6e8-c634496a132a" />
<img width="1246" height="1386" alt="c5456258e4622035025dfaf5cb1b622e" src="https://github.com/user-attachments/assets/7f318595-dbe4-4206-903c-153e6c8ac282" />
<img width="1242" height="1398" alt="65ac084bc2bdabf02ce24325644cae77" src="https://github.com/user-attachments/assets/777a09e8-02e7-4a0f-9ef1-baa46fee6776" />

