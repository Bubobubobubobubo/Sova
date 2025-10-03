# BaLi

BaLi is a small Lisp-inspired language for live-coding in Sova. It provides timing primitives, MIDI/OSC output, and declarative rhythm patterns.

## Your First Steps

### Playing a Note

The simplest BaLi program plays a note:

```lisp
(note c)
```

This plays middle C (c3) immediately. Notes use musical notation: `c`, `eb`, `f#7`, etc.

### Timing

Use `>` to shift forward in time (in beats):

```lisp
(> 1 (note c))      ; Play C after 1 beat
(> 0.5 (note e))    ; Play E after half a beat
```

Multiple statements execute in parallel:

```lisp
(> 0 (note c))
(> 1 (note e))
(> 2 (note g))
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

Or create euclidean rhythms with `eucloop`:

```lisp
(eucloop 5 8 1 (note c))    ; 5 hits distributed across 8 steps
```

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

### Variables

Use local variables or globals (A-Z):

```lisp
(def X 60)
(note X)              ; Play note 60

(loop 4 1
  (note A)
  (def A (+ A 2)))    ; Increment A each iteration
```

Environment variables: `T` (tempo in BPM), `R` (random 0-127).

### Arithmetic

```lisp
(note (+ c 7))        ; c + 7 semitones
(note (* 2 c))        ; double the note number
(note (rand 60 72))   ; random note between 60 and 72
```

### Conditionals and Control

```lisp
(if (gt R 64)
  (note c)
  (note g))

(pick (% R 3)         ; Pick based on R mod 3
  (note c)
  (note e)
  (note g))
```

### Functions

Define reusable functions:

```lisp
(fun transpose note offset
  (+ note offset))

(note (transpose c 7))
```

## Quick Reference

**Timing:**
- `(> time ...)` - shift forward
- `(< time ...)` - shift backward
- `(spread time ...)` - distribute evenly
- `(loop n time ...)` - repeat n times
- `(eucloop steps beats time ...)` - euclidean rhythm
- `(binloop val beats time ...)` - binary rhythm

**Effects:**
- `(note n)` - MIDI note
- `(control cc val)` - MIDI CC
- `(prog n)` - program change
- `(osc "/addr" ...)` - OSC message
- `(dirt "sound" ...)` - SuperDirt

**Control:**
- `(if cond ...)` - conditional
- `(pick n ...)` - select nth element
- `(? n ...)` - pick n random elements
- `(alt ...)` - alternate on each run
- `(with context ...)` - set context

**Context:** `ch:`, `v:`, `dur:`, `dev:`

> Full specification: [The Grammar of BaLi](/docs/research/bali-syntax)
