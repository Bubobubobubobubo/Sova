# BaLi

BaLi is a small language whose syntax is inspired by LISP (**Ba**sic-**Li**sp). This language has been designed and implemented by Loïg Jezequel. It is a mostly declarative language focused on rhythm and timing musical events, with combinatorial constructs to create complex patterns from simple building blocks. BaLi is the default language in Sova and also the first that have been implemented.

## Data Types

The BaLi language is using four basic types:

| Type | Description | Examples |
|------|-------------|----------|
| **Numbers** | Integers | `60`, `-5`, `127` |
| **Decimals** | Floating point values | `0.5`, `1.25`, `-2.3` |
| **Strings** | Text in quotes | `"kick"`, `"/synth/freq"` |
| **Notes** | Musical notes | **Simple:** `c`, `d`, `e`, `f`, `g`, `a`, `b`<br>**With octaves:** `c3`, `f#4`, `bb-1`, `g8`<br>**Sharps:** `c#`, `f#3`, `c3#`, `f3#`<br>**Flats:** `db`, `bb4`, `d4b`, `b4b`<br>**Default:** `c` = `c3` = MIDI 60<br>**Range:** `c-2` (MIDI 0) to `g8` (MIDI 127) |

## Variables

There are global variables that persist across evaluations, and local variables that are reset each time the code is run. Global variables are easy to spot as they are uppercase, and limited in number. Local variables are lowercase and can be named freely (as long as they don't shadow built-in function names).

**Global Variables** - Persist across evaluations: `A`, `B`, `C`, `D`, `W`, `X`, `Y`, `Z`

**Local Variables** - Any lowercase name: `counter`, `pitch`, `my-value`

**Environment Variables:**
- `T` - Current tempo in BPM
- `R` - Random value 0-127 (new value for each access)

Define and use variables:

```lisp
(def X 60)
(note X)

(def my-pitch c4)
(note my-pitch)
```

## Your First Steps

### Playing a Note

The simplest BaLi program plays a MIDI note (middle C3, with default velocity on channel 1):

```lisp
(note c)
```

### Timing

Use `>` to shift forward in time (in beats):

```lisp
(> 1 (note c))      ; Play C after 1 beat
(> 0.5 (note e))    ; Play E after half a beat
(> (// 1 3) (note g))  ; Play G after 1/3 beat
```

Multiple statements execute in parallel:

```lisp
(> 0 (note c))
(> 1 (note e))
(> 2 (note g))
```

Shift backward with `<`:

```lisp
(< 1 (note c))      ; Play C 1 beat before current time
```

### Frame-Based Timing

For sample-accurate timing, use `:f` suffix:

```lisp
(> 100:f (note c))   ; 100 frames from now
```

### Spreading Events

Use `spread` to distribute events evenly:

```lisp
(spread 1
  (note c)
  (note e)
  (note g))
```

This plays c, e, g evenly across 1 beat (at 0, 1/3, 2/3).

### Loops

Repeat patterns with `loop`:

```lisp
(loop 4 1 (note c))     ; Play C 4 times in 1 beat
```

Create euclidean rhythms with `eucloop`:

```lisp
(eucloop 5 8 1 (note c))    ; 5 hits distributed across 8 steps in 1 beat
```

Binary rhythms with `binloop`:

```lisp
(binloop 13 8 1 (note c))   ; Binary pattern 00001101 (13 in binary)
```

### Loop Modifiers

Add modifiers after the timing argument:

```lisp
(loop 4 1 :rev (note c))           ; Reversed
(eucloop 3 8 1 :neg (note c))      ; Inverted pattern
(eucloop 3 8 1 sh: 2 (note c))     ; Shifted by 2 steps
(loop 4 1 :step (note c))          ; Step time mode
```

Combine modifiers:

```lisp
(eucloop 5 8 1 :rev sh: 3 (note c))
```

### Ramps

Gradually change a variable over time:

```lisp
(ramp X 4 60 72 "linear" 2 (note X))   ; Ramp X from 60 to 72 over 2 beats (4 steps)
```

Parameters: `(ramp variable granularity min max distribution time ...)`

### Context

Set MIDI channel, velocity, duration, or device using context:

```lisp
(> 0 ch: 1 v: 100 dur: 0.5
  (note c))
```

Context propagates to nested statements:

```lisp
(spread 1 ch: 1 v: 100
  (note c)
  (note e)
  (note g))
```

**Context Parameters:**
- `ch:` - MIDI channel (1-16)
- `v:` - Velocity (0-127)
- `dur:` - Duration in beats
- `dev:` - Device ID

Use `(with context ...)` to set context for a block:

```lisp
(with ch: 2 v: 80
  (note c)
  (note e))
```

## Effects

### MIDI Effects

**Notes:**

```lisp
(note c)              ; Play C3
(note 60)             ; Play MIDI note 60
(note (+ c 7))        ; Play C + 7 semitones
```

**Program Change:**

```lisp
(prog 0)              ; Change to program 0
(prog 32 ch: 2)       ; Change program on channel 2
```

**Control Change:**

```lisp
(control 7 100)       ; Set CC 7 (volume) to 100
(control 74 64 ch: 3) ; Set CC 74 on channel 3
```

**Aftertouch (Polyphonic):**

```lisp
(at c 100)            ; Aftertouch for note C
(at 60 100 ch: 2)     ; Aftertouch on channel 2
```

**Channel Pressure (Monophonic Aftertouch):**

```lisp
(chanpress 100)       ; Channel pressure value 100
(chanpress 64 ch: 3)  ; Channel pressure on channel 3
```

### OSC Messages

Send Open Sound Control messages:

```lisp
(osc "/synth/freq" 440)
(osc "/synth/freq" 440.5 dev: 2)
(osc "/trigger" "kick" 1.0)
```

### SuperDirt

BaLi provides a built-in quick access to SuperCollider's SuperDirt, known and loved by many live coders for its simplicity and robustness.

Trigger SuperDirt sounds:

```lisp
(dirt "bd" :n 0)
(dirt "cp" :n 2 :gain 0.8)
(dirt "arpy" :n 5 :speed 1.5 :room 0.3)
```

### Audio Engine

Play sounds through Sova's audio engine:

```lisp
(sound "kick" :gain 0.9)
(sound "snare" :pan 0.5)
```

Play samples:

```lisp
(sample :file "kick.wav" :gain 0.8)
(sp :file "snare.wav")     ; Short alias
```

## Expressions

### Arithmetic

```lisp
(+ a b)               ; Addition
(* a b)               ; Multiplication
(- a b)               ; Subtraction
(/ a b)               ; Division
(% a b)               ; Modulo
```

Examples:

```lisp
(note (+ c 7))        ; c + 7 semitones
(note (* 2 30))       ; 60
(note (% R 12))       ; R modulo 12
```

### Math Utilities

```lisp
(min a b)             ; Minimum of two values
(max a b)             ; Maximum of two values
(rand max)            ; Random number 0 to max
(rand min max)        ; Random number min to max
(scale val old_min old_max new_min new_max)  ; Scale value between ranges
(clamp val min max)   ; Clamp value to range
(quantize val step)   ; Quantize to step size
```

Examples:

```lisp
(note (rand 60 72))
(note (scale R 0 127 60 72))
(note (clamp (+ c R) 0 127))
(note (quantize (/ R 10) 1))
```

### Oscillators and LFOs

Generate modulation signals (speed in Hz):

```lisp
(sine speed)          ; Sine wave
(saw speed)           ; Sawtooth wave
(triangle speed)      ; Triangle wave
(isaw speed)          ; Inverted sawtooth
(randstep speed)      ; Random step function
```

Examples:

```lisp
(note (+ 60 (* 12 (sine 0.5))))           ; Pitch LFO
(note c v: (* 64 (+ 1 (triangle 2))))     ; Velocity LFO
```

### MIDI Input

Read MIDI CC values from your controller:

```lisp
(ccin 1)              ; Read CC 1 from context device/channel
(ccin 7 dev: 1 ch: 1) ; Read CC 7 from specific device/channel
```

Use CC values:

```lisp
(note c v: (ccin 1))
(note (+ 60 (ccin 16)))
```

## Boolean Expressions

Use in `if` and `for`:

```lisp
(and b1 b2)           ; Logical AND
(or b1 b2)            ; Logical OR
(not b)               ; Logical NOT
(lt a b)              ; Less than
(leq a b)             ; Less or equal
(gt a b)              ; Greater than
(geq a b)             ; Greater or equal
(== a b)              ; Equal
(!= a b)              ; Not equal
```

## Control Flow

### Conditionals

```lisp
(if (gt R 64)
  (note c)
  (note g))
```

### For Loops

Execute while condition is true:

```lisp
(for (lt X 72)
  (note X)
  (def X (+ X 1)))
```

### Pick

Select specific elements (0-indexed):

```lisp
(pick 0               ; Pick first
  (note c)
  (note e)
  (note g))

(pick (% R 3)         ; Pick based on R mod 3
  (note c)
  (note e)
  (note g))
```

### Random Choice

Pick n random elements:

```lisp
(? 1                  ; Pick 1 random element
  (note c)
  (note e)
  (note g))

(? 2                  ; Pick 2 random elements
  (note c)
  (note e)
  (note g))
```

### Alternation

Alternate through options on each evaluation:

```lisp
(alt
  (note c)
  (note e)
  (note g))
```

First run plays c, second run plays e, third plays g, then repeats.

### Sequencing

Group effects together:

```lisp
(seq
  (note c)
  (note e)
  (note g))
```

### Execution Order

```lisp
(>> (note c))         ; Execute after everything else
(<< (note c))         ; Execute before everything else
```

## Advanced Arguments

### Lists

Cycle through values:

```lisp
(loop 4 1
  (note [c e g b]))   ; Plays c, e, g, b in sequence
```

### Alternation

Alternate on each run:

```lisp
(loop 4 1
  (note <c e g>))     ; First run all c, second run all e, etc.
```

### Random Choice

Pick randomly:

```lisp
(loop 4 1
  (note {c e g}))     ; Each note randomly chosen
```

### Repetition

Repeat an argument n times:

```lisp
(spread 1
  (note c !3)         ; Equivalent to (note c) (note c) (note c)
  (note g))
```

## Functions

Define reusable functions:

```lisp
(fun transpose note offset
  (+ note offset))

(note (transpose c 7))
```

Functions can use any expressions:

```lisp
(fun random-note min max
  (rand min max))

(loop 4 1
  (note (random-note 60 72)))
```

## Complete Reference

### Timing
- `(> time ...)` - shift forward
- `(< time ...)` - shift backward
- `(spread time ...)` - distribute evenly
- `(loop n time ...)` - repeat n times
- `(eucloop steps beats time ...)` - euclidean rhythm
- `(binloop val steps time ...)` - binary rhythm
- `(ramp var gran min max dist time ...)` - ramp over time

### Loop Modifiers
- `:neg` - negate/invert pattern
- `:rev` - reverse pattern
- `sh: n` - shift by n steps
- `:step` - step time mode

### MIDI Effects
- `(note n)` - MIDI note
- `(prog n)` - program change
- `(control cc val)` - MIDI CC
- `(at note val)` - polyphonic aftertouch
- `(chanpress val)` - channel pressure

### OSC/Audio
- `(osc "/addr" ...)` - OSC message
- `(dirt "sound" ...)` - SuperDirt
- `(sound "source" ...)` - Audio engine
- `(sample ...)` - Sample playback
- `(sp ...)` - Sample playback (short)

### Arithmetic
- `(+ a b)`, `(- a b)`, `(* a b)`, `(/ a b)`, `(% a b)`
- `(min a b)`, `(max a b)`

### Math
- `(rand max)`, `(rand min max)` - random
- `(scale val old_min old_max new_min new_max)` - scale
- `(clamp val min max)` - clamp
- `(quantize val step)` - quantize

### Oscillators
- `(sine speed)`, `(saw speed)`, `(triangle speed)`
- `(isaw speed)`, `(randstep speed)`

### MIDI Input
- `(ccin ctrl)` - read MIDI CC
- `(ccin ctrl dev: n ch: m)` - read from specific device/channel

### Boolean
- `(and b1 b2)`, `(or b1 b2)`, `(not b)`
- `(lt a b)`, `(leq a b)`, `(gt a b)`, `(geq a b)`
- `(== a b)`, `(!= a b)`

### Control Flow
- `(if cond ...)` - conditional
- `(for cond ...)` - while loop
- `(pick n ...)` - select nth element
- `(? n ...)` - pick n random elements
- `(alt ...)` - alternate on each run
- `(seq ...)` - sequence effects
- `(with context ...)` - set context
- `(>> ...)` - execute after
- `(<< ...)` - execute before

### Context
- `ch:` - MIDI channel
- `v:` - velocity
- `dur:` - duration
- `dev:` - device ID

### Advanced Arguments
- `[a b c]` - list (cycle)
- `<a b c>` - alternation
- `{a b c}` - random choice
- `arg !n` - repeat n times

### Functions
- `(fun name args... expr)` - define function
- `(def var expr)` - define variable
- `()` - no-op

> Full specification: [The Grammar of BaLi](/docs/research/bali-syntax)
