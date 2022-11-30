# SV3G

## Usage

**Don't.** This is still WIP. Go to [this branch](https://github.com/Rudxain/SV3G/tree/legacy-js) in the meantime.

However, if you want to continue anyways, run this:
```sh
cargo install --git https://github.com/Rudxain/SV3G.git && sv3g help
```

## Example

Personally, I use the program like this (in Linux Mint Cinnamon):

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

# when I just need focus and elegance
sv3g wb > .wp.svg
```

Since LMC has an event-listener for wallpaper files, and .wp.svg is already set as WP (by me), LMC will detect the file-update and auto-change the WP background, nice!

Windows users _can only dream_ of such a feature (I feel the urge to say "I use Arch BTW", but I'm not worthy, LMAO)

And yes, I know LMC already has a "no picture gradient" feature, but it has 2 disadvantages:
1. Only supports 2 colors
2. [It isn't shown in the login screen](https://github.com/linuxmint/cinnamon/issues/11229)

The funny thing is that SVGs without `viewBox` aren't rendered correctly in the login screen, so SV3G doesn't fully solve my problem, it's just a dirty patch, lol

## Why Rust?

This started as a very basic POSIX-compliant shell script that printed a solid-color SVG. Then, I ported it to both Python and Javascript.

I decided to not use Py, because devs tend to generate client-side SVGs in browsers, and browsers _usually_ (see [PyScript](https://pyscript.net)) use JS.

Then, I did a [RIIR](https://github.com/ansuz/RIIR) and here we are!

## Disclaimer

This is (mostly) a tool I use personally, so I'm not responsible for keeping backwards-compatibility, nor to notify about breaking-changes. I may rewrite this in any lang, and target any environment, **at any time without warning**.

I just posted this for people who may have similar (not identical) needs. Use at your own risk.

## Related

https://llamalab.com/automate/community/flows/42305
