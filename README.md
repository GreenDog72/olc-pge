# olc-pge

An *unofficial* reimplementation of the olcPixelGameEngine in Rust. If there is ever an official version of the olcPixelGameEngine for Rust, I will happily give up the crate.

This is a derivative work of the olcPixelGameEngine and as such, uses the OLC-3 license. This license is included in `LICENSE.md` with the source code, and at the end of this Readme.

## Missing Features

* Real documentation
* Anything added in PGE 2.0 or above
* Resource Packs
* Mouse buttons higher than 2
* Fullscreen
* Vsync
* `PixelMode::Custom` just functions as `PixelMode::Normal`
* `set_screen_size()` does nothing
* `set_sub_pixel_offset()` does nothing

## Added Features

Added support for almost every key a on a US keyboard, including left/right versions of control, shift, alt, and the windows key. I added these because I wanted to take a whack at making a text editor, and I needed more keys.

The `Return` key is the one on most people call enter. The `Enter` key is both the `Return` key and the `NumPadEnter` key. Similar to how `Shift` is both `LeftShift` and `RightShift`.

Currently, for reasons I have yet to explain, alt and F10 can't be captured. This seems to be a limitation of `minifb`. It's probably limited to Windows. I'm still looking in to it.

The Windows key also can't be captured, but whatever, I didn't really expect to be able to, but it's *available* in the enum as `System`, `LeftSystem`, and `RightSystem`. Maybe they work on Linux or Mac? Who knows? Not me.

I also didn't add `F13`-`F24`. I'm sure none of you actually need them, most of you didn't know they even existed, and minifb only supports up to `F15` anyway.

## Platforms

In theory, it supports anything that `image` and `minifb` do, but it's only been tested on Windows. Please let me know if it doesn't work on other platforms. I will at least *attempt* to fix it, but if you provide a fix, that saves me the effort.

## Basic Use

Here's the PGE Example program in all it's glory using Rust.

```rust
use olc_pge as olc;

use rand::Rng;

pub struct Example;

impl olc::PGEApplication for Example {
    const APP_NAME: &'static str = "Example - Rust";

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, _: f32) -> bool {
        let mut rng = rand::thread_rng();

        for x in 0..pge.screen_width() as i32 {
            for y in 0..pge.screen_height() as i32 {
                pge.draw(x, y, olc::Pixel::rgb(rng.gen(), rng.gen(), rng.gen()));
            }
        }

        true
    }
}

fn main() {
    olc::PixelGameEngine::construct(Example, 256, 240, 4, 4).start();
}
```

You just implement `PGEApplication` for some `struct` and you're mostly there. `APP_NAME` and `on_user_update()` are the only thngs that *required* but you still have access to `on_user_create()` and `on_user_destroy()`.

The full definition for `PGEApplication` looks like this:
```rust
pub trait PGEApplication {
    const APP_NAME: &'static str;
    fn on_user_create(&mut self, pge: &mut PixelGameEngine) -> bool { true }
    fn on_user_update(&mut self, pge: &mut PixelGameEngine, elapsed_time: f32) -> bool;
    fn on_user_destroy(&mut self) -> bool { true }
}
```

All the functions you would normally just call, like `DrawSprite()` are now wrapped up in a `PixelGameEngine` accessed through `pge`. Aside from that, and just getting used to Rust instead of C++, it should be a relatively straight-forward experience. Except for the fun bits in the next section.

## Changes to Accomodate Rust

### Function Overloads / Default Parameters

Rather, lack thereof. Rust doesn't support this. Drawing functions that supported an optional `scale`, `pattern`, or `mask`, and could handle either vectors or individual components, now have multiple functions.
```cpp
void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, Pixel p = olc::WHITE, uint32_t pattern = 0xFFFFFFFF);
```
This one line became the the following 4 functions:
```rust
fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel);
fn draw_line_pattern(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32);

fn draw_line_v(&mut self, pos1: Vi2d, pos2: Vi2d, p: Pixel);
fn draw_line_pattern_v(&mut self, pos1: Vi2d, pos2: Vi2d, p: Pixel, pattern: u32);
```
Isn't it just grand? I made the decision to make the color mandatory. I think most of the time you would be specifying it anyway, so it doesn't feel like a downgrade.

### Sprites

The most glaring thing will be the changes to sprites. Every function that originally took a `Sprite *` now takes a `SpriteRef`. This is because raw pointers are unsafe, and really shouldn't be used in rust unless you know damn well what you're going to do with them. I don't know what I'm doing with them, and more importantly, I don't know what *you* are going to do with them. `SpriteRef` is nice and safe. In theory.

To make a `SpriteRef`, call `into_ref()` on a newly created sprite. To pass one in to a function, you're going to have to `clone()` it.

As an example:
```rust
let sprite = olc::Sprite::from_file("path/to/file.png").into_ref();
pge.draw_sprite(0, 0, sprite.clone());
```

A `SpriteRef` is defined as `Rc<RefCell<Sprite>>`. To those unfamiliar, an `Rc<>` gives you a reference counted, *immutable*, reference. Because an `Rc<>` is only ever immutable, and I'm pretty sure some people would like to edit sprites at runtime, we need the `RefCell<>`. These neat little structures provide *interior mutability*. Which is a fancy way of saying you can get a mutable reference to an object inside of an immutable one. A `RefCell<>` has the important feature that it enforces Rust's borrowing rules at runtime. That is to say, you can have 1 mutable reference *OR* many immutable references. Not both.

To do anything through a `SpriteRef`, you'll have to explicitly borrow it first:
```rust
let immutable_ref = sprite.borrow();
// These cannot actually coexist and must be in different scopes
let mutable_ref = sprite.borrow_mut();
```

### Draw Targets

The original C++ API let you just throw any old `Sprite*` in you were off to the races. As discussed above, you can't do that here. It also allowed `null` as target to get back to the default target. I don't really know why, but I tried to keep similar functionality. The argument for `set_draw_target()` is an `Option<SpriteRef>`.
```rust
// to set a custom draw target
pge.set_draw_target(Some(sprite.clone());
// to go back to the default
pge.set_draw_target(None);
```

# License (OLC-3)

Copyright 2018-2021 OneLoneCoder.com

Redistribution and use in source and binary forms, with or without 
modification, are permitted provided that the following conditions 
are met:

1. Redistributions or derivations of source code must retain the above 
   copyright notice, this list of conditions and the following disclaimer.

2. Redistributions or derivative works in binary form must reproduce 
   the above copyright notice. This list of conditions and the following 
   disclaimer must be reproduced in the documentation and/or other 
   materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its 
   contributors may be used to endorse or promote products derived 
   from this software without specific prior written permission.
    
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT 
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR 
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT 
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, 
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT 
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, 
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY 
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT 
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
