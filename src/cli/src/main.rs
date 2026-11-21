//! quanttide-create-lab-cli：创意工作实验室命令行。
//!
//! 跑一镜视频生成实验：等服务（没起就拉起 ComfyUI Desktop）→ 提交 → 等完成 →
//! 拷产物 → 写 run.md。留档绑在跑上，实验跑一次自动长一篇。
//! 实验室先试、试成扶正；本包只做实验，不承诺稳定。

use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const CLIP_NAME: &str = "umt5_xxl_fp8_e4m3fn_scaled.safetensors";
const VAE_NAME: &str = "wan_2.1_vae.safetensors";
const UNET_NAME: &str = "wan2.1_t2v_1.3B_fp16.safetensors";
const DEFAULT_NEGATIVE: &str =
    "色调艳丽，过曝，静态，细节模糊不清，字幕，风格，作品，画作，画面，静止，整体发灰";

#[derive(Parser)]
#[command(name = "quanttide-create-lab", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 跑一镜视频生成实验，产物与 run.md 落到 experiments/<name>/
    RunShot {
        /// 实验名：目录与产物文件名（如 r20b-shot3），不含路径分隔符
        name: String,
        /// 正向提示词
        #[arg(long)]
        prompt: String,
        /// 负向提示词（缺省用通用负词）
        #[arg(long)]
        negative: Option<String>,
        /// 随机种子；缺省取当前秒数，实际值写进 run.md
        #[arg(long)]
        seed: Option<u64>,
        #[arg(long, default_value_t = 480)]
        width: u32,
        #[arg(long, default_value_t = 288)]
        height: u32,
        /// latent 帧数（17 ≈ 1 秒 @16fps）
        #[arg(long, default_value_t = 17)]
        frames: u32,
        #[arg(long, default_value_t = 20)]
        steps: u32,
        #[arg(long, default_value_t = 6.0)]
        cfg: f64,
        #[arg(long, default_value_t = 16)]
        fps: u32,
        /// ComfyUI API 地址
        #[arg(long, default_value = "http://127.0.0.1:8188")]
        server: String,
        /// ComfyUI 输出目录（SaveVideo 落盘处）
        #[arg(long, default_value = "~/ComfyUI-Shared/output")]
        output: String,
        /// 落档目录（在实验室根目录执行）
        #[arg(long, default_value = "experiments")]
        experiments: String,
        /// 服务未起时最多等多久（秒）
        #[arg(long, default_value_t = 180)]
        wait_secs: u64,
        /// 生成超时（秒）
        #[arg(long, default_value_t = 1800)]
        timeout_secs: u64,
        /// 不自动启动 ComfyUI Desktop，只等待
        #[arg(long)]
        no_start: bool,
    },
}

struct Shot {
    name: String,
    prompt: String,
    negative: String,
    seed: u64,
    width: u32,
    height: u32,
    frames: u32,
    steps: u32,
    cfg: f64,
    fps: u32,
    server: String,
    output: PathBuf,
    experiments: PathBuf,
    wait_secs: u64,
    timeout_secs: u64,
    no_start: bool,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::RunShot {
            name,
            prompt,
            negative,
            seed,
            width,
            height,
            frames,
            steps,
            cfg,
            fps,
            server,
            output,
            experiments,
            wait_secs,
            timeout_secs,
            no_start,
        } => run_shot(Shot {
            name,
            prompt,
            negative: negative.unwrap_or_else(|| DEFAULT_NEGATIVE.to_string()),
            seed: seed.unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            }),
            width,
            height,
            frames,
            steps,
            cfg,
            fps,
            server,
            output: PathBuf::from(expand_tilde(&output)),
            experiments: PathBuf::from(experiments),
            wait_secs,
            timeout_secs,
            no_start,
        }),
    };
    if let Err(e) = result {
        eprintln!("错误：{e}");
        std::process::exit(1);
    }
}

fn expand_tilde(p: &str) -> String {
    match p.strip_prefix("~/") {
        Some(rest) => {
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{home}/{rest}")
        }
        None => p.to_string(),
    }
}

fn graph(s: &Shot) -> Value {
    json!({
        "1": {"class_type": "CLIPLoader", "inputs": {"clip_name": CLIP_NAME, "type": "wan", "device": "cpu"}},
        "2": {"class_type": "VAELoader", "inputs": {"vae_name": VAE_NAME}},
        "3": {"class_type": "UNETLoader", "inputs": {"unet_name": UNET_NAME, "weight_dtype": "default"}},
        "4": {"class_type": "ModelSamplingSD3", "inputs": {"model": ["3", 0], "shift": 8}},
        "5": {"class_type": "CLIPTextEncode", "inputs": {"clip": ["1", 0], "text": s.prompt}},
        "6": {"class_type": "CLIPTextEncode", "inputs": {"clip": ["1", 0], "text": s.negative}},
        "7": {"class_type": "EmptyHunyuanLatentVideo", "inputs": {"width": s.width, "height": s.height, "length": s.frames, "batch_size": 1}},
        "8": {"class_type": "KSampler", "inputs": {
            "model": ["4", 0], "seed": s.seed, "steps": s.steps, "cfg": s.cfg,
            "sampler_name": "uni_pc", "scheduler": "simple",
            "positive": ["5", 0], "negative": ["6", 0], "latent_image": ["7", 0], "denoise": 1}},
        "9": {"class_type": "VAEDecode", "inputs": {"samples": ["8", 0], "vae": ["2", 0]}},
        "10": {"class_type": "CreateVideo", "inputs": {"images": ["9", 0], "fps": s.fps}},
        "11": {"class_type": "SaveVideo", "inputs": {
            "video": ["10", 0], "filename_prefix": format!("video/{}", s.name), "format": "auto", "codec": "auto"}}
    })
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()
}

fn server_up(a: &ureq::Agent, server: &str) -> bool {
    a.get(&format!("{server}/system_stats"))
        .timeout(Duration::from_secs(5))
        .call()
        .is_ok()
}

/// 找 ComfyUI Desktop 的启动器：先读桌面入口 Exec，再兜底下载目录的 AppImage。
fn find_comfyui() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let apps = Path::new(&home).join(".local/share/applications");
    if let Ok(rd) = std::fs::read_dir(&apps) {
        for entry in rd.flatten() {
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if fname.ends_with(".desktop")
                && fname.contains("comfy")
                && let Ok(txt) = std::fs::read_to_string(entry.path())
            {
                for line in txt.lines() {
                    if let Some(exec) = line.strip_prefix("Exec=")
                        && let Some(first) = exec.split_whitespace().next()
                    {
                        let p = PathBuf::from(first);
                        if p.exists() {
                            return Some(p);
                        }
                    }
                }
            }
        }
    }
    let dl = Path::new(&home).join("下载");
    let mut hits: Vec<PathBuf> = std::fs::read_dir(&dl)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            let n = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            n.ends_with(".appimage") && n.contains("comfy")
        })
        .collect();
    hits.sort();
    hits.pop()
}

fn ensure_server(a: &ureq::Agent, s: &Shot) -> Result<(), String> {
    if server_up(a, &s.server) {
        return Ok(());
    }
    let deadline = Instant::now() + Duration::from_secs(s.wait_secs);
    if !s.no_start {
        let launcher = find_comfyui()
            .ok_or_else(|| "ComfyUI 未运行，也没找到 ComfyUI Desktop 启动器".to_string())?;
        println!("服务未起，启动 {}", launcher.display());
        Command::new(&launcher)
            .arg("--no-sandbox")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| format!("启动 {} 失败：{e}", launcher.display()))?;
    }
    while Instant::now() < deadline {
        if server_up(a, &s.server) {
            println!("服务已就绪 {}", s.server);
            return Ok(());
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    Err(format!(
        "等了 {}s，{} 仍未响应（先启动 ComfyUI Desktop，或调大 --wait-secs）",
        s.wait_secs, s.server
    ))
}

fn submit(a: &ureq::Agent, s: &Shot) -> Result<String, String> {
    let payload = json!({"prompt": graph(s), "client_id": format!("cli-{}", s.name)});
    let url = format!("{}/prompt", s.server);
    let resp = a
        .post(&url)
        .set("Content-Type", "application/json")
        .send_string(&payload.to_string());
    let body = match resp {
        Ok(r) => r.into_string().map_err(|e| e.to_string())?,
        Err(ureq::Error::Status(_, r)) => {
            return Err(format!("提交被拒：{}", r.into_string().unwrap_or_default()));
        }
        Err(e) => return Err(format!("提交失败：{e}")),
    };
    let v: Value = serde_json::from_str(&body).map_err(|e| format!("响应不是 JSON：{e}"))?;
    if let Some(err) = v.get("error") {
        return Err(format!("节点错误：{err}"));
    }
    v.get("prompt_id")
        .and_then(|p| p.as_str())
        .map(|p| p.to_string())
        .ok_or_else(|| format!("响应里没有 prompt_id：{body}"))
}

fn wait_done(a: &ureq::Agent, s: &Shot, pid: &str) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(s.timeout_secs);
    let mut polls: u64 = 0;
    loop {
        if let Ok(r) = a.get(&format!("{}/history/{}", s.server, pid)).call()
            && let Ok(body) = r.into_string()
            && let Ok(v) = serde_json::from_str::<Value>(&body)
            && let Some(entry) = v.get(pid)
        {
            let status = entry.get("status");
            let done = status
                .and_then(|x| x.get("completed"))
                .and_then(|x| x.as_bool())
                .unwrap_or(false);
            if done {
                let state = status
                    .and_then(|x| x.get("status_str"))
                    .and_then(|x| x.as_str())
                    .unwrap_or("");
                if state == "error" {
                    return Err(format!("生成失败：{status:?}"));
                }
                return Ok(());
            }
        }
        if Instant::now() > deadline {
            return Err(format!("生成超时（{}s）", s.timeout_secs));
        }
        polls += 1;
        if polls.is_multiple_of(15) {
            println!("  生成中… 已等 {}s", polls * 2);
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// 在输出目录 video/ 下找本次实验最新落盘的产物。
fn find_output(output: &Path, name: &str) -> Option<PathBuf> {
    let mut hits: Vec<(SystemTime, PathBuf)> = std::fs::read_dir(output.join("video"))
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.file_name()
                    .map(|f| f.to_string_lossy().starts_with(name))
                    .unwrap_or(false)
        })
        .filter_map(|p| {
            std::fs::metadata(&p)
                .ok()
                .and_then(|m| m.modified().ok())
                .map(|t| (t, p))
        })
        .collect();
    hits.sort();
    hits.pop().map(|(_, p)| p)
}

fn run_shot(s: Shot) -> Result<(), String> {
    if s.name.is_empty() || s.name.contains('/') {
        return Err("实验名不能为空且不能含 /".into());
    }
    let a = agent();
    ensure_server(&a, &s)?;

    println!(
        "提交 {}：{}x{}，{} 帧，{} 步，cfg {}，seed {}",
        s.name, s.width, s.height, s.frames, s.steps, s.cfg, s.seed
    );
    let t0 = Instant::now();
    let pid = submit(&a, &s)?;
    wait_done(&a, &s, &pid)?;
    let elapsed = t0.elapsed().as_secs();

    let src = find_output(&s.output, &s.name).ok_or_else(|| {
        format!(
            "生成完成但在 {} 下没找到 {} 的产物",
            s.output.display(),
            s.name
        )
    })?;
    let dir = s.experiments.join(&s.name);
    std::fs::create_dir_all(&dir).map_err(|e| format!("建 {} 失败：{e}", dir.display()))?;
    let ext = src
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dest = dir.join(format!("{}.{}", s.name, ext));
    std::fs::copy(&src, &dest).map_err(|e| format!("拷产物失败：{e}"))?;

    let now = chrono::Local::now().format("%F %T");
    let run_md = format!(
        "# {name}\n\n\
         - 跑于 {now}，耗时 {elapsed}s，服务 {server}，prompt_id {pid}\n\
         - seed {seed}，{width}x{height}，{frames} 帧，{steps} 步，cfg {cfg}，uni_pc/simple，shift 8，fps {fps}\n\
         - 模型 {unet} + {clip} + {vae}\n\
         - prompt：{prompt}\n\
         - negative：{negative}\n\
         - 产物：{file}\n\
         \n## 判断\n\n（补一句：哪里对、哪里崩、下次改什么）\n",
        name = s.name,
        now = now,
        elapsed = elapsed,
        server = s.server,
        pid = pid,
        seed = s.seed,
        width = s.width,
        height = s.height,
        frames = s.frames,
        steps = s.steps,
        cfg = s.cfg,
        fps = s.fps,
        unet = UNET_NAME,
        clip = CLIP_NAME,
        vae = VAE_NAME,
        prompt = s.prompt,
        negative = s.negative,
        file = dest.file_name().unwrap_or_default().to_string_lossy(),
    );
    std::fs::write(dir.join("run.md"), run_md).map_err(|e| format!("写 run.md 失败：{e}"))?;

    println!(
        "完成：{}\n落档：{}",
        dest.display(),
        dir.join("run.md").display()
    );
    Ok(())
}
