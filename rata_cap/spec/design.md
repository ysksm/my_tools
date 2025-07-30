# rata_cap 設計書

## 概要
rata_capは、Ratatuiを使用したターミナルベースのパケットキャプチャツールです。
ユーザーフレンドリーなTUIを提供し、ネットワークインターフェースの選択からパケットキャプチャ、表示までを一貫して行えます。

## アーキテクチャ

### 技術スタック
- **TUIフレームワーク**: Ratatui (ターミナルUI構築)
- **パケットキャプチャ**: pcap/pcap-sys (低レベルパケットキャプチャ)
- **非同期処理**: Tokio (並行処理とイベントループ)
- **ターミナル制御**: Crossterm (クロスプラットフォーム対応)

### モジュール構成

```
src/
├── main.rs              # アプリケーションエントリポイント
├── app.rs               # アプリケーション状態管理
├── ui/                  # UI関連モジュール
│   ├── mod.rs
│   ├── interface_selector.rs  # インターフェース選択画面
│   └── packet_viewer.rs       # パケット表示画面
└── network/             # ネットワーク関連モジュール
    ├── mod.rs
    ├── interface.rs     # ネットワークインターフェース検出
    ├── capture.rs       # パケットキャプチャ処理
    └── packet.rs        # パケットデータ構造

```

## 画面設計

### 1. インターフェース選択画面
- 利用可能なネットワークインターフェースの一覧表示
- 上下キーでインターフェース選択
- Enterキーで選択確定
- qキーで終了

### 2. パケットキャプチャ画面
- リアルタイムでパケット情報を表示
- カラム: 時刻、送信元、宛先、プロトコル、サイズ、情報
- スクロール機能（上下キー）
- spaceキーで一時停止/再開
- qキーでインターフェース選択画面に戻る
- ESCキーで終了

## データ構造

### Packet構造体
```rust
pub struct Packet {
    pub timestamp: SystemTime,
    pub source: String,
    pub destination: String,
    pub protocol: Protocol,
    pub length: usize,
    pub info: String,
    pub raw_data: Vec<u8>,
}

pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Other(String),
}
```

### AppState構造体
```rust
pub enum AppState {
    InterfaceSelection,
    Capturing(CaptureState),
    Exiting,
}

pub struct CaptureState {
    pub interface: String,
    pub packets: Vec<Packet>,
    pub is_paused: bool,
    pub selected_index: usize,
}
```

## 非同期処理設計
- Tokioランタイムでメインループを実行
- パケットキャプチャは別スレッドで実行
- mpscチャネルでUIスレッドとキャプチャスレッド間の通信
- UIの更新は60FPSで実行

## エラーハンドリング
- 権限不足時のエラーメッセージ表示
- ネットワークインターフェースが見つからない場合の処理
- パケットキャプチャエラーの適切な処理

## セキュリティ考慮事項
- root権限が必要な操作の明示
- キャプチャデータはメモリ内のみで保持（ファイル保存は実装しない）