use std::{
    io::{stdin, stdout, Result, Write, BufWriter},
    net::TcpStream,
    thread,
    time::{Duration, Instant},
    path::Path,
    process::Command,
    fs,
};

use image::{
    imageops,
    DynamicImage,
};

fn main() -> Result<()> {
    println!("[sender] start");

    let mut input_path = String::new();
    print!("File path (image or video): ");
    stdout().flush()?;
    stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim().to_string();

    // ファイルの拡張子をチェックして動画か画像かを判定
    if is_video_file(&input_path) {
        println!("[sender] Processing video file...");
        
        // フレームレート設定
        let mut fps_input = String::new();
        print!("Frame rate (fps, default: 30): ");
        stdout().flush()?;
        stdin().read_line(&mut fps_input)?;
        let fps: f64 = fps_input.trim().parse().unwrap_or(30.0);
        
        process_video(&input_path, fps)?;
    } else {
        println!("[sender] Processing image file...");
        process_single_image(&input_path)?;
    }

    println!("[sender] stop");
    Ok(())
}

fn is_video_file(path: &str) -> bool {
    let video_extensions = ["mp4", "avi", "mov", "mkv", "webm", "flv", "m4v"];
    if let Some(extension) = Path::new(path).extension() {
        if let Some(ext_str) = extension.to_str() {
            return video_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

fn process_single_image(path: &str) -> Result<()> {
    println!("[sender] Loading image: {}", path);
    let binary_data = convert_image_to_binary(path);
    
    let mut stream = connect_server("localhost", 8080)?;
    let mut writer = BufWriter::new(&mut stream);
    writer.write_all(binary_data.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    println!("[sender] Image sent successfully");
    Ok(())
}

fn process_video(path: &str, fps: f64) -> Result<()> {
    let temp_dir = "temp_frames";
    
    // 一時ディレクトリを作成
    if Path::new(temp_dir).exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    fs::create_dir(temp_dir)?;
    
    println!("[sender] Extracting frames from video...");
    
    // ffmpegで動画をフレーム画像に分割
    let output = Command::new("ffmpeg").args(&[
            "-i",
            path,
            "-vf",
            "fps=30",
            "-y",
            &format!("{}/frame_%04d.png", temp_dir)
        ]).output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("[sender] ffmpeg error: {}", String::from_utf8_lossy(&output.stderr));
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "ffmpeg failed to extract frames"
                ));
            }
        },
        Err(e) => {
            eprintln!("[sender] Failed to run ffmpeg. Make sure ffmpeg is installed.");
            return Err(e);
        }
    }

    // 抽出されたフレーム数を取得
    let frame_files = get_frame_files(temp_dir)?;
    println!("[sender] Extracted {} frames", frame_files.len());

    if frame_files.is_empty() {
        eprintln!("[sender] No frames were extracted from the video");
        cleanup_temp_dir(temp_dir)?;
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No frames extracted"
        ));
    }

    println!("[sender] Connecting to server...");
    let mut stream = connect_server("localhost", 8080)?;
    let mut writer = BufWriter::new(&mut stream);

    let frame_duration = Duration::from_secs_f64(1.0 / fps);
    
    println!("[sender] Starting video transmission at {:.1} fps...", fps);

    // 各フレームを順次処理して送信
    for (i, frame_path) in frame_files.iter().enumerate() {
        let start_time = Instant::now();

        // 既存の画像処理ロジックを使用
        let binary_data = convert_image_to_binary(frame_path);

        // データ送信
        writer.write_all(binary_data.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;

        if (i + 1) % 30 == 0 {
            println!("[sender] Sent {} frames", i + 1);
        }

        // フレームレート制御
        let elapsed = start_time.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    println!("[sender] Video transmission completed. Total frames: {}", frame_files.len());

    // 一時ファイルをクリーンアップ
    cleanup_temp_dir(temp_dir)?;
    
    Ok(())
}

fn convert_image_to_binary(path: &str) -> String {
    // 既存の画像処理ロジックをそのまま使用
    let data = load_image(path.to_string());
    let data = data.to_luma8();
    let data = imageops::resize(&data, 8, 8, imageops::Triangle);
    let data: Vec<u8> = data.pixels().map(|p| p[0]).collect();
    data.iter()
        .map(|&pixel| if pixel > 128 { '1' } else { '0' })
        .collect::<String>()
}

fn get_frame_files(temp_dir: &str) -> Result<Vec<String>> {
    let mut frame_files = Vec::new();
    
    let entries = fs::read_dir(temp_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    if filename_str.starts_with("frame_") && filename_str.ends_with(".png") {
                        frame_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    
    // ファイル名でソート（frame_0001.png, frame_0002.png, ...の順番）
    frame_files.sort();
    
    Ok(frame_files)
}

fn cleanup_temp_dir(temp_dir: &str) -> Result<()> {
    println!("[sender] Cleaning up temporary files...");
    if Path::new(temp_dir).exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    Ok(())
}

fn connect_server(addr: &str, port: u64) -> Result<TcpStream> {
    println!("[sender] Attempting to connect to {}:{}...", addr, port);
    
    // 接続リトライ機能
    for attempt in 1..=5 {
        match TcpStream::connect(format!("{}:{}", addr, port)) {
            Ok(stream) => {
                println!("[sender] Connected to {}:{}", addr, port);
                return Ok(stream);
            },
            Err(e) => {
                eprintln!("[sender] Connection attempt {} failed: {}", attempt, e);
                if attempt < 5 {
                    println!("[sender] Retrying in 2 seconds...");
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "Failed to connect after 5 attempts"
    ))
}

fn load_image(path: String) -> DynamicImage {
    match image::open(&path) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("[sender] Failed to open/decode image: {}", e);
            panic!("Failed to open/decode image: {}", e);
        }
    }
}