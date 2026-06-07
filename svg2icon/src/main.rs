use clap::{Parser, ValueEnum};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::DynamicImage;
use roxmltree::Document;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use tiny_skia::{Pixmap, Transform};

/// SVG 转高清图标 — 支持将标准 SVG 资产转换为多尺寸 / 多格式的交付文件（PNG / JPEG / ICO / ICNS）。
///
/// 默认会先执行 SVG 质量检查；若存在阻断性错误则停止导出。
#[derive(Parser, Debug)]
#[command(name = "svg2icon", version, about, long_about = None)]
struct Args {
    /// 输入的 SVG 文件路径
    #[arg(short = 's', long)]
    svg: PathBuf,

    /// 输出尺寸列表（像素），逗号分隔，例如：512,256,128,64（默认 512）
    #[arg(long, value_delimiter = ',', default_values_t = vec![512])]
    sizes: Vec<u32>,

    /// 输出格式，支持逗号分隔多值：png / jpg / ico / icns（默认 png）
    #[arg(short = 'f', long, value_delimiter = ',', default_value = "png")]
    format: Vec<String>,

    /// 导出变体，支持逗号分隔多值：primary / mono / reversed（默认 primary）
    #[arg(long, value_delimiter = ',', value_enum, default_value = "primary")]
    variants: Vec<Variant>,

    /// 超采样倍率（母版尺寸 = 最大目标尺寸 × 倍率），默认 2
    #[arg(short = 'x', long, default_value_t = 2)]
    supersample: u32,

    /// JPEG 质量 (1-100)，仅当 --format 包含 jpg 时有效
    #[arg(short = 'q', long, default_value_t = 95)]
    quality: u8,

    /// 输出目录，默认为当前目录
    #[arg(short = 'o', long, default_value = ".")]
    out_dir: PathBuf,

    /// 背景色，hex 格式 #RRGGBB 或 #RRGGBBAA。不指定则透明背景（JPEG 默认白色）
    #[arg(long)]
    bg: Option<String>,

    /// 仅执行质量检查，不导出文件
    #[arg(long, default_value_t = false)]
    check_only: bool,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Variant {
    Primary,
    Mono,
    Reversed,
}

impl Variant {
    fn as_str(&self) -> &'static str {
        match self {
            Variant::Primary => "primary",
            Variant::Mono => "mono",
            Variant::Reversed => "reversed",
        }
    }
}

#[derive(Default, Debug)]
struct QualityReport {
    passes: Vec<String>,
    warnings: Vec<String>,
    errors: Vec<String>,
}

impl QualityReport {
    fn pass<S: Into<String>>(&mut self, message: S) {
        self.passes.push(message.into());
    }

    fn warn<S: Into<String>>(&mut self, message: S) {
        self.warnings.push(message.into());
    }

    fn error<S: Into<String>>(&mut self, message: S) {
        self.errors.push(message.into());
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn print(&self) {
        println!("🧪 SVG 质量检查报告");
        println!("----------------------------------------");

        for msg in &self.passes {
            println!("✅ {msg}");
        }
        for msg in &self.warnings {
            println!("⚠️  {msg}");
        }
        for msg in &self.errors {
            println!("❌ {msg}");
        }

        println!(
            "----------------------------------------\n检查结果：{} 项通过，{} 项警告，{} 项错误\n",
            self.passes.len(),
            self.warnings.len(),
            self.errors.len()
        );
    }
}

// ---------------------------------------------------------------------------
// 背景色解析
// ---------------------------------------------------------------------------

/// 解析背景色字符串，返回 `[R, G, B, A]`。
/// `None` 表示透明。
fn parse_bg_color(s: &str) -> Option<[u8; 4]> {
    let s = s.trim();
    if s.eq_ignore_ascii_case("transparent") || s.eq_ignore_ascii_case("none") {
        return None;
    }
    let hex = s.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// 质量检查
// ---------------------------------------------------------------------------

fn parse_length_value(value: &str) -> Option<f32> {
    let trimmed = value.trim();
    let numeric: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    if numeric.is_empty() {
        None
    } else {
        numeric.parse::<f32>().ok()
    }
}

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

fn run_quality_checks(svg_text: &str) -> Result<QualityReport, Box<dyn std::error::Error>> {
    let doc = Document::parse(svg_text)?;
    let mut report = QualityReport::default();

    let root = doc.root_element();
    if root.tag_name().name() != "svg" {
        report.error("根节点不是 <svg>。");
        return Ok(report);
    }

    match root.attribute("viewBox") {
        Some(view_box) => {
            let values: Vec<f32> = view_box
                .replace(',', " ")
                .split_whitespace()
                .filter_map(|item| item.parse::<f32>().ok())
                .collect();
            if values.len() == 4
                && approx_eq(values[0], 0.0)
                && approx_eq(values[1], 0.0)
                && approx_eq(values[2], 512.0)
                && approx_eq(values[3], 512.0)
            {
                report.pass("viewBox 符合 0 0 512 512 标准画板。\n");
            } else {
                report.error(format!(
                    "viewBox 当前为 \"{view_box}\"，不符合 skill 要求的 0 0 512 512。"
                ));
            }
        }
        None => report.error("缺少 viewBox，skill 要求显式声明 viewBox=\"0 0 512 512\"。"),
    }

    let width = root.attribute("width").and_then(parse_length_value);
    let height = root.attribute("height").and_then(parse_length_value);
    match (width, height) {
        (Some(w), Some(h)) if approx_eq(w, 512.0) && approx_eq(h, 512.0) => {
            report.pass("width / height 与 512×512 源画板一致。")
        }
        (Some(w), Some(h)) => report.warn(format!(
            "width / height 为 {}×{}；推荐与源画板保持一致（512×512）。",
            w, h
        )),
        _ => report.warn("未同时声明 width / height；虽不阻断导出，但建议显式写为 512×512。"),
    }

    match root.attribute("shape-rendering") {
        Some("geometricPrecision") => report.pass("shape-rendering 已设置为 geometricPrecision。"),
        Some(value) => report.warn(format!(
            "shape-rendering 当前为 \"{value}\"；建议设为 geometricPrecision。"
        )),
        None => report.warn("未设置 shape-rendering；建议添加 geometricPrecision 以提升几何形状稳定性。"),
    }

    let mut has_text = false;

    for node in doc.descendants().filter(|n| n.is_element()) {
        let tag = node.tag_name().name();

        if tag == "image" {
            report.error("发现 <image> 元素。skill 要求 SVG 自包含矢量内容，不允许位图嵌入。");
        }

        if tag == "text" {
            has_text = true;
        }

        for attr in node.attributes() {
            if attr.name() == "href" {
                let value = attr.value().trim();
                if value.starts_with("data:image/") {
                    report.error(format!(
                        "发现 data URI 位图嵌入：{value}。请移除所有嵌入式位图。"
                    ));
                } else if !value.is_empty() && !value.starts_with('#') {
                    report.error(format!(
                        "发现外部资源引用 \"{value}\"。skill 要求 SVG 不依赖外部资源。"
                    ));
                }
            }
        }
    }

    if has_text {
        report.warn("发现 <text> 元素；若依赖字体渲染，跨平台导出可能不稳定，建议转为 path。");
    } else {
        report.pass("未发现 <text> 元素，字体兼容风险较低。")
    }

    if !report.has_errors() {
        report.pass("SVG 通过阻断性检查，可继续导出。")
    }

    Ok(report)
}

// ---------------------------------------------------------------------------
// 图像处理辅助函数
// ---------------------------------------------------------------------------

/// 将 `img` 叠在纯色背景上，返回合成后的图像。
fn apply_bg(img: &DynamicImage, bg: [u8; 4], size: u32) -> DynamicImage {
    let mut bg_img = image::RgbaImage::from_pixel(size, size, image::Rgba(bg));
    image::imageops::overlay(&mut bg_img, &img.to_rgba8(), 0, 0);
    DynamicImage::ImageRgba8(bg_img)
}

/// 从母版缩放到目标尺寸（若尺寸相同则直接克隆）。
fn prepare_image(master_img: &DynamicImage, size: u32) -> DynamicImage {
    if size == master_img.width() && size == master_img.height() {
        master_img.clone()
    } else {
        master_img.resize_exact(size, size, FilterType::Lanczos3)
    }
}

/// 将已有图像转换为正式的品牌变体：primary / mono / reversed。
fn apply_variant(img: &DynamicImage, variant: &Variant) -> DynamicImage {
    match variant {
        Variant::Primary => img.clone(),
        Variant::Mono => remap_foreground_color(img, [0, 0, 0]),
        Variant::Reversed => remap_foreground_color(img, [255, 255, 255]),
    }
}

/// 将非透明前景统一映射为指定颜色，并保留 alpha。
fn remap_foreground_color(img: &DynamicImage, color: [u8; 3]) -> DynamicImage {
    let mut rgba = img.to_rgba8();
    for pixel in rgba.pixels_mut() {
        if pixel[3] > 0 {
            pixel[0] = color[0];
            pixel[1] = color[1];
            pixel[2] = color[2];
        }
    }
    DynamicImage::ImageRgba8(rgba)
}

fn normalize_formats(formats: &[String]) -> Vec<String> {
    formats
        .iter()
        .map(|fmt| fmt.trim().to_ascii_lowercase())
        .filter(|fmt| !fmt.is_empty())
        .collect()
}

fn output_stem(svg_path: &Path) -> String {
    svg_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("icon")
        .to_string()
}

fn bundle_filename(base: &str, variant: &Variant, fmt: &str) -> String {
    format!("{}-{}.{}", base, variant.as_str(), file_ext(fmt))
}

fn raster_filename(base: &str, variant: &Variant, size: u32, fmt: &str) -> String {
    format!(
        "{}-{}-{}x{}.{}",
        base,
        variant.as_str(),
        size,
        size,
        file_ext(fmt)
    )
}

// ---------------------------------------------------------------------------
// 保存单个文件
// ---------------------------------------------------------------------------

fn save_image(
    img: &DynamicImage,
    path: &PathBuf,
    fmt: &str,
    quality: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    match fmt {
        "jpg" | "jpeg" => {
            let file_out = fs::File::create(path)?;
            let mut buf_writer = BufWriter::new(file_out);
            let mut encoder = JpegEncoder::new_with_quality(&mut buf_writer, quality);
            let rgb = img.to_rgb8();
            encoder.encode(
                rgb.as_raw(),
                img.width(),
                img.height(),
                image::ColorType::Rgb8.into(),
            )?;
        }
        _ => {
            img.save(path)?;
        }
    }
    Ok(())
}

/// 为给定 fmt 决定输出扩展名。
fn file_ext(fmt: &str) -> &str {
    match fmt {
        "jpg" | "jpeg" => "jpg",
        "ico" => "ico",
        "icns" => "icns",
        _ => "png",
    }
}

// ---------------------------------------------------------------------------
// ICO / ICNS 打包写入
// ---------------------------------------------------------------------------

fn write_ico(
    master_img: &DynamicImage,
    args: &Args,
    bg_color: Option<[u8; 4]>,
    out_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &args.sizes {
        let mut image = prepare_image(master_img, size);
        if let Some(bg) = bg_color {
            image = apply_bg(&image, bg, size);
        }
        let rgba = image.to_rgba8().into_raw();
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba);
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }

    let file_out = fs::File::create(out_path)?;
    let mut writer = BufWriter::new(file_out);
    icon_dir.write(&mut writer)?;
    writer.flush()?;
    println!("✅ 已生成 {}", out_path.display());
    Ok(())
}

fn write_icns(
    master_img: &DynamicImage,
    args: &Args,
    bg_color: Option<[u8; 4]>,
    out_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut family = icns::IconFamily::new();

    for &size in &args.sizes {
        let mut image = prepare_image(master_img, size);
        if let Some(bg) = bg_color {
            image = apply_bg(&image, bg, size);
        }
        let rgba = image.to_rgba8().into_raw();
        let icon_type = icns::IconType::from_pixel_size(size, size)
            .ok_or_else(|| format!("不支持的 ICNS 尺寸: {}", size))?;
        let icns_image = icns::Image::from_data(icns::PixelFormat::RGBA, size, size, rgba)?;
        family.add_icon_with_type(&icns_image, icon_type)?;
    }

    let file_out = fs::File::create(out_path)?;
    let mut writer = BufWriter::new(file_out);
    family.write(&mut writer)?;
    writer.flush()?;
    println!("✅ 已生成 {}", out_path.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// 主流程
// ---------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.sizes.is_empty() {
        return Err("至少需要指定一个输出尺寸。".into());
    }
    if args.sizes.iter().any(|size| *size == 0) {
        return Err("输出尺寸不能为 0。".into());
    }
    if args.supersample == 0 {
        return Err("超采样倍率不能为 0。".into());
    }

    fs::create_dir_all(&args.out_dir)?;

    let formats = normalize_formats(&args.format);
    if formats.is_empty() {
        return Err("至少需要指定一个输出格式。".into());
    }

    // 解析背景色
    let bg_color = match args.bg.as_deref() {
        Some(raw)
            if raw.eq_ignore_ascii_case("transparent") || raw.eq_ignore_ascii_case("none") =>
        {
            None
        }
        Some(raw) => Some(
            parse_bg_color(raw)
                .ok_or_else(|| format!("无法解析背景色: {raw}，请使用 #RRGGBB / #RRGGBBAA / transparent"))?,
        ),
        None => None,
    };

    // 读取 SVG 文本并先执行质量检查
    let svg_data = fs::read(&args.svg)?;
    let svg_text = String::from_utf8_lossy(&svg_data);
    let report = run_quality_checks(&svg_text)?;
    report.print();

    if report.has_errors() {
        return Err("SVG 未通过质量检查，导出已终止。".into());
    }

    if args.check_only {
        println!("ℹ️  已完成质量检查（check-only 模式），未执行导出。");
        return Ok(());
    }

    // 解析 SVG
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &opt)?;

    // 计算母版尺寸（超采样）
    let max_size = args.sizes.iter().max().copied().unwrap_or(512);
    let master_w = max_size * args.supersample;

    // 渲染母版（透明背景）
    let mut pixmap = Pixmap::new(master_w, master_w).ok_or("无法创建 Pixmap")?;
    let scale = master_w as f32 / tree.size().width();
    let transform = Transform::from_scale(scale, scale);
    {
        let mut pixmap_mut = pixmap.as_mut();
        resvg::render(&tree, transform, &mut pixmap_mut);
    }

    let master_img = DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(master_w, master_w, pixmap.take()).unwrap(),
    );

    let base_name = output_stem(&args.svg);

    for variant in &args.variants {
        if *variant == Variant::Reversed {
            if let Some([r, g, b, a]) = bg_color {
                if r == 255 && g == 255 && b == 255 && a == 255 {
                    eprintln!("⚠️  reversed 变体配合纯白背景会降低可见性，请确认这是否符合你的交付预期。");
                }
            }
        }

        let variant_master = apply_variant(&master_img, variant);

        for fmt in &formats {
            match fmt.as_str() {
                "ico" => {
                    let filename = bundle_filename(&base_name, variant, fmt);
                    let out_path = args.out_dir.join(filename);
                    write_ico(&variant_master, &args, bg_color, &out_path)?;
                }
                "icns" => {
                    let filename = bundle_filename(&base_name, variant, fmt);
                    let out_path = args.out_dir.join(filename);
                    write_icns(&variant_master, &args, bg_color, &out_path)?;
                }
                "png" | "jpg" | "jpeg" => {
                    for &size in &args.sizes {
                        let icon = prepare_image(&variant_master, size);

                        // JPEG 不支持透明；未指定 --bg 时默认白色背景
                        let effective_bg = if fmt == "jpg" || fmt == "jpeg" {
                            match bg_color {
                                Some(c) => Some(c),
                                None => {
                                    eprintln!(
                                        "⚠️  JPEG 不支持透明背景，使用白色（可用 --bg 指定其他颜色）"
                                    );
                                    Some([255, 255, 255, 255])
                                }
                            }
                        } else {
                            bg_color
                        };

                        let final_img = match effective_bg {
                            Some(bg) => apply_bg(&icon, bg, size),
                            None => icon,
                        };
                        let filename = raster_filename(&base_name, variant, size, fmt);
                        let out_path = args.out_dir.join(filename);
                        save_image(&final_img, &out_path, fmt, args.quality)?;
                        println!("✅ 已生成 {}", out_path.display());
                    }
                }
                _ => {
                    eprintln!(
                        "⚠️  不支持的格式：{}（支持：png, jpg, ico, icns）",
                        fmt
                    );
                }
            }
        }
    }

    Ok(())
}
