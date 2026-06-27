# Undertale Demo Code

This is the original GML source code from the [Undertale Demo](https://undertale.com/demo/),
extracted via [OpenGMK](https://github.com/OpenGMK/OpenGMK) and some custom code in the `__gen` directory.

This is quite a bit different than the decompiled bytecode you will see for Undertale or Deltarune because it has been completely preserved.
All comments are as Toby Fox wrote them and all whitespace is how it was originally written.

Source code access provides another upside.
The bytecode decompiler used for modern GameMaker games is [Underanalyzer](https://github.com/UnderminersTeam/Underanalyzer).
It can only tell in certain circumstances whether `0` is supposed to be a number, the first game object,
the first room, the first sprite, the color `c_black`/`#000000`, the constant `bm_normal`, the constant `fa_left`, or anything else.
This repository provides the original source code, which does use the proper constants and asset names, which makes the intent immediately clear.

## Notes

The provided directory/file structure was arbitrarily chosen by me to make it convenient and readable. It does not follow any official GameMaker format.

Theoretically, there would be room (instance) creation code and timelines exported here as well, but the Undertale Demo does not use them at all.

Only the GML source code is unpacked here, which is only a small portion of the game asset data.
If you wish to fully understand the context behind these scripts and events, you will need to download GameMaker 8.1 from some shady website
and load the `.gmk` file. To load it successfully, you also need to go to *Extension Packages* in the left sideview and enable *Caster*.

Most game object event actions contain their source code directly. That source code is completely untouched in the GML file.
However, some actions use GM8's Drag and Drop blocks to call functions. Those have been reconstructed into GML that calls the given function.
That code was not written by Toby Fox directly, which is why it has a comment: `// This action is a function call and was recreated as GML code`.

## Legal

This repository is not affiliated with Toby Fox or the Undertale team.
If you are a representative of Toby Fox or Fangamer and wish to have the site taken down, please send an email to
[biotomatede@proton.me](mailto:biotomatede@proton.me?subject=Undertale%20Demo%20Source%20Code%20Takedown%20Request).
