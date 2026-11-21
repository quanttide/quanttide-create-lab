# quanttide-create-lab-cli

量潮创意工作实验室命令行——跑一镜视频生成实验并自动落档。

实验室先试、试成扶正：跑镜命令面、落档格式、服务起停都先落这儿。本包只做实验，不承诺稳定，也不发布（`publish = false`）。

## 装与跑

在实验室根目录执行（`cargo run` 保持调用方工作目录，`experiments/` 才落对地方）：

```bash
cargo run --manifest-path src/cli/Cargo.toml -- --help
cargo run --manifest-path src/cli/Cargo.toml -- run-shot shot1 --prompt "靠窗卡座，慢推近"
```

产物与 `run.md` 落到 `experiments/shot1/`；服务没起会拉起 ComfyUI Desktop（`--no-start` 只等待）。

## 依赖与许可

`Cargo.toml` 的每个直接依赖列在这里，写明用途与许可；新增依赖先入表。

| 依赖 | 版本 | 许可 | 用途 |
| :-- | :-- | :-- | :-- |
| `clap` | 4 | MIT OR Apache-2.0 | 命令行解析（derive） |
| `serde_json` | 1 | MIT OR Apache-2.0 | 构造提交 graph、解析 API 响应 |
| `ureq` | 2 | MIT OR Apache-2.0 | 访问 ComfyUI HTTP API |
| `chrono` | 0.4 | MIT OR Apache-2.0 | run.md 时间戳 |

## 许可

Apache-2.0
