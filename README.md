![App Preview](./preview.png)

# To run

```
# if you are on ubuntu: apt install libx11-dev libxi-dev libgl1-mesa-dev

cargo run --release
```

Alternatively, you can download and run one of the [precompiled binaries](https://github.com/bddap/dss-takehome-andrew-dirksen/tags).

The first time the program is launched it takes a bit because all data is pulled from the api eagerly before rendering starts. To speed development, network requests are cached so the second launch will be quicker.

Press escape to exit the running program. Arrow keys navigate.
