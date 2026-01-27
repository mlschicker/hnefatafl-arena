# Tournament Guide

This guide explains how to organize bot tournaments for your students.

## Quick Start

```bash
# Build the project
cargo build --release

# Run a simple match
cargo run --release --example simple_match

# Test custom bot
cargo run --release --example custom_bot
```

## Creating a Student Bot Package

Each student should create their own bot in a separate file:

### Step 1: Create bot file

Create `examples/student_name_bot.rs`:

```rust
use hnefatafl_arena::*;
use std::time::Duration;

pub struct StudentBot {
    name: String,
}

impl StudentBot {
    pub fn new(name: String) -> Self {
        StudentBot { name }
    }
}

impl Bot for StudentBot {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn get_move(&mut self, state: &GameState, _time_limit: Duration) -> Option<Move> {
        // Student's strategy here
        let moves = state.legal_moves();
        if moves.is_empty() {
            None
        } else {
            Some(moves[0])  // Replace with real strategy
        }
    }
}

fn main() {
    let my_bot = Box::new(StudentBot::new("StudentBot".to_string()));
    let opponent = Box::new(GreedyBot::new("GreedyBot".to_string()));
    
    let config = MatchConfig::default();
    let mut match_game = Match::new(my_bot, opponent, config, true);
    match_game.play();
}
```

### Step 2: Test the bot

```bash
cargo run --release --example student_name_bot
```

## Organizing a Tournament

### Method 1: Simple Round-Robin Script

Create `tournament.sh`:

```bash
#!/bin/bash

# List of all student bots
BOTS=(
    "student1_bot"
    "student2_bot"
    "student3_bot"
    # Add more...
)

echo "Starting Hnefatafl Tournament"
echo "=============================="

# Run each bot against every other bot
for i in "${!BOTS[@]}"; do
    for j in "${!BOTS[@]}"; do
        if [ $i -lt $j ]; then
            echo ""
            echo "Match: ${BOTS[$i]} vs ${BOTS[$j]}"
            # You would need to modify the match runner to accept bot names as args
            cargo run --quiet --release --example run_match "${BOTS[$i]}" "${BOTS[$j]}"
        fi
    done
done
```

### Method 2: Rust Tournament Runner

Create `examples/tournament_runner.rs`:

```rust
use hnefatafl_arena::*;
use std::time::Duration;
use std::collections::HashMap;

// Import all student bots
mod student1_bot;
mod student2_bot;
// ... etc

fn main() {
    println!("Hnefatafl Tournament");
    println!("====================\\n");
    
    let config = MatchConfig {
        time_per_move: Duration::from_secs(5),
        max_moves: 150,
    };
    
    // Scoreboard: bot name -> (wins, draws, losses)
    let mut scoreboard: HashMap<String, (u32, u32, u32)> = HashMap::new();
    
    // List of bot constructors
    let bot_names = vec!["Student1", "Student2", "Student3"];
    
    // Play round-robin tournament
    for i in 0..bot_names.len() {
        for j in (i+1)..bot_names.len() {
            let name1 = bot_names[i];
            let name2 = bot_names[j];
            
            println!("\\n{'='*60}");
            println!("Match: {} vs {}", name1, name2);
            println!("{'='*60}");
            
            // Game 1: Bot1 as Attackers
            let bot1 = create_bot(name1);
            let bot2 = create_bot(name2);
            let result1 = play_match(bot1, bot2, &config, name1, name2);
            update_scoreboard(&mut scoreboard, name1, name2, result1);
            
            // Game 2: Bot2 as Attackers (swap sides)
            let bot1 = create_bot(name1);
            let bot2 = create_bot(name2);
            let result2 = play_match(bot2, bot1, &config, name2, name1);
            update_scoreboard(&mut scoreboard, name2, name1, result2);
        }
    }
    
    // Print final standings
    print_standings(&scoreboard);
}

fn create_bot(name: &str) -> Box<dyn Bot> {
    match name {
        "Student1" => Box::new(student1_bot::Student1Bot::new(name.to_string())),
        "Student2" => Box::new(student2_bot::Student2Bot::new(name.to_string())),
        // Add more cases...
        _ => Box::new(RandomBot::new(name.to_string())),
    }
}

fn play_match(
    attacker: Box<dyn Bot>,
    defender: Box<dyn Bot>,
    config: &MatchConfig,
    attacker_name: &str,
    defender_name: &str,
) -> MatchResult {
    let mut match_game = Match::new(
        attacker,
        defender,
        config.clone(),
        false,  // Set to true for verbose output
    );
    match_game.play()
}

fn update_scoreboard(
    scoreboard: &mut HashMap<String, (u32, u32, u32)>,
    attacker_name: &str,
    defender_name: &str,
    result: MatchResult,
) {
    let attacker_entry = scoreboard.entry(attacker_name.to_string()).or_insert((0, 0, 0));
    let defender_entry = scoreboard.entry(defender_name.to_string()).or_insert((0, 0, 0));
    
    match result {
        MatchResult::AttackersWin { .. } => {
            attacker_entry.0 += 1;  // Win
            defender_entry.2 += 1;  // Loss
        }
        MatchResult::DefendersWin { .. } => {
            defender_entry.0 += 1;  // Win
            attacker_entry.2 += 1;  // Loss
        }
        MatchResult::Draw { .. } => {
            attacker_entry.1 += 1;  // Draw
            defender_entry.1 += 1;  // Draw
        }
        MatchResult::Timeout { violator, .. } | MatchResult::IllegalMove { violator, .. } => {
            if violator == attacker_name {
                defender_entry.0 += 1;  // Win
                attacker_entry.2 += 1;  // Loss
            } else {
                attacker_entry.0 += 1;  // Win
                defender_entry.2 += 1;  // Loss
            }
        }
    }
}

fn print_standings(scoreboard: &HashMap<String, (u32, u32, u32)>) {
    println!("\\n\\n{'='*60}");
    println!("FINAL STANDINGS");
    println!("{'='*60}");
    println!("{:<20} {:>8} {:>8} {:>8} {:>8}", "Bot", "Wins", "Draws", "Losses", "Points");
    println!("{}", "-".repeat(60));
    
    let mut standings: Vec<_> = scoreboard.iter().collect();
    standings.sort_by_key(|(_, (w, d, _))| std::cmp::Reverse((w * 3 + d, *w)));
    
    for (name, (wins, draws, losses)) in standings {
        let points = wins * 3 + draws;
        println!("{:<20} {:>8} {:>8} {:>8} {:>8}", name, wins, draws, losses, points);
    }
    println!("{'='*60}");
}
```

## Scoring Systems

### Simple Win/Loss
- Win: 1 point
- Loss: 0 points

### Chess-style
- Win: 3 points
- Draw: 1 point
- Loss: 0 points

### ELO Rating
For more sophisticated tournaments, implement ELO rating system.

## Tournament Rules

### Recommended Settings

```rust
MatchConfig {
    time_per_move: Duration::from_secs(5),  // 5 seconds per move
    max_moves: 200,  // Max 200 moves before draw
}
```

### Fair Play
1. Each pair of bots plays twice (once as attacker, once as defender)
2. Time limits must be enforced
3. Illegal moves result in immediate loss
4. Bots should not use randomness for reproducibility (or seed it)

### Tiebreakers
1. Total points
2. Number of wins
3. Head-to-head record
4. Average moves to win

## Collecting Student Bots

### Option 1: Separate Files
Students submit individual bot files that you add to `examples/`.

### Option 2: Git Branches
Each student works on their own branch and you merge for tournament.

### Option 3: Modules
Create `bots/` directory:

```
bots/
├── mod.rs
├── student1.rs
├── student2.rs
└── student3.rs
```

In `mod.rs`:
```rust
pub mod student1;
pub mod student2;
pub mod student3;
```

## Testing Individual Bots

Create `examples/test_bot.rs`:

```rust
use hnefatafl_arena::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example test_bot <bot_name>");
        return;
    }
    
    let bot_name = &args[1];
    
    // Test against standard opponents
    let opponents = vec![
        Box::new(RandomBot::new("Random".to_string())),
        Box::new(GreedyBot::new("Greedy".to_string())),
    ];
    
    for opponent in opponents {
        let bot = create_bot(bot_name);
        let config = MatchConfig::default();
        let mut match_game = Match::new(bot, opponent, config, true);
        match_game.play();
    }
}
```

## Debugging Tips

### Enable Verbose Output
```rust
let mut match_game = Match::new(bot1, bot2, config, true);  // true = verbose
```

### Print Board State
```rust
println!("{}", state.display_board());
```

### Track Move History
Bots can use `notify_move` to track all moves.

### Time Profiling
Check move times in verbose output to identify slow bots.

## Sample Tournament Schedule

For 8 students (28 matches total):

```
Round 1: Student1 vs Student2, Student3 vs Student4, ...
Round 2: Student1 vs Student3, Student2 vs Student5, ...
...
Final:   Top 2 from round-robin
```

## Results Format

Generate results as:
- CSV file for spreadsheets
- Markdown table for documentation
- JSON for web display
- HTML page with interactive board replay

## Example Results Table

```markdown
| Rank | Bot Name | Wins | Draws | Losses | Points |
|------|----------|------|-------|--------|--------|
| 1    | Alice    | 12   | 2     | 0      | 38     |
| 2    | Bob      | 10   | 4     | 0      | 34     |
| 3    | Charlie  | 8    | 3     | 3      | 27     |
```

## Advanced: Parallel Execution

For large tournaments, run matches in parallel:

```rust
use rayon::prelude::*;

let results: Vec<_> = matches.par_iter()
    .map(|(bot1, bot2)| play_match(bot1, bot2))
    .collect();
```

(Requires adding `rayon` dependency)

## Questions?

See [README.md](README.md) for API documentation and [API_REFERENCE.md](API_REFERENCE.md) for quick reference.
