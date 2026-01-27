# Brandubh Variant Guide

## What is Brandubh?

Brandubh is an Irish variant of Tafl games, played on a smaller 7x7 board. The name means "black raven" in Irish Gaelic.

## Differences from Copenhagen Hnefatafl

| Feature | Copenhagen | Brandubh |
|---------|------------|----------|
| Board Size | 11x11 | 7x7 |
| Attackers | 24 pieces | 8 pieces |
| Defenders | 12 pieces | 4 pieces |
| King | 1 piece | 1 piece |
| Starting Position | Complex cross | Simple cross |

## Board Setup

```
[Brandubh]
    0  1  2  3  4  5  6 
 0  X  .  .  A  .  .  X 
 1  .  .  .  A  .  .  . 
 2  .  .  .  D  .  .  . 
 3  A  A  D  K  D  A  A 
 4  .  .  .  D  .  .  . 
 5  .  .  .  A  .  .  . 
 6  X  .  .  A  .  .  X 
```

- **X** = Corners (king's goal)
- **T** = Throne (center)
- **K** = King
- **D** = Defenders (4)
- **A** = Attackers (8)

## Rules

Same as Copenhagen Hnefatafl:
1. All pieces move like rooks in chess
2. Pieces capture by sandwiching
3. King must be surrounded on all 4 sides to be captured
4. Defenders win if king reaches any corner
5. Attackers win if they capture the king

## Using Brandubh in Code

### Create a Brandubh Game

```rust
use hnefatafl_arena::*;

// Create a Brandubh game state
let state = GameState::new_brandubh();

// Or use the variant enum
let state = GameState::new(Variant::Brandubh);
```

### Play a Brandubh Match

```rust
use hnefatafl_arena::*;
use std::time::Duration;

fn main() {
    let bot1 = Box::new(MyBot::new("Bot1".to_string()));
    let bot2 = Box::new(MyBot::new("Bot2".to_string()));
    
    let config = MatchConfig {
        time_per_move: Duration::from_secs(5),
        max_moves: 100,
    };
    
    // Create a Brandubh match
    let mut match_game = Match::with_variant(
        bot1, 
        bot2, 
        config, 
        true,
        Variant::Brandubh
    );
    
    let result = match_game.play();
}
```

### Copenhagen (Default)

```rust
// These are equivalent and create Copenhagen games:
let state = GameState::new_copenhagen();
let state = GameState::new(Variant::Copenhagen);
let state = GameState::default();

// Match defaults to Copenhagen
let match_game = Match::new(bot1, bot2, config, true);
```

## Strategy Differences

### For Defenders (Brandubh)
- **Faster escapes**: Corners are closer (only 3 moves away)
- **Less protection**: Only 4 defenders to guard the king
- **Open board**: More space to maneuver
- **Critical early game**: Must establish escape route quickly

### For Attackers (Brandubh)
- **Fewer pieces**: Must use 8 attackers efficiently
- **Tighter formation**: Easier to surround king initially
- **Guard corners**: Critical to block 4 escape squares
- **Quick captures**: Less distance to close in

## Game Length

Brandubh games tend to be:
- **Shorter**: Smaller board means faster resolution
- **More tactical**: Less strategic depth, more focus on immediate threats
- **Higher tempo**: King can escape faster

## Example Matches

Run the example:

```bash
cargo run --release --example brandubh_match
```

Compare with Copenhagen:

```bash
cargo run --release --example simple_match
```

## Tournament Considerations

When running tournaments with both variants:

```rust
// Play both variants in round-robin
for variant in [Variant::Copenhagen, Variant::Brandubh] {
    for (bot1, bot2) in all_pairs {
        let match_game = Match::with_variant(
            bot1,
            bot2,
            config,
            false,
            variant
        );
        let result = match_game.play();
    }
}
```

## Bot Adaptations

Your bot can detect which variant is being played:

```rust
impl Bot for MyBot {
    fn get_move(&mut self, state: &GameState, time_limit: Duration) -> Option<Move> {
        match state.variant() {
            Variant::Copenhagen => {
                // Use Copenhagen strategy
            }
            Variant::Brandubh => {
                // Use Brandubh strategy  
            }
        }
        
        // Or check board size
        if state.board_size() == 7 {
            // Brandubh-specific logic
        }
        
        // Your strategy here
    }
}
```

## Historical Note

Brandubh is one of the oldest documented Tafl variants, with references in Irish medieval literature. It was played by Irish nobility and is considered simpler and faster than the Scandinavian variants.
