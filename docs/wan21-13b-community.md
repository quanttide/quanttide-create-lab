# Wan2.1-T2V-1.3B 社区用法

本地跑的就是这个模型（`wan2.1_t2v_1.3B_fp16.safetensors`，2.7G）。以下是 2026-11-21 查证的官方与社区实际用法，出处见文末。

## 官方定位

- 只要 **8.19 GB 显存**，兼容几乎所有消费级显卡；RTX 4090 上 5 秒 480P 约 4 分钟（未量化），官方称表现可比肩部分闭源模型。
- 官方推荐 **480P**：720P 能出片但该分辨率训练少、不稳。
- 官方示例参数 **guide_scale 6、shift 8–12**——run-shot 的缺省（cfg 6、shift 8）就是官方推荐值。
- ComfyUI 官方教程的 T2V 工作流**默认文件就是 `wan2.1_t2v_1.3B_fp16.safetensors`**（原文 "This workflow requires the …"，14B 可替换）——本地跑的是同一条官方路。

## 社区在 1.3B 本体上做成了什么

- **EchoShot**（西安交大 + 阿里云，arXiv 2506.15838）：多镜头同角色人像视频——按提示词生成多条**同一个人物**的镜头，内容可控。分镜短片的直接同路。
- **Phantom**（字节，ICCV 2025）：单/多主体参考一致性视频生成，放出 Phantom-Wan-1.3B 权重；kijai 的 ComfyUI-WanVideoWrapper 已适配 1.3B。角色一致性的现成轮子。
- **免训练视频编辑**（WANAlign2.1 / FlowEdit / FlowAlign，FlowAlign 入选 ICLR 26）：不训练、不反演直接改视频，A100 上 75 秒改一条（其对比：WANEdit2.1 150 秒、FlowDirector 540 秒）；已被官方 Wan 仓收录进 Community Works。
- **VACE-1.3B**（通义，ICCV 2025）：参考图控视频、视频编辑的一体化框架，官方直接提供 1.3B 权重与 ComfyUI 集成。

## 提速与降本的社区轮子

Wan 官方 Community Works 收录：TeaCache、CFG-Zero*（加速/提质）、DiffSynth-Studio（模型库与示例）、LightX2V（官方介绍其可在 RTX 4060 8GB 上跑 Wan 系）。

## 与本地的印证

- 官方说 8.19 GB 显存起步；本地 GTX1060 3GB 靠 CPU offload 已出 15 条 480×288/17 帧 mp4（2026-11-20）——「消费卡友好」属实，代价是慢。
- journal 2026-11-20 记的两条经验（seed 比提示词重要、视频只需生成图间关系）与社区方向一致：一致性靠参考与多镜头项目（EchoShot、Phantom），不靠堆提示词。

## 出处（2026-11-21 抓取）

- <https://github.com/Wan-Video/Wan2.1> —— 规格、官方参数、Community Works
- <https://comfyanonymous.github.io/ComfyUI_examples/wan/> —— 官方教程默认 1.3B
- <https://github.com/JoHnneyWang/EchoShot>
- <https://github.com/Phantom-video/Phantom>
- <https://github.com/KyujinHan/Awesome-Training-Free-WAN2.1-Editing>
- <https://github.com/ali-vilab/VACE>
- <https://github.com/ModelTC/LightX2V>
