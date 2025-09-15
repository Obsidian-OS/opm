# `opm`

A powerful, lightweight package manager for **ObsidianOS**, built with Rust. `opm` leverages `pacman` for package operations and [`obsidianctl`](https://github.com/Obsidian-OS/obsidianctl) for managing system extensions, providing a seamless way to install, remove, and manage software packages as [ObsidianOS Overlays](https://github.com/Obsidian-OS/overlays).

## Features

- **Fast & Efficient**: Built with Rust for high performance and low resource usage
- **Pacman Integration**: Uses Arch Linux's pacman for robust package management
- **Extension Management**: Installs packages as ObsidianOS Extensions for clean system isolation
- **Advanced Querying**: Search, list, and inspect packages with detailed information
- **Maintenance Tools**: Clean cache, remove orphans, and update databases
- **Transaction Logging**: Keep track of all package operations with detailed history
- **Flexible Commands**: Support for both short options (`-S`, `-R`, `-Q`) and subcommands

## Supported Commands

| Command | Description |
|---------|-------------|
| `install` | Install packages with options for reinstall and download-only |
| `remove` | Remove packages with autoremove support |
| `update` | Update package database |
| `search` | Search for packages in repositories |
| `list` | List packages (installed, upgradable, or all) |
| `show` | Display detailed package information |
| `depends` | Show package dependencies |
| `clean` | Clean package cache |
| `autoremove` | Remove unused dependencies |
| `history` | View transaction history |

## Installation

> Note: It should be available on ObsidianOS Version 2025-09-15-patch0 onwards.

### From the AUR

```bash
yay -S obsidianospm # or paru. yay comes built into ObsidianOS.
```

### From Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Obsidian-OS/opm.git
   cd opm
   ```

2. **Build with Cargo:**
   ```bash
   cargo build --release
   ```

3. **Install:**
   ```bash
   sudo cp target/release/opm /usr/local/bin/
   ```

### Dependencies

- `pacman` (Arch Linux package manager)
- `obsidianctl` (ObsidianOS's secret sauce)
- `mksquashfs` (for creating SquashFS images)
- `tar` (for package extraction)

## Usage

### Basic Syntax

```bash
opm <command> [options]
```

### Examples

**Install a package:**
```bash
opm install firefox
# or using short options
opm -S firefox
```

**Remove a package:**
```bash
opm remove firefox
# or
opm -R firefox
```

**Search for packages:**
```bash
opm search "web browser"
# or
opm -Ss "web browser"
```

**List installed packages:**
```bash
opm list --installed
# or
opm -Q
```

**Show package information:**
```bash
opm show firefox
# or
opm -Si firefox
```

**Update package database:**
```bash
opm update
```

**Clean cache:**
```bash
opm clean --all
```

**View transaction history:**
```bash
opm history --limit 20
```

**Remove unused dependencies:**
```bash
opm autoremove
```

### Advanced Usage

**Download only (without installing):**
```bash
opm install --download-only package_name
```

**Force reinstall:**
```bash
opm install --reinstall package_name
```

**Show dependencies:**
```bash
opm depends package_name
# or
opm -Sd package_name
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for [ObsidianOS](https://github.com/ObsidianOS)
- Uses Arch Linux's pacman package manager
- Thanks to the Rust community for excellent tooling
