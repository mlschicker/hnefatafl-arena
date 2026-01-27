# Plugin Bot System

This guide explains how to create bots as compiled plugins where the source code is not visible to other students, and how to use **pondering** to think during the opponent's turn.

## Overview

The plugin system allows you to:
- **Compile your bot as a shared library** (`.so`, `.dll`, or `.dylib`)
- **Hide your source code** - only distribute the compiled binary
- **Enable pondering** - continue thinking while your opponent makes their move
- **Use full Rust capabilities** in your bot implementation

## Why Use Plugins?

1. **Privacy**: Students can't see each other's strategies
2. **Performance**: Native compiled code runs at full speed
3. **Pondering**: Think during opponent's turn for competitive advantage
4. **Distribution**: Easy to share compiled bots without source code

## Example Plugin Bots

Two complete examples are included:

- **Greedy Bot** (`plugins/greedy_bot_plugin/`) - Simple single-move evaluation
- **Alpha-Beta Bot** (`plugins/alphabeta_bot_plugin/`) - Advanced minimax search
  - Wins as defenders in ~8 moves vs greedy bot
  - Uses exponential king position evaluation
  - Implements immediate win detection
  - Great reference for competitive bots!

## Creating a Plugin Bot

### Step 1: Set up the Plugin Project

Create a new directory in `plugins/`:

```bash
mkdir -p plugins/my_bot
cd plugins/my_bot
```

### Step 2: Create `Cargo.toml`

```toml
[package]
name = "my_bot_plugin"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]  # This creates a shared library

[dependencies]
hnefatafl-arena = { path = "../.." }
```

**Important**: `crate-type = ["cdylib"]` tells Rust to compile as a dynamic library.

### Step 3: Implement Your Bot

Create `src/lib.rs`:

```rust
use hnefatafl_arena::{Bot, GameState, Move, Player};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct MyBot {
    name: String,
    pondering: Arc<AtomicBool>,
    // Add your internal state here
}

impl Default for MyBot {
    fn default() -> Self {
        Self {
            name: "MyBot".to_string(),
            pondering: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Bot for MyBot {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&mut self, state: &GameState, time_limit: Duration) -> Option<Move> {
        // Your strategy here
        let moves = state.legal_moves();
        if moves.is_empty() {
            None
        } else {
            Some(moves[0])  // Replace with your logic
        }
    }

    fn game_start(&mut self, player: Player) {
        // Called when game starts - initialize your state
        self.pondering.store(false, Ordering::Relaxed);
    }

    fn notify_move(&mut self, mv: Move) {
        // Called when any move is made (yours or opponent's)
        // Update your internal game state here
    }

    fn game_end(&mut self) {
        // Called when game ends - cleanup
        self.pondering.store(false, Ordering::Relaxed);
    }

    fn opponent_thinking(&mut self, state: &GameState) {
        // Called when opponent is thinking - USE THIS TIME!
        self.pondering.store(true, Ordering::Relaxed);
        
        // Example: Start pre-computing moves
        // In a real bot, run this in a loop checking the pondering flag
        while self.pondering.load(Ordering::Relaxed) {
            // Do useful work: evaluate positions, build search trees, etc.
            // Break when stop_pondering() is called
        }
    }

    fn stop_pondering(&mut self) {
        // Called when opponent finishes their move
        self.pondering.store(false, Ordering::Relaxed);
    }
}

// REQUIRED: Export your bot using this macro
hnefatafl_arena::export_bot!(MyBot);
```

### Step 4: Compile Your Plugin

```bash
cd plugins/my_bot
cargo build --release
```

This creates:
- **Linux**: `target/release/libmy_bot_plugin.so`
- **macOS**: `target/release/libmy_bot_plugin.dylib`
- **Windows**: `target/release/my_bot_plugin.dll`

### Step 5: Distribute Your Bot

Share only the compiled library file - **not** the source code! Students can use your bot without seeing how it works.

## Using Plugin Bots

### Loading a Plugin

```rust
use hnefatafl_arena::{PluginBot, Match, MatchConfig};
use std::time::Duration;

fn main() {
    // Load the plugin (adjust path for your OS)
    let my_bot = PluginBot::load("path/to/libmy_bot_plugin.so")
        .expect("Failed to load plugin");
    
    let opponent = Box::new(RandomBot::new("Opponent".to_string()));
    
    let config = MatchConfig {
        time_per_move: Duration::from_secs(5),
        max_moves: 200,
        enable_pondering: true,  // Enable pondering!
    };
    
    let mut game = Match::new(
        Box::new(my_bot),
        opponent,
        config,
        true  // verbose
    );
    
    let result = game.play();
}
```

### Running the Example

```bash
# First, compile the example plugin
cd plugins/greedy_bot_plugin
cargo build

# Then run the example
cd ../..
cargo run --example plugin_match
```

## Implementing Pondering

Pondering allows your bot to think during the opponent's turn. Here are strategies:

### Strategy 1: Simple Pre-computation

```rust
fn opponent_thinking(&mut self, state: &GameState) {
    self.pondering.store(true, Ordering::Relaxed);
    
    // Evaluate likely opponent moves
    let moves = state.legal_moves();
    for mv in moves {
        if !self.pondering.load(Ordering::Relaxed) {
            break;  // Stop if called
        }
        self.precompute_response(state, mv);
    }
}
```

### Strategy 2: Background Thread

```rust
use std::thread;

fn opponent_thinking(&mut self, state: &GameState) {
    let pondering = Arc::clone(&self.pondering);
    let state = state.clone();
    
    pondering.store(true, Ordering::Relaxed);
    
    thread::spawn(move || {
        while pondering.load(Ordering::Relaxed) {
            // Do expensive computation
        }
    });
}
```

### Strategy 3: Speculative Search

```rust
fn opponent_thinking(&mut self, state: &GameState) {
    // Predict opponent's most likely moves
    let likely_moves = self.predict_opponent_moves(state);
    
    for mv in likely_moves {
        if !self.pondering.load(Ordering::Relaxed) {
            break;
        }
        
        // Pre-compute your response to this move
        let mut future_state = state.clone();
        future_state.make_move(mv).ok();
        let response = self.compute_best_move(&future_state);
        
        // Cache the response
        self.cache_move(mv, response);
    }
}
```

## Best Practices

### 1. Always Check the Pondering Flag

```rust
while self.pondering.load(Ordering::Relaxed) {
    // Your work here
}
```

### 2. Use Atomic Operations for Thread Safety

```rust
use std::sync::atomic::{AtomicBool, Ordering};

let pondering = Arc::new(AtomicBool::new(false));
```

### 3. Clean Up in stop_pondering()

```rust
fn stop_pondering(&mut self) {
    self.pondering.store(false, Ordering::Relaxed);
    // Cancel any ongoing work
    // Save useful results
}
```

### 4. Don't Block in opponent_thinking()

The function should return quickly. Use threads or iterative processing.

## Advanced Topics

### Caching Evaluations

```rust
use std::collections::HashMap;

struct MyBot {
    eval_cache: HashMap<GameState, i32>,
    pondering: Arc<AtomicBool>,
}

impl Bot for MyBot {
    fn opponent_thinking(&mut self, state: &GameState) {
        // Pre-compute evaluations during pondering
        for mv in state.legal_moves() {
            if !self.pondering.load(Ordering::Relaxed) {
                break;
            }
            let mut next = state.clone();
            next.make_move(mv).ok();
            let eval = self.evaluate(&next);
            self.eval_cache.insert(next, eval);
        }
    }
}
```

### Transposition Tables

Share data between pondering and actual move computation:

```rust
struct MyBot {
    transposition_table: Arc<Mutex<HashMap<u64, i32>>>,
}
```

## Tournament Setup

Mix plugin and regular bots:

```rust
let bot1 = PluginBot::load("plugins/bot1.so")?;
let bot2 = PluginBot::load("plugins/bot2.so")?;
let bot3 = Box::new(RandomBot::new("Random".to_string()));

let config = MatchConfig {
    enable_pondering: true,
    ..Default::default()
};

// Run matches...
```

## Troubleshooting

### Plugin Won't Load

```
Error: Failed to load library: libmy_bot_plugin.so
```

**Solution**: Make sure the file exists and is in the correct location. Use absolute paths if needed.

### Missing Symbol

```
Error: Failed to find create_bot function
```

**Solution**: Make sure you added `hnefatafl_arena::export_bot!(YourBot);` at the end of your lib.rs.

### Wrong Library Type

```
Error: crate-type must be cdylib
```

**Solution**: Add `crate-type = ["cdylib"]` to `[lib]` section in Cargo.toml.

## Example: Complete Pondering Bot

See `plugins/greedy_bot_plugin/` for a complete example that:
- ✅ Compiles as a plugin
- ✅ Implements pondering
- ✅ Uses atomic operations for thread safety
- ✅ Pre-computes move evaluations

## Security Note

While plugin bots hide source code, remember:
- Compiled code can be reverse-engineered (though it's difficult)
- For true security in competitions, run bots in sandboxed environments
- The plugin system provides privacy, not cryptographic security

## Learning Resources

- **Pondering**: Used in chess engines like Stockfish
- **Transposition Tables**: Common in game tree search
- **Thread Safety**: Essential for concurrent computation
- **FFI**: Rust's Foreign Function Interface for interop

## Next Steps

1. Create your own plugin bot
2. Implement basic pondering
3. Test with `enable_pondering: false` vs `true`
4. Measure time savings from pre-computation
5. Enter tournaments!
