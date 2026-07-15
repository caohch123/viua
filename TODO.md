# TODO

## High Priority

- [ ] **宽度校验** — 图片宽度 > 终端宽度时警告或自动裁剪，防止折行
- [ ] **--mode 参数** — 支持 ascii / image / halfblock 三种模式
- [ ] **stdin 管道** — 检测管道输入，追加文件列表

## Medium Priority

- [ ] **分隔线** — 多图之间自动插入分隔线，实时取终端宽度
- [ ] **--algorithm 参数** — 支持 lum / shape 等算法切换
- [ ] **win glob 展开** — 对位置参数做跨平台 glob（Windows cmd/pwsh 不会自动展开 *.png）
- [ ] **文件不存在友好提示** — 当前 image::open 直接 panic/error

## Low Priority

- [ ] **-o / --html 输出** — 纯文本和 HTML 文件输出恢复
