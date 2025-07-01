# LogAnalyser
A tool that uses rust to detect bruteforce for archlinux logs using journalctl

=======================================================================

Clone the repository

```bash
git clone https://github.com/yourusername/LogAnalyser.git
cd LogAnalyser
```
Build the program (release mode)
```bash
cargo build --release
```
Run the tool
```bash
./target/release/LogAnalyser
```

To install it globally
```bash
cargo install --path .
```
Now you can run it anywhere using
```bash
LogAnalyser
```
