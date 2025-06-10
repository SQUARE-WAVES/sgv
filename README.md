# sgv

# what is this?

I wanted to play around with parsing languages and stuff, so this is the project I made.

this is a little program for doing music stuff. If you've ever seen "live coding" performances
they typically use stuff like [supercollider](https://supercollider.github.io/) or [tidal cycles](https://tidalcycles.org/)
to make sounds and do the sequencing all on the computer I like to use external hardware, that's what this thing is for.


# how does it work?

pretty minimally at this point, you basically have a repl that you can program little "sequences" with and you can assign them
to play out of midi outputs. It's quite limited in what you can do, just send note-ons and note-offs according to patterns

the language looks kidna like this
```
X = (C_1 127)
x = (C_1 64)
C = (F#1 127)
c = (F#1 64)

oontz = [X - C - ]
4on_the_floor = [oontz oontz oontz oontz]

<0> = 4on_the_floor
play 0
```

you set variables to either "triggers" like `(C_1 127)` which triggers a note
then you make "bars" and assign them to slots. you can then play or stop slots or other things

there is also code for unsing a novation launchpad to start and stop slots.

# plans for more?

Yeah I might re-do the language at some point, I also intend to add randomizing and/or generat stuff 
like triggers that can play from sets of notes
or sub bars.
