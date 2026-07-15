# TODO

## Done

- [x] **宽度校验** — 图片宽度 > 终端宽度时警告或自动裁剪
- [x] **--mode 参数** — ascii / image / halfblock
- [x] **stdin 管道** — 检测管道输入，追加文件列表
- [x] **图片信息页脚** — 居中框体（文件名·尺寸·格式·大小）；ASCII 署名
- [x] **图片内联定位** — viuer absolute_offset: false
- [x] **--monochrome 全模式支持** — ASCII/Image/HalfBlock 灰度
- [x] **分隔线** — 多图之间 `─` 填充终端宽度
- [x] **文件不存在友好提示** — warning + skip
- [x] **单元测试** — 29 tests（human_size / charset / lum / ansi / cli / config）
- [x] **代码清理** — 抽 viuer_print() 消除重复；删 ensure_iterm_detection 死代码

## Pending

- [ ] **--algorithm 参数** — lum / shape 算法切换
- [ ] **win glob 展开** — Windows cmd/pwsh 不自展 `*.png`
