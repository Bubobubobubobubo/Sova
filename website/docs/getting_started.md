## About the Sova project

<div style="display: flex; gap: 2rem; align-items: flex-start; flex-wrap: wrap;">
  <div style="flex: 0 0 auto;">
    <img src="./assets/diagrams/sova_architecture.svg" width=100%, style="max-width: 100%;">
  </div>
  <div style="flex: 1; min-width: 300px;">
    <p>Sova is a software environment for collaborative musical live coding. It is composed of four software components. Each of them can be installed and used independently in your projects. Nonetheless, everything is designed to work as a cohesive whole forming the Sova playground. The goal of this documentation is to teach you what Sova is, how it works, and what you can expect from it. It is both a technical tour and an explanation of the philosophy guiding us through the development of the tool.</p>
  </div>
</div>

If you only want to play some music, you will be mostly interested by the [installation section](/docs/installation) and by the description of the [graphical user interface](/docs/gui/gui). We encourage you to refer to more in-hands tutorials and examples to learn how to play and improvise!


## Architecture

| Component | Purpose | Key Features |
|-----------|---------|--------------|
| [Server]() |  Communication |• Receive/send messages from users, orchestrates the session<br>• Communicates with other components (engine, core, GUI)|
| [Core](docs/core/core) | Core | • Host compilers/interpreters for live coding languages<br>• Manage MIDI/OSC, audio I/O and world interaction<br>• Synchronize musicians via [Ableton Link](https://ableton.github.io/link/) protocol<br>• Manage the shared _scene_ state (_the jam session_) |
| [GUI](docs/gui/gui) | Graphical interface | • Main user interface: spawn / connect to a server<br>• Code editor (with highlighting, error reporting, etc)<br>• Save / Load / Edit scene snapshots and sessions<br>• Collaborative real-time jamming through code! |
| [Engine](docs/engine/engine) | Audio engine | • Audio synthesis, sampling, effects, etc<br>• Portable, lightweight and robust<br>• Easy to extend with new synthesis modules|
