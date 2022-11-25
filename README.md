# SV3G

## Usage

After you clone/download the repo, you can run any/all of the following 4 scripts to get info about the program:

```sh
node SV3G /?

cd SV3G
node . man

# W^X for security
chmod 555 main.js
./main.js help

sudo npm link
sv3g ℹ️
```

## Example

Personally, I use the script like this (in Linux Mint Cinnamon):

```sh
# when using dark-theme in a pitch-black room
sv3g custom '#000' > .wp.svg

# bloody dark, to reduce blue-light emission
sv3g custom '#700' '#000' > .wp.svg
# I also set the accent color to red in that case

# when using light-theme with "mint-green" accent color
sv3g mint > .wp.svg

# when I want to be fully awake and focused
sv3g custom '#fff' > .wp.svg

# pride lol (I usually don't use this, because it's too colorful)
sv3g rainbow > .wp.svg

# warm and comfy
sv3g fire > .wp.svg

# when I just need focus
sv3g wb > .wp.svg
```

Since LMC has an event-listener for wallpaper files, and .wp.svg is already set as WP (by me), LMC will detect the file-update and auto-change the WP background, nice!

Windows users _can only dream_ of such a feature (I feel the urge to say "I use Arch BTW", but I'm not worthy, LMAO)

## Why JS?

This script started as a very basic POSIX-compliant shell script that printed a solid-color SVG. Then, I ported it to both Python and Javascript.

I decided to not use Py, because devs tend to generate client-side SVGs in browsers, and browsers _usually_ (see [PyScript](https://pyscript.net)) use JS.

I'm planning to turn this into a "dual" NPM package, both a program and a library, so that anyone can import the function used by the CLI.

Perhaps I should [RIIR](https://github.com/ansuz/RIIR)?

## Disclaimer

This is (mostly) a tool I use personally, so I'm not responsible for keeping backwards-compatibility, nor to notify about breaking-changes. I may rewrite this in any lang, and target any environment, **at any time without warning**.

I just posted this for people who may have similar (not identical) needs. Use at your own risk.

Speaking of that, look at [this branch](https://github.com/Rudxain/SV3G/tree/RIIR)
