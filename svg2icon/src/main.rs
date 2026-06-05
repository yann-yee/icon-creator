use clap::Parser;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use std::io::{BufWriter, Write};
use tiny_skia::{Pixmap, Transform};
use std::path::PathBuf;
use std::fs;

/// SVG 转高清图标 - 支持批量生成多个指定尺寸的 PNG / JPEG / ICO / ICNS
///
/// 默认生成六种尺寸：1024, 512, 256, 128, 64, 32
/// 采用超采样技术确保缩放后锐利无比。
#[derive(Parser, Debug)]
#[command(name = "svg2icon", version = "0.2", about, long_about = None)]
struct Args {
    /// 输入的 SVG 文件路径
    #[arg(short = 's', long)]
    svg: PathBuf,

    /// 输出尺寸列表（像素），逗号分隔，例如：256,128,64
    #[arg(long, value_delimiter = ',',
          default_values_t = vec![1024, 512, 256, 128, 64, 32])]
    sizes: Vec<u32>,

    /// 输出格式：png / jpg / ico / icns
    #[arg(short = 'f', long, default_value = "png")]
    format: String,

    /// 超采样倍率（母版尺寸 = 最大目标尺寸 × 倍率），默认 2
    /// 提高到 3~4 可获得更优锐度，但会增加内存和耗时
    #[arg(short = 'x', long, default_value_t = 2)]
    supersample: u32,

    /// JPEG 质量 (1-100)，仅当 --format jpg 时有效
    #[arg(short = 'q', long, default_value_t = 95)]
    quality: u8,

    /// 输出目录，默认为当前目录
    #[arg(short = 'o', long, default_value = ".")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 读取并解析 SVG
    let svg_data = fs::read(&args.svg)?;
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &opt)?;

    // 计算母版尺寸（超采样）
    let max_size = args.sizes.iter().max().copied().unwrap_or(256);
    let master_w = max_size * args.supersample;

    // 渲染母版（透明背景）
    let mut pixmap = Pixmap::new(master_w, master_w)
        .ok_or("无法创建 Pixmap")?;
    let scale = master_w as f32 / tree.size().width();
    let transform = Transform::from_scale(scale, scale);
    {
        let mut pixmap_mut = pixmap.as_mut();
        resvg::render(&tree, transform, &mut pixmap_mut);
    }

    let master_img = DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(master_w, master_w, pixmap.take())
            .unwrap(),
    );

    if args.format == "ico" {
        write_ico(&master_img, &args)?;
        return Ok(());
    }

    if args.format == "icns" {
        write_icns(&master_img, &args)?;
        return Ok(());
    }

    // 生成各个尺寸
    for &size in &args.sizes {
        let mut icon = if args.supersample > 1 {
            master_img.resize_exact(size, size, FilterType::Lanczos3)
        } else {
            master_img.clone()
        };

        // JPEG 需要白色背景
        if args.format == "jpg" || args.format == "jpeg" {
            let mut bg = image::RgbaImage::from_pixel(
                size, size, image::Rgba([255, 255, 255, 255])
            );
            image::imageops::overlay(&mut bg, &icon.to_rgba8(), 0, 0);
            icon = DynamicImage::ImageRgba8(bg);
        }

        // 输出文件
        let ext = match args.format.as_str() {
            "jpg" | "jpeg" => "jpg",
            _ => "png",
        };
        let filename = format!("icon_{}x{}.{}", size, size, ext);
        let out_path = args.out_dir.join(filename);

        match args.format.as_str() {
            "jpg" | "jpeg" => {
                let file_out = fs::File::create(&out_path)?;
                let mut buf_writer = std::io::BufWriter::new(file_out);
                icon.write_to(&mut buf_writer, ImageFormat::Jpeg)?;
            }
            _ => {
                icon.save(&out_path)?;
            }
        }
        println!("✅ 已生成 {}", out_path.display());
    }

    Ok(())
}

fn write_ico(
    master_img: &DynamicImage,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &args.sizes {
        let image = prepare_image(master_img, size, "ico");
        let rgba = image.to_rgba8().into_raw();
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba);
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }

    let out_path = args.out_dir.join("icon.ico");
    let file_out = fs::File::create(&out_path)?;
    let mut writer = BufWriter::new(file_out);
    icon_dir.write(&mut writer)?;
    writer.flush()?;
    println!("✅ 已生成 {}", out_path.display());
    Ok(())
}

fn write_icns(
    master_img: &DynamicImage,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut family = icns::IconFamily::new();

    for &size in &args.sizes {
        let image = prepare_image(master_img, size, "icns");
        let rgba = image.to_rgba8().into_raw();
        let icon_type = icns::IconType::from_pixel_size(size, size)
            .ok_or_else(|| format!("不支持的 ICNS 尺寸: {}", size))?;
        let icns_image = icns::Image::from_data(
            icns::PixelFormat::RGBA,
            size,
            size,
            rgba,
        )?;
        family.add_icon_with_type(&icns_image, icon_type)?;
    }

    let out_path = args.out_dir.join("icon.icns");
    let file_out = fs::File::create(&out_path)?;
    let mut writer = BufWriter::new(file_out);
    family.write(&mut writer)?;
    writer.flush()?;
    println!("✅ 已生成 {}", out_path.display());
    Ok(())
}

fn prepare_image(master_img: &DynamicImage, size: u32, format: &str) -> DynamicImage {
    let mut icon = if size == master_img.width() && size == master_img.height() {
        master_img.clone()
    } else {
        master_img.resize_exact(size, size, FilterType::Lanczos3)
    };

    if format == "jpg" || format == "jpeg" {
        let mut bg = image::RgbaImage::from_pixel(
            size,
            size,
            image::Rgba([255, 255, 255, 255]),
        );
        image::imageops::overlay(&mut bg, &icon.to_rgba8(), 0, 0);
        icon = DynamicImage::ImageRgba8(bg);
    }

    icon
}