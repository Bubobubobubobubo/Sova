[![Crates.io](https://img.shields.io/crates/v/sova.svg)](https://crates.io/crates/sova) [![Documentation](https://docs.rs/sova/badge.svg)](https://docs.rs/sova) [![Build Status](https://github.com/username/sova/workflows/Rust/badge.svg)](https://github.com/Bubobubobubobubo/sova/actions)

# Sova: A Polyglot Sequencer & Virtual Machine for Live Coding

Sova is a brand new sequencer and musical programming environment designed in Rust. It is based on a robust virtual machine that allows to improvise music in real time through code. The virtual machine allows Sova to support multiple bespoke programming languages, each tailored to a specific way to think about music. **Sova is free and open-source software**, developed for artists, developers and researchers alike. It aims to help popularize a creative and musical approach to computer programming based on musical improvisation.

* **Virtual machine**: Sova is built on a robust virtual machine that specializes in executing code precisely in realtime for musical purposes. The virtual machine is tightly coupled with a realtime scheduler -- based on [Ableton Link](https://developer.ableton.com/link/). Code execution is temporized, offering very strong guarantees of accuracy.


* **Polyglocy**: Sova can support multiple programming languages, both compiled and interpreted. You can build bespoke languages for Sova or adapt your favorite live coding library to the Sova virtual machine. Each language can provide a different way to think about musical expression or a different way to describe musical sequences and musical objects.
* **Sequencer Session**: Sova code execution works following a timeline-like paradigm. It allows you to order and sequence multiple scripts both sequentialy or in parallel. It is possible to execute multiple scripts written in different languages in the same session. 
  * _Composers_: use the timeline, this is meant to help you to compose and lay down musical pieces through improvisation.
  * _Old-school live coders_: ignore the timeline if you wish, or use it to your advantage :)
* **Server/Client Architecture:** Sova is designed to support multiple clients in the same session. By default, Sova is ready for collaborative jamming. Anybody can start a session that you can join on your local network.
* **Modular**: Sova is not a monolithic piece of software. You can think of it as multiple objects forming a coherent framework or system. There is the virtual machine, the server, the various clients, etc. Take the bits you like, leave the other stuff behind.
* **I/O**: Sova is capable of emitting and receiving MIDI and OSC messages. Thanks to its modular design, it is relatively easy to add support for other protocols if the need arises.

## Project status

Sova is a young project, still to be considered in alpha stage. The virtual machine is functional and already really cool. It already supports multiple programming languages, both compiled and interpreted. Clients are still under development but more than usable! Documentation and more bespoke programming languages are already on the way. You can start hacking freely, the code is stable!

## Build

We do not yet offer released versions. You can build Sova from source by following the instructions in the [CONTRIBUTING.md](/CONTRIBUTING.md) file. This is mostly a matter of cloning the repository and running `cargo`. Some dependencies may require additional setup, such as installing Rust, Cargo, pnpm, and NodeJS.

## License

Sova is distributed under the [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license. A copy of the license is distributed with the software: [LICENSE](/LICENSE).

## Acknowledgments

The research and design work on Sova was supported by many institutions, including:

- The [LS2N Laboratory](https://www.ls2n.fr) and the [University of Nantes](https://www.univ-nantes.fr)
- [Athenor CNCM](https://www.athenor.com) and its director, Camel Zekri.

Many thanks for their continuous support and collaboration all along the last years.
