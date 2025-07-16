# Matrix Stream

TCP経由で8x8ドットマトリクスLEDに動画を表示するシステムです。

Raspberry Piとシフトレジスタ(74HC595)を使って、PCから送信された画像・動画データをリアルタイムでドットマトリクスLED(OSL641505)に表示できます。

# 特徴

- TCP通信によるネットワーク経由でのリアルタイム表示
- mp4、avi、movなどの動画ファイルを8x8に変換して表示
- 静止画像も表示可能
- ダイナミック点灯制御で約60FPS
- Rustで実装

# 必要なハードウェア

## Raspberry Pi側（Receiver）
- Raspberry Pi 4 Model B
- 8x8 青色ドットマトリクスLED（OSL641505-BB）
- 8ビットシフトレジスタ（74HC595） × 2個
- ブレッドボード、ジャンパワイヤー、抵抗（350Ω）

## PC側（Sender）
- Rust開発環境
- ffmpeg（動画処理用）

# セットアップ

## 1. リポジトリをクローン

```bash
git clone https://github.com/totosuki/matrix-stream.git
cd matrix-stream
```

## 2. 依存関係をインストール

### Raspberry Pi側
```bash
cargo build --features raspi
```

### PC側（Sender）
```bash
# ffmpegをインストール
sudo apt install ffmpeg  # Ubuntu/Debian
brew install ffmpeg      # macOS

cargo build
```

# 使用方法

## 1. Receiver（Raspberry Pi）を起動

```bash
cargo run --bin receiver --features raspi
```

## 2. Sender（PC）から画像/動画を送信

```bash
cargo run --bin sender
```

プロンプトに従ってファイルパスを入力してください。

# データ形式

- 入力: 任意サイズの画像・動画
- 処理: 8x8ピクセルにリサイズ → グレースケール変換 → 2値化
- 通信: 64bit（8x8）の白黒データをTCP送信
- 表示: ダイナミック点灯制御でLEDマトリクス表示

# トラブルシューティング

## Permission denied エラー
```bash
sudo usermod -a -G gpio $USER
# 再ログイン必要
```

## 接続エラー
- Raspberry PiのIPアドレスを確認
- `receiver.rs`の`bind_addr`を`"0.0.0.0"`に変更してネットワーク接続を許可

## LEDが点灯しない
- 配線を再確認
- 抵抗値をチェック（350Ω）
- GPIOピン番号を確認

# プロジェクト構造

```
matrix-stream/
├── src/
│   ├── bin/
│   │   ├── receiver.rs
│   │   └── sender.rs
│   ├── drivers/
│   │   ├── hc595.rs
│   │   ├── mod.rs
│   │   └── osl641505.rs
│   ├── display_controller.rs
│   ├── lib.rs
│   └── protocol.rs
├── .gitignore
├── Cargo.lock
└── Cargo.toml
```

## ライセンス
このプロジェクトはMITライセンスの下で公開されています。
