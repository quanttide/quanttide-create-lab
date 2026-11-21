# CHANGELOG

所有显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)。

版本遵循语义化版本规范：0.0.x（探索期）→ 0.x.y（验证期）→ x.y.z（正式期）

---

## [Unreleased]

### 新增

- 新增 `docs/wan21-13b-community.md`：Wan2.1-T2V-1.3B 官方规格与社区成功用法查证（EchoShot 多镜头同角色、Phantom 主体一致性、免训练编辑、VACE-1.3B，均带出处）
- 新增 `src/cli`（Rust，quanttide-create-lab-cli）：`run-shot` 跑一镜视频生成实验——服务没起则拉起 ComfyUI Desktop、提交 graph、等完成、拷产物、写 run.md，留档绑在跑上
- 仓库初始化：README、CHANGELOG 与许可
