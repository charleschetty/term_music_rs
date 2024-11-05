# term_music_rs

A terminal music player written in Rust, which uses very low resources and is superfast.


![music](/shots/1.png)
![demo](/shots/demo.png)
![helper](/shots/helper.png)
![btm](/shots/btm.png)


## Build from source

> You should install [rust](https://www.rust-lang.org/tools/install) first

```shell
git clone https://github.com/charleschetty/term_music_rs
cd term_music_rs
cargo build --release
# then copy term_music_rs under target/release/ to desired path you want
```

*note : In order to display icons correctly in the terminal, you should install [nerd font](https://github.com/ryanoasis/nerd-fonts) first.*

*I only test it on Arch Linux*

## Usage

Press `Tab` to see the helper.

# Terminal Music Player

### File Browser
| Shortcut          | Action                                         |
|-------------------|------------------------------------------------|
| `q / ESC`         | Quit                                           |
| `l / Right`       | Switch To Playing List                        |
| `j / Down`        | Select Next Item                              |
| `k / Up`          | Select Previous Item                          |
| `g`               | Select First Item                             |
| `G`               | Select Last Item                              |
| `a / Enter`       | Add Music To Playing List                     |
| `A`               | Add All The Music In This Folder To Playing List |
| `o`               | Open Folder                                   |
| `Backspace`       | Close Folder                                  |

---

### Playing List
| Shortcut          | Action                                         |
|-------------------|------------------------------------------------|
| `q / ESC`         | Quit                                           |
| `h / Left`        | Switch To File Browser                        |
| `j / Down`        | Select Next Item                              |
| `k / Up`          | Select Previous Item                          |
| `g`               | Select First Item                             |
| `G`               | Select Last Item                              |
| `Enter`           | Play Current Music                            |
| `p`               | Play / Pause                                  |
| `s`               | Stop Playing                                  |
| `n`               | Play Next Music                               |
| `d`               | Remove from Playing List (slow)              |
| `D`               | Remove from Playing List (fast, but may change order) |
| `m`               | Change Playing Mode (Auto|Repeat|Random|Manual) |
| `+`               | Volume Up                                     |
| `-`               | Volume Down                                   |

---

### Helper
| Shortcut          | Action                                         |
|-------------------|------------------------------------------------|
| `j / Down`        | Select Next Item                              |
| `k / Up`          | Select Previous Item                          |
| `q / ESC / Tab`   | Quit Helper                                   |
## Todo

- [ ] User configuration


## Reference 

[kronos](https://github.com/TrevorSatori/kronos) , [music_player](https://github.com/ZegWe/music-player)
