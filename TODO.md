# TODO

## Done

- [x] 宽度校验 · --mode · stdin 管道 · 信息页脚 · 内联定位
- [x] --monochrome · 分隔线 · 文件不存在提示
- [x] 单元测试 29 tests · 代码清理

## Near Term

- [ ] **多算法支持** — 当前仅实现亮度映射，后续扩展：结构相似性匹配（SSIM）、边缘检测 + 方向梯度映射、CLAHE 预处理等
- [x] **win glob 展开** — Windows 不自展 `*.png`
- [x] **--height 参数** — 与 --width 对称

## Backlog

- [ ] **GIF 动图播放** — viuer 原生支持，加帧循环
- [x] **递归目录** `-r` — 直接传入文件夹
- [x] **URL 输入** — `viua https://...` 下载并显示
- [x] **集成测试** — e2e golden test
- [ ] **CI/CD** — GitHub Actions auto test + lint
