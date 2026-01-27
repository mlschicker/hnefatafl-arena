# Quick Start: Creating a Plugin Bot with Pondering

This guide gets you up and running with a plugin bot in 5 minutes!

## 1. Create Your Plugin Directory

```bash
mkdir -p plugins/my_awesome_bot/src
cd plugins/my_awesome_bot
```

## 2. Create `Cargo.toml`

```toml
[package]
name = "my_awesome_bot"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
hnefatafl-arena = { path = "../.." }
```

## 3. Create `src/lib.rs`

```rust
use hnefatafl_arena::{Bot, GameState, Move, Player};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct MyAwesomeBot {
    name: String,
    pondering: Arc<AtomicBool>,
}

impl Default for MyAwesomeBot {
    fn default() -> Self {
        Self {
            name: "MyAwesomeBot".to_string(),
            pondering: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Bot for MyAwesomeBot {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&mut self, state: &GameState, _time_limit: Duration) -> Option<Move> {
        // TODO: Implement your strategy here!
        let moves = state.legal_moves();
        moves.first().copied()
    }

    fn opponent_thinking(&mut self, state: &GameState) {
        // TODO: Use this time to think ahead!
        self.pondering.store(true, Ordering::Relaxed);
        
        // Example: Pre-compute evaluations
        let moves = state.legal_moves();
        for mv in moves.iter().take(10) {
            if !self.pondering.load(Ordering::Relaxed) {
                break;
            }
            // Do useful work here
        }
    }

    fn stop_pondering(&mut self) {
        self.pondering.store(false, Ordering::Relaxed);
    }
}

// REQUIRED: Export your bot
hnefatafl_arena::export_bot!(MyAwesomeBot);
```

## 4. Compile Your Bot

```bash
cargo build --release
```

This creates: `target/release/libmy_awesome_bot.so` (or `.dylib` on macOS, `.dll` on Windows)

## 5. Test Your Bot

Create `test_my_bot.rs` in the arena's examples folder:

```rust
use hnefatafl_arena::{Bot, Match, MatchConfig, PluginBot, RandomBot};
use std::time::Duration;

fn main() {
    let my_bot = PluginBot::load("plugins/my_awesome_bot/target/release/libmy_awesome_bot.so")
        .expect("Failed to load bot");
    
    let opponent = Box::new(RandomBot::new("Random".to_string()));
    
    let config = MatchConfig {
        time_per_move: Duration::from_secs(5),
        enable_pondering: true,
        ..Default::default()
    };
    
    let mut game = Match::new(Box::new(my_bot), opponent, config, true);
    let result = game.play();
    
    println!("\nResult: {:?}", result);
}
```

Run it:
```bash
cargo run --example test_my_bot
```

## 6. Share Your Bot

Distribute only the compiled library file:
- `libmy_awesome_bot.so` (Linux)
- `libmy_awesome_bot.dylib` (macOS)  
- `my_awesome_bot.dll` (Windows)

**Your source code stays private!** 🔒

## Tips for Success

### 💡 Make Your Bot Smarter

1. **Evaluate positions**: Count pieces, check king safety
2. **Search ahead**: Try multiple moves and pick the best
3. **Use pondering**: Pre-compute while opponent thinks
4. **Cache results**: Remember evaluations you've already done

### ⚡ Use Pondering Effectively

```rust
fn opponent_thinking(&mut self, state: &GameState) {
    self.pondering.store(true, Ordering::Relaxed);
    
    // Predict opponent's likely moves
    let opponent_moves = self.predict_moves(state);
    
    for mv in opponent_moves {
        if !self.pondering.load(Ordering::Relaxed) {
            break;  // Stop when called
        }
        
        // Pre-compute your response
        let mut future = state.clone();
        future.make_move(mv).ok();
        let response = self.find_best_move(&future);
        self.cache_response(mv, response);
    }
}
```

### 🎯 Strategy Ideas

**For Attackers:**
- Surround the king
- Control center squares
- Block escape routes to corners

**For Defenders:**
- Protect the king
- Create escape paths
- Sacrifice pieces to clear the way

## Next Steps

1. Read [PLUGIN_GUIDE.md](PLUGIN_GUIDE.md) for detailed information
2. Check [API_REFERENCE.md](API_REFERENCE.md) for all available methods
3. Study the example plugins:
   - **Greedy Bot**: `plugins/greedy_bot_plugin/` - Simple starting point
   - **Alpha-Beta Bot**: `plugins/alphabeta_bot_plugin/` - Advanced AI (wins in ~8 moves!)
4. Enter tournaments and compete!

## Study the Examples

Before writing your own bot, check out the included examples:

### Simple Example: Greedy Bot
```bash
cat plugins/greedy_bot_plugin/src/lib.rs
```
- Single-move evaluation
- Simple pondering
- Good starting point

### Advanced Example: Alpha-Beta Bot
```bash
cat plugins/alphabeta_bot_plugin/src/lib.rs
```
- Minimax search with alpha-beta pruning
- Exponential king evaluation (distance 1 = 50,000 points!)
- Immediate win detection
- Iterative deepening
- Strong competitive play

**Performance**: Alpha-beta bot wins as defenders in ~8 moves vs greedy bot!

## Common Issues

**"Failed to load library"**: Use the full path or correct relative path

**"Missing create_bot function"**: Did you add `export_bot!(YourBot);`?

**"Wrong crate type"**: Make sure `Cargo.toml` has `crate-type = ["cdylib"]`

## Resources

- **Greedy Plugin**: `plugins/greedy_bot_plugin/` - Simple working example
- **Alpha-Beta Plugin**: `plugins/alphabeta_bot_plugin/` - Advanced AI with minimax search
- **Algorithm Details**: [plugins/alphabeta_bot_plugin/README.md](../plugins/alphabeta_bot_plugin/README.md)
- **Test Examples**: `examples/plugin_match.rs`, `examples/alphabeta_match.rs`
- **Full Guide**: [PLUGIN_GUIDE.md](PLUGIN_GUIDE.md) - Comprehensive plugin documentation
- **API Reference**: [API_REFERENCE.md](API_REFERENCE.md) - Complete API docs

Good luck! 🚀
