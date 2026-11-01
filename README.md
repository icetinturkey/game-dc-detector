# Game Disconnect Detector
![Image](https://cdn.ismailcetin.dev/rust_dcdetector.jpg)
##### This is a Rust application that detects when your connection drops—especially in games while you are away from the computer—and triggers an alarm.
##### It can send a signal to a remote network by making an HTTP request to any API endpoint you configure when you get disconnected from the game. However, you don't have to use this feature!
##### :warning: You must install Tesseract OCR on your computer for the program to work. Continue reading this document for installation instructions.
##### &nbsp;
##### &nbsp;
## :sparkles: 1) Getting Started
#### This software is coded entirely in Rust for the Windows platform and uses the Tesseract OCR engine to read text on the screen. Therefore, the first thing you need to do is install it on your computer. Don't worry, installing it is very simple.
- **1**: Type "cmd" in the Start search box and run it as administrator.
- **2**: Then, run the command below;
   ```sh
   winget install -e --id UB-Mannheim.TesseractOCR
   ```
- **3**: Complete the installation by answering "yes" to the confirmation prompts.
- **4**: Restart your computer.
## :hammer_and_wrench: 2A) I want to compile the program myself
- **1**: Create a new project using [Rust Cargo](https://doc.rust-lang.org/cargo);
   ```sh
   cargo new dcdetector
   ```
- **2**: Replace the `main.rs` file with the one provided in this repository.
- **3**: Compile it by running the following command;
   ```sh
   cargo build --release
   ```
## :rocket: 2B) I don't want to deal with compiling
- **1**: Simply [click here](https://github.com/icetinturkey/game-dc-detector/releases) to download the `dcdetector.exe` file.
## :gear: 3) How to Setup
Setting up the program can be a bit confusing. However, once you handle this, Dc Detector will save your data to a txt file. For every subsequent use, all you will need to do is start it.
The first thing you need to do is select which monitor your game is currently running on. If you are already using a single monitor, you can skip this step. This part is very important: `Your game must run in windowed mode`.
The next thing you need to set is the workspace coordinates. You can simply set this to 0,0 and 1920,1080 for a Full HD monitor to capture a photo of the entire screen. However, this will put a heavy load on your computer and make it very difficult for the OCR engine to detect the text. Therefore, it is strongly recommended that you shrink the area to be photographed as much as possible through trial and error.
The next stage is which word to search for in the photo. This word should be simple. Like: "disconnect", "terminate", or "lost". When it catches this word in the photo, an alarm will sound on your computer.
Finally, there is the Link section, but I will detail this in a separate section later. If you do not have sufficient programming knowledge, you can leave this box empty. The program will still run smoothly.
## :arrow_forward: 4) How to Run
When you run the program, Dc Detector will automatically load your previous settings. All you need to do is click the `Start` button.
## :satellite: 5) Sending a notification to a remote computer (optional)
#### If you do not have sufficient programming knowledge or an expert who can help you, skip this section.
You might want to send a notification of your disconnection not just as an alarm, but to another computer. Or you might want a notification to be sent to your Telegram. How you code your existing backend system is up to you. However, Dc Detector can notify your system of your disconnection by sending a GET request to it.
I use SlimPHP for this task. When a request comes in, I save it to a MySQL database. Then, I check whether there are any changes in the table using JS AJAX. Of course, these are not things simple enough to share entirely here. However, I am still sharing my SlimPHP endpoint that saves incoming notifications to the database.
##### &nbsp;
##### &nbsp;
## :coffee: Buy me a coffee
To help this community grow, you can add my repository to your favorites or make a donation to me.
* Litecoin
  ```sh
  LMuSGiA477Pw3DFaEixMoAaL8XBWJQmaAW
  ```
