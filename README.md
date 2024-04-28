## Make 2D Bullet Hells simply by editing configuration files
### Installation Steps
1) Have **Git** and **Rust** installed (if so, skip this step):
    - **On Windows:** Open Powershell with Administrator priviledges and run the following command
    ```bash
    winget install Git.Git Rustlang.Rustup
    ```
    - **On Mac:** Open a terminal, you probably also need to run the following command as administrator
    ```bash
    brew install git rust    
    ```
    - **On Ubuntu / Debian:** 
    ```bash
    sudo apt install git rustc cargo
    ```
    > If you cannot get the latest Rust version through you package manager (`winget` / `brew` / `apt`). Consider installing it from the [Rust official website](https://www.rust-lang.org/tools/install)
2) Run the following commands to download and run the project:
    ```bash
    git clone https://github.com/LuckyToaster/2D_game
    cd 2D_game
    cargo run
    ```
    > This might take a few minutes, as cargo will have to download and compile the game engine library
