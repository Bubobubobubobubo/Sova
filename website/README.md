# Sova: a polyglot live coding environment

### What is Sova?

_Sova_ is a music creation software designed as part of a research project supported by [Athénor CNCM](https://www.athenor.com/) in Saint-Nazaire and by the [LS2N laboratory](https://www.ls2n.fr/) at the University of Nantes. This software is freely available and open source licensed. It is developed by a small team of developers and volunteer contributors. Sova is a multifaceted software. It can be described as both a creative programming environment and a musical sequencer. It is a tool for artistic experimentation, designed as a runtime for various musical programming languages specialized in music performance. Sova is built for [live coding](https://livecoding.fr) performances. Our goal is to develop a tool that will encourage musicians to develop a performative and expressive approach to computer programming. Sova encourages its user to perceive the computer as a musical instrument: a technical, creative and poetic object. Sova seeks to offer an immediate, playful, and embodied experience of musical programming.

### How does it work?

Sova is based on a familiar concept: the step sequencer. We have adopted this model and changed it just a bit. Our goal is not to play one event per step but rather to execute one script per step. A step can be of any duration. The script associated with a step can be of any complexity. It can generate a great amount of events or none at all, modify its own runtime environment, alter the state used by future scripts, etc.


![Nested structure of a Sova scene](assets/images/scene_demo.svg)

Sova can sequence events in a complex environment. It is capable of sending and receiving MIDI and OSC, to communicate with its own realtime audio synthesis and sampling engine, etc. Multiple sequences of _scripts_ can be played together, interrupted and/or reprogrammed on the fly! Scripts are executed with metronomic precision, relatively to a precise networked musical clock. The musician can control all aspects of the performance, from defining the environment to use to writing the scripts or altering the sequencer behaviour. All the scripts forming a playing session are available to all musicians connected to the same session.


### Who is it for?

Sova was designed to support learning programming and/or computer music. The software is therefore accessible to any beginner musician. No technical or musical prerequisites are necessary to get started. All complexity arises from the gradual mastery of the tool that the musician acquires through experimentation and play. Using Sova begins with learning the most elementary musical and technical concepts: the music theory specific to _live coding_. Learning then extends toward mastering more advanced programming/composition techniques. The most dedicated users can even modify the tool itself. They will thus possess complete mastery of the instrument and make it evolve with them. The tool is designed to be intuitive. It only gradually exposes the complexity of its operation, always at the musician's initiative.

This software will also interest more experienced musicians and artists. They will find in Sova a tool allowing precise control and synchronization of their various machines, synthesizers, sound/visual generation software. Sova is all at once:
- an extensible, _open source_, multi-language programming and prototyping environment.
- a collaborative (multi-client) and real-time musical sequencer.
- an algorithmic and reactive musical instrument.

Sova can be used to prepare complex musical performances. It can also help the musician formalize while improvising certain playing techniques and/or ways of thinking about musical writing and performance: algorithmic composition, generative stochastic, random, etc.

![First Sova sequence](assets/images/first_line.jpg)
*First musical sequence compiled with Sova (March 2025). Left: raw program, right: emitted messages.*

### What programming languages does Sova support?

Sova is designed to support different programming languages built _ad hoc_ for the software. These languages are specialized in describing musical events or sequences. Each _script_ can be programmed, as needed, using a different programming language. Some languages will naturally specialize in writing melodic-harmonic sequences, others in describing rhythms, events, or more abstract processes. The Sova server handles the transmission of these _scripts_, written in high-level languages, to an internal machine representation, close to assembly. If the communication protocol with the server is respected, scripts written in very different languages can coexist and be executed without problem on the server. Different languages can be added provided they can be compiled/interpreted into the intermediate representation used by Sova's internal event engine. At the foundation of Sova is a generic and powerful language for abstractly describing musical programs in a synchronous/imperative form.

![Client-server architecture](assets/images/test_export.svg)
*Client/server architecture, multiple script languages are interpreted into a single internal representation.*

Being able to build different languages and choose which one to use depending on the situation, device and/or project allows freely exploring different ways of programming and thinking about music. Each programming language also induces a different relationship between the musician and the instrument. Musicians can choose the abstractions most suited to their playing style, their way of working and collaborating (multi-client play). It is not necessary for developers to master the Rust language to propose new languages. The server has an interface allowing submission of a program serialized in JSON format, which will then be translated into machine language and executed by Sova.