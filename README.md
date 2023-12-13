# rs-audio-engine

This all started with an experiment to see how FM synthesis works. Now it
also has a basic DirtyWave M8 inspired tracker and is being extended with
effects and samples as well.

Audio Engine will form the basic audio engine of a game project in development
All steps in making the audio engine will be extensively documented with the
issues that have been solved along the way.

The main idea is to generate the audio track based on some known patterns,
transitions and let the game state also influence the transitions and
allow effects, instruments etc.

Using a tracker would make this easy as the music is already ordered in
a way that can be controlled by an external event. Transitions can
be controlled by allowing track to progress to a certain phrase or jump to
a previous phrase to keep in the same mood.

## Packages and libraries

Audio engine is structured in many packages each of them solve a different
part of making an audio engine. Using traits the implementation is kept as
usage independent.


| Package                          | Description                                   |
|----------------------------------|-----------------------------------------------|
| `audio-engine-common`            | Common datatypes, traits for audio            |
| `audio-engine-notes`             | Notes & Scales based on music theory          |
| `audio-engine-instrument-fm`     | Instrument model for FM Syntesis              |
| `audio-engine-instrument-sample` | Instrument model for tracker sample           |
| `audio-engine-instruments`       | Instrument sound library                      |
| `audio-engine-sequencer`         | Base data types for sound and state tracking  |
| `audio-engine-tracker`           | Dirtywave M8 inspired tracker                 |


## Tracker

The library `audio-engine-tracker` implements a basic M8 inspired tracker.
It is still heavily in development and supports the next features

* 8 tracks
* Phrases and Patterns control a single track
* Note On, Note Off and Note Release are used to emulate keyboard/midi events
* Can parse a tracker formatted text to load patterns, phrases and tracks


```mermaid
classDiagram
    class Song
    class Instrument
    class Track
    class Phrase
    class Pattern
    class Row
    class RowEvent {
        NoteOn,
        NoteOff,
        NoteRelease,
        PatternEnd,
    }

    Song *--> Instrument: instruments
    Song *--> Pattern: patterns
    Song *--> Phrase: phrases
    Song *--> Track: tracks
    Track o..> Phrase: phrases
    Phrase o..> Pattern: patterns
    Pattern *--> Row: rows
    Row *--> RowEvent: event
    RowEvent o..> Instrument: instrument
```


**TODOs**

* [ ] Requires to render the song to a audio buffer, before playback in a separate step.
  Eventually this should use streaming.
* [ ] Would use the sequencer for mixing audio, global effects, polyphony, and a common
  interface for the player.

## Instruments

The definition of an instrument is kept abstract. Anything that can produce
sound at a certain pitch, supports note on and note off time events can be
used as an instrument.

The different instruments are organized in instrument models. At this time
we have an instrument model for FM synthesis and there is a tracker/sample
based instrument in the making.

```mermaid
classDiagram
    class Instrument
    class FMInstrument
    class Sample

    Instrument <|-- Sample
    Instrument <|-- FMInstrument
```

To keep track of the 'note state' of an instrument each instrument model
also has an note state structure that can store the some data to continue
when the next sample is requested. For simplicity these note states are
not added to the class diagrams on this page.


### FM Synthesis

The experiment renders a single note to the default audio of your computer.
Due to that this is an experiment it assumes that the audio device handles
float based samples by default. This might not be the case which would
crash.

```
cargo run --example experiment
```

> See `audio-engine-instrument-fm` folder

The experiment uses the next logical structure.

```mermaid
classDiagram

    class Instrument {
        sample()
    }
    class Operators
    class Operator {
        level: Level
        rate: f32

        sample()
        modulate()
    }
    class Envelope {
        delay: Time
        attack: Time
        hold: Time
        decay: Time
        sustain: Level
        release: Time

        level()
    }
    class Algorithm {
        A
        BModulatesA

        sample()
    }
    class Waveform {
        Sine
        Block
        Saw

        sample()
    }

    Operators *--> Operator: a
    Operators *--> Operator: b
    Operators *--> Operator: c
    Operators *--> Operator: d
    Operator *--> Envelope: envelope
    Instrument *--> Operators: operators
    Instrument *--> Algorithm: algorithm
    Operator *--> Waveform: waveform

```

### Sample Instrument

Next to the FM instrument model there is also a sample instrument model in
the making that inspired by s3m instrument samples. Other instrument models
can be added later as well.

### Instrument library

There is also an instrument library (`audio-engine-instruments`) that collects
instruments created in a sound library that can be easily loaded into as a
tracker instrument.

## Effects

Effects are components that can change an audio signal. Being this abstract
allows us to add effects in any level desired. Most likely some instrument
models have their own effects and each track can have its own set of effects

Effects will be organized in their own package for simplicity.
For performance reasons effects will work on buffers.
There is a special ring buffer data type that is often used in audio.

Some type of effects are:

* Delay
* Chorus
* Phaser
* Distortion


## Future Ideas

* Add effects like delay, chorus, phaser, distortion
* Add sound to fm instrument converter