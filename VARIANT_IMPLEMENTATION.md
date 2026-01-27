# Brandubh Variant - Implementation Summary

## What Was Added

The Hnefatafl Arena now supports **two game variants**:
1. **Copenhagen Hnefatafl** (11x11) - Original implementation
2. **Brandubh** (7x7) - New Irish variant

## Changes Made

### 1. Core Game Engine (`src/game.rs`)

#### New Types
```rust
pub enum Variant {
    Copenhagen,  // 11x11, traditional
    Brandubh,    // 7x7, Irish variant
}
```

#### New Constants
```rust
pub const COPENHAGEN_SIZE: usize = 11;
pub const BRANDUBH_SIZE: usize = 7;
pub const MAX_BOARD_SIZE: usize = 11;
```

#### Updated GameState
- Now stores `variant: Variant` and `board_size: usize`
- Board array uses `MAX_BOARD_SIZE` for flexibility
- All board operations use `self.board_size` instead of constant

#### New Factory Methods
```rust
GameState::new(variant: Variant)           // Create with specific variant
GameState::new_copenhagen()                 // Create Copenhagen (11x11)
GameState::new_brandubh()                   // Create Brandubh (7x7)
GameState::default()                        // Defaults to Copenhagen
```

#### New Setup Functions
- `setup_copenhagen()` - Sets up 11x11 board
- `setup_brandubh()` - Sets up 7x7 board with Irish layout

#### New Accessors
```rust
state.variant() -> Variant
state.board_size() -> usize
```

### 2. Match System (`src/arena.rs`)

#### Updated Match Constructor
```rust
// Default Copenhagen match
Match::new(attacker, defender, config, verbose)

// Specify variant
Match::with_variant(attacker, defender, config, verbose, variant)
```

### 3. Bot Compatibility (`src/bot.rs`)

- Updated example bots to use `state.board_size()` instead of constant
- Bots can now detect variant and adapt strategy

### 4. New Example

**`examples/brandubh_match.rs`**
- Demonstrates Brandubh gameplay
- Shows initial board setup
- Explains differences from Copenhagen

### 5. Documentation

**New Files:**
- `BRANDUBH.md` - Complete guide to Brandubh variant
  - Rules and setup
  - Strategy differences
  - Code examples
  - Historical context

**Updated Files:**
- `README.md` - Added variant support section
- `API_REFERENCE.md` - Would need updating with variant methods

## Brandubh Setup

### Board Layout (7x7)
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

### Piece Counts
- **Attackers**: 8 pieces (vs 24 in Copenhagen)
- **Defenders**: 4 pieces (vs 12 in Copenhagen)
- **King**: 1 piece (same)
- **Total**: 13 pieces (vs 37 in Copenhagen)

## Usage Examples

### Create Brandubh Game
```rust
// Three equivalent ways
let state = GameState::new_brandubh();
let state = GameState::new(Variant::Brandubh);

// Check variant
assert_eq!(state.variant(), Variant::Brandubh);
assert_eq!(state.board_size(), 7);
```

### Play Brandubh Match
```rust
let bot1 = Box::new(MyBot::new("Bot1".to_string()));
let bot2 = Box::new(MyBot::new("Bot2".to_string()));
let config = MatchConfig::default();

let mut match_game = Match::with_variant(
    bot1, 
    bot2, 
    config, 
    true,
    Variant::Brandubh
);

let result = match_game.play();
```

### Bot Adaptation
```rust
impl Bot for MyBot {
    fn get_move(&mut self, state: &GameState, time_limit: Duration) -> Option<Move> {
        // Detect variant
        match state.variant() {
            Variant::Copenhagen => {
                // Use long-term strategy
            }
            Variant::Brandubh => {
                // Use aggressive, fast strategy
            }
        }
        
        // Or check board size
        if state.board_size() == 7 {
            // Brandubh-specific tactics
        }
    }
}
```

## Testing

### Run Copenhagen
```bash
cargo run --release --example simple_match
```

### Run Brandubh
```bash
cargo run --release --example brandubh_match
```

### Both work correctly!
- ✅ Copenhagen displays as 11x11
- ✅ Brandubh displays as 7x7
- ✅ Variant name shown in board display
- ✅ All rules enforced correctly
- ✅ Bots work with both variants

## Backward Compatibility

All existing code continues to work:
```rust
// Still works - defaults to Copenhagen
let state = GameState::default();
let match_game = Match::new(bot1, bot2, config, true);
```

## Benefits

1. **Educational**: Students learn about game variants
2. **Faster testing**: Brandubh games are quicker
3. **Strategy variety**: Different optimal strategies
4. **Historical accuracy**: Both are authentic Tafl games
5. **Flexible**: Easy to add more variants in future

## Future Variants

The system is now set up to easily add:
- Tablut (9x9, another Swedish variant)
- Ard Ri (7x7, similar to Brandubh)
- Tawlbwrdd (11x11, Welsh variant)
- Custom sizes for experimentation

Just add to the `Variant` enum and implement setup!

## Performance Impact

- **None**: Board size checked at runtime but operations are identical
- Memory: Uses MAX_BOARD_SIZE (11) for all variants
- Small unused space for Brandubh (7x7 in 11x11 array)
- Negligible overhead for typical gameplay

## Files Modified

1. ✅ `src/game.rs` - Core variant support
2. ✅ `src/arena.rs` - Match variant selection
3. ✅ `src/bot.rs` - Dynamic board size
4. ✅ `README.md` - Documentation updates
5. ✅ `examples/brandubh_match.rs` - New example
6. ✅ `BRANDUBH.md` - New guide

## Summary

The Hnefatafl Arena now fully supports both Copenhagen Hnefatafl (11x11) and Brandubh (7x7) variants, with clean API, full backward compatibility, and comprehensive documentation. Students can choose which variant to play, and bots can adapt their strategies based on the detected variant!
