# rs-audio-engine

An experiment to see how FM synthesis works.

The experiment renders a single note to the default audio of your computer.
Due to that this is an experiment it assumes that the audio device handles
float based samples by default. This might not be the case which would
crash.

```
cargo run --example experiment
```

## FM Synthesis

The experiment uses the next structure.

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