# tcbobalt Command Line Utility

tcobalt (or tcb) is a command-line tool for downloading your favorite videos and audios from the internet. This tool uses the api of [wukko](https://github.com/wukko)'s [cobalt.tools](https://cobalt.tools), which is an amazing website to save what you love without ads, trackers, or anything creepy.
<img src=https://github.com/OSCH2008/tcobalt/assets/82794982/f12acec9-c668-4b8b-819c-1256fe802915 style="width: 100%"></img>
Stop using random websites to download videos, just use [cobalt.tools](https://cobalt.tools), or the unofficial command-line version, tcobalt.

## Features
* tcobalt allows you to download videos and audios just as easily as you can with [cobalt.tools](https://cobalt.tools) with `tcb get`
* tcobalt also allows you to download multiple videos/audios at once with `tcb bulk get` and `tcb bulk execute`
* tcobalt allows some integration with other command-line tools, by allowing a url to be piped into `tcb get +`
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

## Installation
This section will be made later, compile this yourself.
### Support roadmap
1. Arch Linux (and its derivatives)
2. Debian (and its derivatives)
3. Windows 10/11
4. (if i can do this with flatpak) other distros
5. MacOS

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
