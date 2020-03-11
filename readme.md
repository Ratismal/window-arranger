Note: based on https://github.com/gurrhack/RestoreWindows

# Window Arranger

This program examines the open windows across all monitors and stores their position.

When the number of monitors changes, the program attempts to restore them to their previous positions.

Positions are based on the number of monitors. So, the program will remember the layouts for 1, 2, 3, etc. monitors individually, and attempt to restore based on how many monitors there now are.

## Why?

I switch frequently from having 4 monitors to having 1 (because work laptop). Switching always rearranges my windows, which I hate. So I wanted to fix that.

This project is based on [RestoreWindows](https://github.com/gurrhack/RestoreWindows). But [RestoreWindows](https://github.com/gurrhack/RestoreWindows) is written in c++, and I don't know c++. But I do (vaguely) know Rust. So I wrote a new program in Rust based on [RestoreWindows](https://github.com/gurrhack/RestoreWindows) to more closely suit my needs. All the code was written by me, but I've never used winapi before so I used [RestoreWindows](https://github.com/gurrhack/RestoreWindows) as a reference to how to use it. I just want to be very clear that this project is based on [RestoreWindows](https://github.com/gurrhack/RestoreWindows), and I didn't just fart it out of my own knowledge and skills. If you look at the code in this project, you'll probably find many simularities to [RestoreWindows](https://github.com/gurrhack/RestoreWindows). And maybe also things that aren't similar to [RestoreWindows](https://github.com/gurrhack/RestoreWindows). This project probably wouldn't have existed without [RestoreWindows](https://github.com/gurrhack/RestoreWindows). I would have forked [RestoreWindows](https://github.com/gurrhack/RestoreWindows) if I knew c++, instead of making it all over again in Rust. The [author](https://github.com/gurrhack) of [RestoreWindows](https://github.com/gurrhack/RestoreWindows) is probably super cool and you should check them out.