## Conway's Game of Life
<p>
	<img src="https://github.com/landstander27/game-of-life/actions/workflows/build.yml/badge.svg">
</p>

- GOL made in Rust

### Playing
- Try out the game [here](https://landstander27.github.io/game-of-life/).
- If you want better performance you or just want it stored locally you have 3 options:
  - [Build it yourself](https://github.com/LandStander27/game-of-life#building-from-source).
  - [Grab the latest artifact](https://github.com/LandStander27/game-of-life#getting-latest-build).
  - Get the latest release in the releases tab.

#### Getting latest build
- Go to the actions tab at the top.
- Look for the latest build action, should have `build ##` under it.
- Click on `Game of Life build` at the bottom of the page.

#### Building from source
- Install [Rust](https://www.rust-lang.org/).
- Clone the repo.
- Run `cargo b --release`.
- End binary will be in `.\target\release\gol.exe`.