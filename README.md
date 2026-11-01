# Game Disconnect Detector
![Image](https://cdn.ismailcetin.dev/rust_dcdetector.jpg)
##### This is a Rust application that detects when your connection drops—especially in games while you are away from the computer—and triggers an alarm.
##### It can send a signal to a remote network by making an HTTP request to any API endpoint you configure when you get disconnected from the game. However, you don't have to use this feature!
##### :warning: You must install Tesseract OCR on your computer for the program to work. Continue reading this document for installation instructions.
##### &nbsp;
##### &nbsp;
## :sparkles: Getting Started
#### This software is coded entirely in Rust for the Windows platform and uses the Tesseract OCR engine to read text on the screen. Therefore, the first thing you need to do is install it on your computer. Don't worry, installing it is very simple.
- **1**: Type "cmd" in the Start search box and run it as administrator.
- **2**: Then, run the command below;
   ```sh
   winget install -e --id UB-Mannheim.TesseractOCR
   ```
- **3**: Complete the installation by answering "yes" to the confirmation prompts.
- **4**: Restart your computer.
## :hammer_and_wrench: 1A) I want to compile the program myself
- **1**: Create a new project using [Rust Cargo](https://doc.rust-lang.org/cargo).
- **2**: Replace the `main.rs` file with the one provided in this repository.
- **3**: Compile it by running the following command;
   ```sh
   cargo build --release
   ```
## :rocket: 1B) I don't want to deal with compiling
- **1**: Simply [Rust Cargo](https://github.com/icetinturkey/game-dc-detector/releases) to download the `dcdetector.exe` file.
##### &nbsp;
##### &nbsp;
## :coffee: Bana bir kahve ısmarla
Bu paylaşım topluluğunun büyümesini sağlamak için depomu favorilere ekleyebilir veya bana bağışta bulunabilirsiniz.
* Litecoin
  ```sh
  LMuSGiA477Pw3DFaEixMoAaL8XBWJQmaAW
  ```
