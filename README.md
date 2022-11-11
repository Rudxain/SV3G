# SV3G

This is just a program that I use to set the wallpaper in Linux Mint

## Usage

After you clone/download the repo (duh), you can use any of the following 3 scripts to get info about the program:

```sh
# outside
node SVG-gradient-gen /?

# inside
cd SVG-gradient-gen
node . man

# Linux only
chmod 644 main.js
./main.js help
```

## Example

Personally, I use the script like this:

```sh
# assuming WD and package are at the Desktop

# when using dark-theme in a pitch-black room
node SVG-gradient-gen '#000' > bg.svg

# bloody dark, to reduce blue-light emission
node SVG-gradient-gen '#700' '#000' > bg.svg
# I also set the accent color to red in that case

# when using light-theme with "mint-green" accent color
node SVG-gradient-gen mint > bg.svg

# when I want to be fully awake and focused
node SVG-gradient-gen '#fff' > bg.svg

# pride lol (I usually don't use this, because it's too colorful)
node SVG-gradient-gen rainbow > bg.svg

# if I'm feeln' fancy
node SVG-gradient-gen sky > bg.svg

# when I just need focus
node SVG-gradient-gen wb > bg.svg
```

Since Linux Mint has an event-listener for wallpaper files, and bg.svg is already set as wallpaper, LM will detect the file-update and automatically change the wallpaper background. Nice! Windows users can only dream of such a feature (I feel the urge to say "I use Arch BTW", but I'm not worthy, LMAO)
