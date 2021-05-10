# Description
A CLI video player using ASCII characters written with Rust

This project takes a video as the first input, then split it into images and then it transforms those images into text files with each ASCII image and at the end, you can play the video in ypur console

# Dependencies
- `ffmpeg`

# Usage
If you need help to use the program, you can use `asciivid --help` to see the general help or if you need help about a subcommand (let's say `asciivid play`), you can use `asciivid play --help`

First of all, be sure to hae a video. I will create a folder called `asciivid` to store all the artifacts and inside that folder, I will create this:
- `render`
- `split`
- `videos`

I will save my video in the folder `videos` with the name `input.mp4`

Also, we need to know the number of files and columns of our terminal. If you are using linux, you can use this command:
```bash
tput cols & tput lines
```
And in my case, the number of columns is `170` and the number of files is `46`

Now we have to split our video into images and for that, we must use thos command:
```bash
asciivid -w 683 -h 192 split -f "frame_%d.png" -i videos/input.mp4 -o split/
```
the format of the ouput is the same of ffmpeg. You can learn more about it [here](https://en.wikibooks.org/wiki/FFMPEG_An_Intermediate_Guide/image_sequence#Filename_patterns)

After splitting the video, we have to render it into ascii format. We can do it like this:
```bash
asciivid -w 683 -h 192 render -i split/ -o render/
```

And now we have our ASCII video ready to play it. We can do it with this command:

```bash
asciivid -w 683 -h 192 play -i render/
```
