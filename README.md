sudo apt update
sudo apt install -y libssl-dev pkg-config curl build-essential gcc libudev-dev
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv

RUSTUP_TOOLCHAIN=stable cargo install espup --locked
espup install --targets esp32
source /home/vscode/export-esp.sh (cat で.bashrcに追加)
rustup default esp (ビルドのデフォルトをespに設定)

cargo install espflash --locked
cargo install ldproxy
cargo install cargo-generate
cargo install cargo-espflash --locked



自分で実行 名前はホストフォルダ名と同じにするとビルドが速い20倍くらい 対話型 devkitC は 多分esp32 
cargo generate esp-rs/esp-idf-template cargo
フォルダの中身を親フォルダに移す

cargo build -vv
cargo run

espflash monitor

### rust-analyzer エラー
ctrl + shift + P Reload Workspace 

# time::sleep 使わない
FreeRTOS::delay_ms(100); を使う
内部変換されてるっぽいから、別にどうでもいいかも

## スタック不足のエラーがあるらしい

# ディレクトリ構造が下手
いつかworkspace