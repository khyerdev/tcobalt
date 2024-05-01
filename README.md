[![Packaging status](https://repology.org/badge/tiny-repos/tcobalt.svg)](https://repology.org/project/tcobalt/versions)
[![GitHub Downloads](https://img.shields.io/github/downloads/khyerdev/tcobalt/total)](https://github.com/khyerdev/tcobalt/releases/)
[![GitHub Release](https://img.shields.io/github/v/release/khyerdev/tcobalt)](https://github.com/khyerdev/tcobalt/releases/)
[![GitHub License](https://img.shields.io/github/license/khyerdev/tcobalt)](https://github.com/khyerdev/tcobalt/blob/main/LICENSE)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/khyerdev/tcobalt/auto_tests.yml?label=tests)](https://github.com/khyerdev/tcobalt/actions)
[![Evil Purged](https://img.shields.io/badge/evil-purged-red)]()

# tcobalt Command Line Utility

tcobalt (or tcb) is a command-line tool for downloading your favorite videos and audios from the internet. This tool uses the api of [wukko](https://github.com/wukko)'s [cobalt.tools](https://cobalt.tools), which is an amazing website to save what you love without ads, trackers, or anything creepy.
<img src=https://github.com/OSCH2008/tcobalt/assets/82794982/f12acec9-c668-4b8b-819c-1256fe802915 style="width: 100%"></img>
Stop using random websites to download videos, just use [cobalt.tools](https://cobalt.tools), or the unofficial command-line version, tcobalt.

## Features
* tcobalt allows you to download videos and audios just as easily as you can with [cobalt.tools](https://cobalt.tools) with `tcb get`
* tcobalt also allows you to download multiple videos/audios at once with `tcb bulk get` and `tcb bulk execute`
* tcobalt allows some integration with other commands, by allowing a url to be piped into `tcb get +`
* tcobalt includes all the settings that web cobalt has, along with a few extra features that will get extended in the future
* tcobalt gives methods to check cobalt's version and to list the supported services right from your terminal
* tcobalt's help method is easy to understand, and gives the option to list usage examples
* tcobalt supports single letter methods for lazy people `tcb g`, `tcb b e`

## Examples
Basic downloading of a video
```
tcb get 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' -o rickroll.mp4
```
Downloading of a song with '.ogg' format and using the default filename
```
tcb get -af ogg 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'
```
Shuffling a list of youtube links and downloading the first one as a .webm file
```
cat links.txt | shuf | head -n 1 | tcb get -c vp9 +
```
Downloading the same video twice, but with one being audio-only:
```
tcb bulk execute links.txt
```
```
https://www.youtube.com/watch?v=dQw4w9WgXcQ -o roll.mp4
https://www.youtube.com/watch?v=dQw4w9WgXcQ -ao silly-song.mp3
```
Downloading three coding videos at once with 720p resolution each
```
tcb bulk get -q 720 'https://www.youtube.com/watch?v=qclZUQYZTzg' 'https://www.youtube.com/watch?v=wvQCIMjlxHw' 'https://www.youtube.com/watch?v=3T3ZDquDDVg'
```
Full usage help is available with `tcb help`, but can also be seen in the [usage.txt](https://github.com/khyerdev/tcobalt/blob/main/src/strings/usage.txt) and [info.txt](https://github.com/khyerdev/tcobalt/blob/main/src/strings/info.txt) files

tcobalt also supports `tcb --help`, `tcb -h`, `tcb --version`, and `tcb -v` for people who have not read the help or this page

## Support
x86_64 is the only architecture officially supported. It is hard for me to cross-compilr to other architectures or run other architectures. If you are on a different arch, you can compile tcobalt yourself. You can contribute prebuilt binaries too if you feel like it.

### OS/Packager Support Progress <a href="https://repology.org/project/tcobalt/versions"><img src="https://repology.org/badge/vertical-allrepos/tcobalt.svg" alt="Packaging status" align="right"></a>
1. Arch Linux (AUR) (COMPLETE)
2. Windows 10/11 (Prebuilt binary)
3. Fedora
4. Ubuntu (Prebuilt PKG)

The repos that contain tcobalt can be seen on the right, along with the highest version the repo has.

## Installation

### Prebuilt Binary Downloads
| Arch Linux (pkg) | Ubuntu (pkg) | Linux | Windows |
| ---------------- | ------------ | ----- | ------- |
| [v1.1.0-1 x86_64](https://github.com/khyerdev/tcobalt/releases/download/v1.1.0/tcobalt-1.1.0-1-x86_64.pkg.tar.zst) | [v1.0.2 amd64](https://github.com/khyerdev/tcobalt/releases/download/v1.0.2-2/tcobalt-1.0.2-amd64.deb) | [v1.1.0 x86_64](https://github.com/khyerdev/tcobalt/releases/download/v1.1.0/tcobalt-linux-x86_64) | [v1.1.0 x86_64](https://github.com/khyerdev/tcobalt/releases/download/v1.1.0/tcobalt-windows-x86_64.exe) |

### Arch Linux
1. Install `yay` or `paru`:
   ```sh
   sudo pacman -S base-devel
   git clone https://aur.archlinux.org/yay.git # or paru.git
   cd yay # or paru
   makepkg -si
   ```
2. Use either `yay` or `paru` to install `tcobalt`:
   ```
   yay -S tcobalt
   ```
   ```
   paru -S tcobalt
   ```
OR

Get it directly from the AUR without installing an AUR helper:
```
sudo pacman -S base-devel
git clone https://aur.archlinux.org/tcobalt.git
cd tcobalt
makepkg -si
```

OR
1. Download the pkg from the above prebuilt binaries (or use cURL)
2. Open a terminal and navigate to the directory containing the donwloaded .pkg.tar.zst file
3. Run the following command (replacing the fields in the angle brackets as necessary)
   ```
   sudo pacman -U tcobalt-<version>-<rel>-<arch>.pkg.tar.zst
   ```

The PKGBUILD declares that this only supports x86_64, but since it builds from source, yay/paru will allow you to install tcobalt on any architecture

This process will also work on arch-based distros

### Ubuntu
Packaging to a PPA has literally no good documentation online, and the official Ubuntu website for it has LITERALLY NO TEXT ON IT (its undergoing maintenance). Debian docs and launchpad docks also suck. You are stuck with this process until I feel like requesting tcobalt to be added to the official Ubuntu repositories. I won't bother with debian because they release stable updates every 2 years.
1. Download the pkg from the above prebuilt binaries (or use cURL)
2. Open a terminal and navigate to the directory containing the downloaded .deb file
3. Run the following command (replacing the fields in the angle brackets as necessary)
   ```
   sudo dpkg -i tcobalt-<version>-<arch>.deb
   ```

### Windows 10/11
Download the .exe from the above prebuilt binaries, put it into any `%PATH%` folder, and rename it to `tcb.exe`

I will figure out how to add tcobalt to winget-pkgs sometime in the future

### Other Linux
You can try downloading the linux binary from the above prebuilt binaries, putting it into `/usr/bin`, and renaming it to `tcb`

### Unsupported OSes
More support will come later. If you are on an unsupported operating system or architecture, or downloading a prebuilt binary didnt work, do this:
1. Clone the repository
   ```
   git clone https://github.com/khyerdev/tcobalt.git
   cd tcobalt
   ```
2. Compile (make sure rust is installed and the default rust toolchain is also installed)
   * On Linux:
      ```
      make
      ```
   * On Windows/MacOS:
     ```
     cargo build --release
     ```
3. Install tcobalt
   * On Linux:
     ```
     sudo make install
     ```
   * On Windows/MacOS:
     
     Copy the tcobalt binary from `target/release` into a folder that has your PATH and rename it to `tcb` with the file extension corresponding to your OS (`.exe` on windows)

## Dependencies
tcobalt is designed to use as little dependencies as possible. Here are the ones it uses:

The [tokio](https://crates.io/crates/tokio) runtime, for easily handling asynchronous code

[futures](https://crates.io/crates/futures), for easily parralelizing bulk downloads

[reqwest](https://crates.io/crates/reqwest), for making web requests with HTTPS. Requires tokio to work

I have reinvented the wheel for things that are possible for me to do without dependencies, like parsing json

## Other Projects
tcobalt is not original, and a few other projects have the same concept implemented in other languages. Check them out, see if you prefer them over this one.
* [lostdusty/cobalt](https://github.com/lostdusty/cobalt): Cobalt CLI written in golang
* [lostdusty/gobalt](https://github.com/lostdusty/gobalt): Golang library for the Cobalt API
* [tskau/tobalt](https://github.com/tskau/tobalt): Typescript library for the Cobalt API

## License
This project is licensed under the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html), meaning you are allowed to freely use, modify, and distribute this project as long as you keep it as free and open-source as this project is, and as long as you include the same lisence and indicate your changes. More information about this license is [here (fossa.com)](https://fossa.com/blog/open-source-software-licenses-101-gpl-v3/) and [here (gnu.org)](https://www.gnu.org/licenses/quick-guide-gplv3.html).

Cobalt has granted permission for anyone to use the cobalt api in their personal perojects, and not for commercial use. tcobalt is a personal project of mine, and I am not affiliated with wukko in any way.

I would prefer if you credited my work when you share this.
