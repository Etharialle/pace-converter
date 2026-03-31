# 🏃 Pace Converter TUI

A fast, lightweight Terminal User Interface (TUI) built in **Rust** for calculating running paces. Effortlessly switch between metric and imperial units with real-time updates and "microwave-style" time entry.

---

## ✨ Features

* **Real-time Calculation:** See your pace in both `min/km` and `min/mile` instantly as you type.
* **Intuitive Time Entry:** Uses a "digit-shift" input (HH:MM:SS) so you never have to type colons manually.
* **Unit Toggling:** Quickly switch input context between Kilometers and Miles.
* **Responsive UI:** Built with `ratatui` for a clean, flicker-free terminal experience.

---

## ⌨️ Controls

| Key | Action |
| :--- | :--- |
| **`Tab`** | Toggle focus between **Distance** and **Time** fields |
| **`U`** | Toggle distance units (**Kilometers** ↔ **Miles**) |
| **`Backspace`** | Delete last digit/character |
| **`Esc`** | Exit the application |
| **`0-9` / `.`** | Input data |

---

## 🚀 Getting Started

### Prerequisites
* **Rust** (latest stable version)
* A terminal that supports **ANSI** escape sequences (WSL, Linux, macOS, or modern Windows Terminal)

### Installation
1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/etharialle/pace-converter.git](https://github.com/etharialle/pace-converter.git)
    cd pace-converter
    ```

2.  **Run the application:**
    ```bash
    cargo run
    ```

3.  **Build for release (Optional):**
    ```bash
    cargo build --release
    ./target/release/pace-converter
    ```

---

## 🛠️ Technical Details

* **Language:** Rust
* **TUI Library:** [ratatui](https://github.com/ratatui-org/ratatui) (v0.26)
* **Terminal Handling:** [crossterm](https://github.com/crossterm-rs/crossterm)
* **Architecture:** The project separates concerns into:
    * **Terminal Lifecycle:** Raw mode initialization and automatic cleanup.
    * **Input Management:** Selective character filtering for distance and time.
    * **Math Logic:** Precise floating-point conversions between metric and imperial pace units.

---

## 📝 License

This project is open source and available under the [MIT License](LICENSE).