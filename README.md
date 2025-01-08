### Simple tower defence game made in rust

Map starts out empty. Click on a tile to build a platform, which blocks enemy movement and allows you to build a tower on top. 

Enemies will walk towards your base along the shortest path. The shortest path from spawner to base will be highlighted. The highlight is purely cosmetic - you can build on that path to block it. 

If enemies have no path to your base, they will ignore all platforms and go directly towards it.

### Game can be played here
https://dawidratynski.github.io/rust_tower_defence/

### Controls
- Fast-forward: C
- Pause: P
- Camera:
  - Movement: WASD / Arrow keys
  - Rotation: Q, E
  - Zoom in/out: Z, X
