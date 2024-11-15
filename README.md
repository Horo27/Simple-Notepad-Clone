# Notepad Clone App

This is a simple **Notepad Clone App** built using **Tauri** with a Rust backend and a vanilla frontend (HTML, CSS, JavaScript). The app also uses **Deno** for some additional functionality. It provides basic functionality to manage text files and offers a lightweight, desktop-friendly alternative for quick note-taking.

## Features

- **Open**: Load text files into the editor.
- **Save**: Save your notes as text files to your local system.
- **Clear**: Quickly clear the editor for new notes.

## Technology Stack

- **Backend**: Rust (via Tauri framework)
- **Frontend**: HTML, CSS, and JavaScript
- **Additional Functionality**: Deno (for various tasks)

## Getting Started

To run the app locally, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Horo27/Simple-Notepad-Clone.git
   cd notepad-clone

2. **Install dependencies**:<br>
     Make sure you have Rust installed. You can follow the installation guide [here](https://doc.rust-lang.org/beta/book/ch01-01-installation.html).<br>
     Make sure you have Tauri prerequisites installed. You can follow the installation guide [here](https://v2.tauri.app/start/).<br>
     Make sure you have Deno prerequisites installed. You can follow the installation guide [here](https://docs.deno.com/runtime/getting_started/installation/).<br>
     
3. **Run the app**: Once dependencies are installed, run the following command to start the app in development mode:
   ```bash
   deno task tauri dev

## Why Tauri?
This app leverages Tauri to create a lightweight desktop application with minimal resource usage. Tauri's Rust-based backend ensures high performance and security, while its flexibility allows for seamless integration with a vanilla frontend.

Feel free to contribute or provide feedback!
