# tcobalt Command Line Utility

tcobalt (or tcb) is a command-line tool for downloading your favorite videos and audios from the internet. This tool uses the api of [wukko](https://github.com/wukko)'s [cobalt.tools](https://cobalt.tools), which is an amazing website to save what you love without ads, trackers, or anything creepy.
<img src=https://github.com/OSCH2008/tcobalt/assets/82794982/f12acec9-c668-4b8b-819c-1256fe802915 style="width: 100%"></img>
Stop using random websites to download videos, just use [cobalt.tools](https://cobalt.tools), or the unofficial command-line version, tcobalt.

## Features
* tcobalt allows you to download videos and audios just as easily as you can with [cobalt.tools](https://cobalt.tools) with `tcb get`
* tcobalt also allows you to download multiple videos/audios at once with `tcb bulk get` and `tcb bulk execute`
* tcobalt allows some integration with other commands, by allowing a url to be piped into `tcb get +`
* tcobalt doesnt have ALL the settings of web cobalt, but it has the ones most people use. This may change later when I feel like it
* tcobalt gives methods to check cobalt's version and to list the supported services right from your terminal
* tcobalt's help method is easy to understand, and gives the toption to list usage examples
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
Full usage help is available with `tcb help`

tcobalt also supports `tcb --help`, `tcb -h`, `tcb --version`, and `tcb -v` for people who have not read the help or this page

## Support
<a href="https://repology.org/project/tcobalt/versions"><img src="https://repology.org/badge/vertical-allrepos/tcobalt.svg" alt="Packaging status" align="right"></a>
As of now, tcobalt only officially supports the x86_64 architecture, but it can theoretically compile and run on others. If tcobalt compiles and runs well on your machine with a different architecture, open up an issue.

The live packaging status can be seen on the right.

### Distro Support roadmap
1. Arch Linux (and its derivatives) (DONE)
2. Debian (and its derivatives) (work in progress, they make it so hard to do so fjgksaertfgvsyuigfyas)
3. Windows 10/11 (DONE)
4. (if i can do this with flatpak) other distros
5. MacOS

### Architecture Support Progress
As of now, the PKGBUILD for tcobalt on the AUR only has 'x86_64' in the arch array, but someone I know was able to install it on their aarch64 system right from the AUR
1. x86_64 - YES
2. aarch64 - Compiled
3. i686 - Not Tested
4. pentium4 - Not Tested

## Installation
Make sure you have rust version 1.77.0 or higher before installing
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
This process will also work on arch-based distros. On Manjaro, run `pamac install` instead of `pacman -S`

### Other / unsupported
More support will come later. If you are on an unsupported operating system, do this:
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

[reqwest](https://crates.io/crates/reqwest), for making web requests with HTTPS. Requires tokio to work.

I have reinvented the wheel for things that are possible for me to do without dependencies, like parsing json.

## License
This project is licensed under the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html), meaning you are allowed to freely use, modify, and distribute this project as long as you keep it as free and open-source as this project is, and as long as you include the same lisence and indicate your changes. More information about this license is [here (fossa.com)](https://fossa.com/blog/open-source-software-licenses-101-gpl-v3/) and [here (gnu.org)](https://www.gnu.org/licenses/quick-guide-gplv3.html).

Cobalt has granted permission for anyone to use the cobalt api in their personal perojects, and not for commercial use. tcobalt is a personal project of mine, and I am not affiliated with wukko in any way.

I would prefer if you credited my work when you share this.
