# TODO

## High Priority

- [x] **宽度校验** — 图片宽度 > 终端宽度时警告或自动裁剪，防止折行
- [x] **--mode 参数** — 支持 ascii / image / halfblock 三种模式
- [x] **stdin 管道** — 检测管道输入，追加文件列表
- [x] **六图图片信息页脚** — 居中框体显示文件名、尺寸、格式、大小；ASCII 模式署名
- [x] **图片内联定位** — viuer absolute_offset: false，在当前光标位置渲染
- [x] **monochrome 全模式支持** — ASCII/Image/HalfBlock 均支持 --monochrome 灰度输出；ASCII 默认彩色

## Medium Priority

- [x] **分隔线** — 多图之间自动插入分隔线，实时取终端宽度
- [ ] **--algorithm 参数** — 支持 lum / shape 等算法切换
- [ ] **win glob 展开** — 对位置参数做跨平台 glob（Windows cmd/pwsh 不会自动展开 *.png）
- [x] **文件不存在友好提示** — 当前 image::open 直接 panic/error

## Low Priority

- [ ] **-o / --html 输出** — 纯文本和 HTML 文件输出恢复
