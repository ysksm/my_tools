# rata_cap

A terminal-based packet capture tool built with Rust and Ratatui.

## Features

- Interactive TUI for network interface selection
- Real-time packet capture and display
- Support for TCP, UDP, ICMP, and ARP protocols
- Scrollable packet list with pause/resume functionality
- Cross-platform support (requires root/admin privileges)

## Installation

### Prerequisites

- Rust toolchain (1.70+)
- libpcap (Linux/macOS) or WinPcap/Npcap (Windows)
- Root/Administrator privileges for packet capture

### Build from source

```bash
git clone https://github.com/yourusername/rata_cap.git
cd rata_cap
cargo build --release
```

## Usage

Run with root/administrator privileges:

```bash
sudo ./target/release/rata_cap
```

### Controls

#### Interface Selection Screen
- `↑/↓`: Navigate through interfaces
- `Enter`: Select interface and start capture
- `q`: Quit application

#### Packet Capture Screen
- `↑/↓`: Scroll through packets
- `Space`: Pause/Resume capture
- `q`: Return to interface selection
- `Esc`: Exit application

## Development

### Running in development mode

```bash
sudo cargo run
```

### Running tests

```bash
cargo test
```

### Code formatting and linting

```bash
cargo fmt
cargo clippy
```

## License

This project is licensed under the MIT License.