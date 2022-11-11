# SV3G

This is just a program that I use to set the wallpaper in Linux Mint

## Usage

After you clone/download the repo (duh), you can run any/all of the following 4 scripts to get info about the program:

```sh
# outside
node SVG-gradient-gen /?

# inside
cd SVG-gradient-gen
node . man

# Linux only
chmod 555 main.js
./main.js help

npm link
sv3g /?
```

## Example

Personally, I use the script like this:

```sh
# assuming WD = Desktop

# when using dark-theme in a pitch-black room
sv3g '#000' > bg.svg

# bloody dark, to reduce blue-light emission
sv3g '#700' '#000' > bg.svg
# I also set the accent color to red in that case

# when using light-theme with "mint-green" accent color
sv3g mint > bg.svg

# when I want to be fully awake and focused
sv3g '#fff' > bg.svg

# pride lol (I usually don't use this, because it's too colorful)
sv3g rainbow > bg.svg

# if I'm feeln' fancy
sv3g sky > bg.svg

# when I just need focus
sv3g wb > bg.svg
```

Since Linux Mint has an event-listener for wallpaper files, and bg.svg is already set as wallpaper, LM will detect the file-update and automatically change the wallpaper background. Nice! Windows users can only dream of such a feature (I feel the urge to say "I use Arch BTW", but I'm not worthy, LMAO)
