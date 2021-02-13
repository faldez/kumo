[![Release Stats](https://img.shields.io/github/downloads/faldez/tanoshi/total.svg?logo=github)](https://somsubhra.com/github-release-stats/?username=faldez&repository=tanoshi)

# Tanoshi
Selfhosted web manga reader with extension model.

Rewrite in progress of [Tanoshi](https://github.com/faldez/tanoshi/tree/rust), track issue [here](https://github.com/faldez/tanoshi/issues/137)

## Feature
- Browse manga from various sources (only mangadex and local for now)
- Read in continous vertical, single or double paged
- Read from right to left or right to left
- Fit image to width, height or original size
- Favorite manga for faster access

## Usage
Run using
```
./tanoshi-{os}-{arch} -config /path/to/config.yml
```
Default config can be fount at [configs/config.yml](configs/config.yml)

for local manga, directory structure needs to be
```
/path/to/manga
|_ Series 1
|   |_ Series 1 Chapter1.cbz
|   |_ Series 1 Chapter2.cbz
|_ Series 2
    |_ Series 2 Chapter1.cbz
    |_ Series 2 Chapter2.cbz    
```

### Standalone
### Windows

On windows, localhost access is blocked by default, to enable run below command on command prompt as administrator
```
CheckNetIsolation.exe LoopbackExempt -a -n="Microsoft.Win32WebViewHost_cw5n1h2txyewy"
```
It's also necessary to download both dlls in [build/windows](build/windows) and put theme in same directory as `tanoshi-desktop`

## Screenshot
![](assets/Screen%20Shot%202021-01-31%20at%2016.20.38.png)
![](assets/Screen%20Shot%202021-01-31%20at%2016.23.41.png)
![](assets/Screen%20Shot%202021-01-31%20at%2016.20.34.png)

<img src="assets/IMG_73577C410A56-1.jpeg" width="250">
<img src="assets/IMG_3436B10A2508-1.jpeg" width="250">
<img src="assets/IMG_B8461880E874-1.jpeg" width="250">
