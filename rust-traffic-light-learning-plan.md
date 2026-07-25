# Learning Rust Through a Traffic Light Game

A staged plan for building a traffic-light simulation game in Rust, structured so each stage teaches a specific set of language concepts before you need them. You're currently around chapter 4 (slices) of *The Rust Programming Language* — this plan assumes that starting point and layers on top of it.

**How to use this:** Work through the stages in order. Each one names the Rust concepts it's meant to teach, gives you a concrete goal, and suggests a few questions to research yourself (rather than answers) so the learning sticks. Don't skip ahead — later stages assume you're comfortable with what came before. Resist the urge to over-engineer early stages; the point of each one is the concept it teaches, not making it feature-complete.

---

## Stage 0: Setup and Orientation

**Goal:** Get a blank window open and drawing a single rectangle on screen.

**Concepts:** Cargo project structure, dependencies, the game loop pattern (update → draw, repeat).

**What to do:**
- Create a new Cargo project (`cargo new traffic_lights`).
- Add `macroquad` as a dependency — it's the simplest 2D library for this, no complex setup.
- Get the classic macroquad skeleton running: an async `main`, a loop, `next_frame().await`.
- Draw one colored rectangle at a fixed position.

**Questions to research:**
- Why does macroquad's `main` need to be `async`, and what does `next_frame().await` actually do each loop iteration?
- What's the difference between `Cargo.toml` dependencies and the standard library — where does macroquad's code actually live on your machine?

**Done when:** You have a window that opens, shows a colored square, and closes cleanly.

---

## Stage 1: Modeling a Single Traffic Light

**Goal:** A single traffic light that cycles Red → Green → Yellow → Red on a timer, drawn on screen with the correct color.

**Concepts:** Enums, `match`, structs, mutability, basic control flow.

**What to do:**
- Define an enum for the light's state (Red, Yellow, Green).
- Write a function that, given the current state, returns the next state.
- Store elapsed time and use it to trigger transitions (e.g. 3 seconds per state).
- Use `match` to pick the draw color based on state.

**Questions to research:**
- Why is an enum a better fit here than three booleans or an integer code?
- What does it mean for `match` to be "exhaustive," and why does the compiler care?
- Where should the timer live — inside the enum, or alongside it in a struct?

**Done when:** One light cycles through its states correctly and indefinitely, and you can explain why you structured the state/timer relationship the way you did.

**Stretch (optional, only if you want extra practice):** Make the durations different per color and store them as struct fields instead of hardcoded numbers.

---

## Stage 2: One Intersection, Two Lights, Coordinated

**Goal:** Two lights at a right-angle intersection that are never both green at once.

**Concepts:** Ownership basics, structuring related data, avoiding duplicated logic, `impl` blocks.

**What to do:**
- Decide how to represent "an intersection" — likely a struct owning two (or more) lights.
- Write the coordination rule as a method on that struct rather than free-floating logic.
- Think about whether each light owns its own timer or whether the intersection drives a single shared timer.

**Questions to research:**
- What's the actual difference between putting logic in a free function vs. an `impl` method — when does each make more sense?
- If two lights both need to know "whose turn it is," who should own that fact?

**Done when:** You can watch the two lights and verify by eye that the safety rule (never both green) always holds, and you can explain in your own words why your data layout makes that rule easy (or hard) to enforce.

**This is a good checkpoint to pause and reflect:** did modeling this as a struct feel natural, or did you fight the borrow checker? That friction is useful signal — it usually means the data relationships aren't quite modeled the way Rust wants them.

---

## Stage 3: Multiple Independent Intersections

**Goal:** Several intersections on screen at once, each running its own cycle independently.

**Concepts:** `Vec<T>`, iterating and mutating collections, ownership of collections, borrowing in loops.

**What to do:**
- Store your intersections in a `Vec`.
- Update all of them each frame, then draw all of them each frame.
- Give each one a different position on screen and a slightly different timing offset so they don't all change in lockstep (this will surface some ownership questions worth sitting with).

**Questions to research:**
- Why can't you always just do `for light in &lights { light.update() }` if `update` needs `&mut self`?
- What's the practical difference between iterating with `.iter()`, `.iter_mut()`, and `into_iter()` — and why does Rust force you to be explicit about this?

**Done when:** You have at least 3 intersections running independently on screen, updated via a loop over a `Vec`, with no logic duplicated per-intersection.

---

## Stage 4: Cars That Queue and React

**Goal:** Cars spawn periodically, move toward an intersection, and stop if the light is red.

**Concepts:** More complex structs, `Vec` of entities with removal, basic collision/distance logic, working with `Option`.

**What to do:**
- Define a `Car` struct with position, speed, and maybe a target intersection.
- Spawn cars on a timer, add them to a `Vec<Car>`.
- Each frame, move cars forward unless they're too close to a red light or another car ahead of them.
- Remove cars once they've passed through and left the screen.

**Questions to research:**
- What's the cleanest way to remove items from a `Vec` while iterating, without fighting the borrow checker? (This is a genuinely common Rust stumbling block — worth understanding properly rather than finding a workaround.)
- Where does `Option<T>` naturally show up here — e.g. "the car ahead of me," which might not exist?

**Done when:** Cars visibly stop at red lights, queue up behind each other, and move again on green, without ever overlapping.

---

## Stage 5: Player Control and Win/Lose Conditions

**Goal:** Turn the simulation into an actual game — the player controls the lights (manually or by setting timing), and there's a way to "lose" (e.g. gridlock, a crash, or a queue that grows too long).

**Concepts:** Input handling, simple game state (menu/playing/game-over), maybe a basic scoring system.

**What to do:**
- Let the player click a light to force it to change, or adjust its timing.
- Track something the player is trying to optimize (throughput, or avoiding queue overflow).
- Add a simple game-over condition and a way to restart.

**Questions to research:**
- How do you cleanly represent "what screen/mode the game is in" (menu vs. playing vs. game over) — another enum, most likely. Does the same `match`-based pattern from Stage 1 scale to this?

**Done when:** Your kids can sit down, understand the goal in about 10 seconds, and play without you explaining Rust to them.

---

## Stage 6 (Optional): Polish Pass

Only worth doing once the game is actually fun. Ideas, roughly in order of effort:
- Sound effects on light changes or crashes.
- A simple UI showing score/time survived.
- Save the best score between runs (this is a good excuse to touch basic file I/O in Rust).
- Multiple intersection layouts or difficulty levels.

---

## General notes on approach

- **Don't reach for `Rc<RefCell<>>`, threads, or `unsafe` for any of this.** If you find yourself wanting them, it's almost always a sign the data modeling needs rethinking, not that you need a more advanced Rust tool. Struggling through that rethink is where a lot of the real learning is.
- **When the borrow checker fights you, stop and ask why**, before looking for a workaround. Nine times out of ten it's telling you something true about who should own what — that's the whole point of using Rust for this instead of a language that would let you paper over it.
- **Keep a running list of things you had to look up.** After a few stages, patterns in that list will show you what to deliberately study next (it might be error handling, might be traits, might be lifetimes).
