# CHANGELOG

所有显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)。

版本遵循语义化版本规范：0.0.x（探索期）→ 0.x.y（验证期）→ x.y.z（正式期）

---

## [Unreleased]

### 变更

- `docs/wan21-13b-community.md` 同步接入进度：VACE-1.3B 已跑通四条实验（结论回写《先预演后渲染》，生产下限 14B Q4），另三个轮子未接入；镜卡拆档后的失效路径一并修正

### 新增

- 新增 `docs/wan21-13b-community.md`：Wan2.1-T2V-1.3B 官方规格与社区成功用法查证（EchoShot 多镜头同角色、Phantom 主体一致性、免训练编辑、VACE-1.3B，均带出处），并附对到本仓 profile 素材的逐条对照
- 新增 `src/cli`（Rust，quanttide-create-lab-cli）：`run-shot` 跑一镜视频生成实验——服务没起则拉起 ComfyUI Desktop、提交 graph、等完成、拷产物、写 run.md，留档绑在跑上
- 仓库初始化：README、CHANGELOG 与许可
