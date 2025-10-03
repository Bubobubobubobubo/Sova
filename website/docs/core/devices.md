# Devices

### How are devices managed in Sova?

Sova interacts with the outside world through various input and output devices. These devices can be broadly categorized into two types: MIDI and OSC. These are the most common protocols used in digital music and multimedia applications. The interface for connecting and managing these devices is called the `DeviceMap`. It is quite generic and flexible by design, making it easy for all accessible languages to tap into all the input and output devices available during a session.

### DeviceMap

Sova configures input and output devices through the `DeviceMap`. It is a collection of `Device` objects, each representing a specific input or output device. Each input and each output is given a slot number, which can be used to reference the device in code. By default, your computer might have several MIDI and OSC devices available, depending on the software and hardware you have installed and/or connected. Sova automatically detects these devices and populates the `DeviceMap` accordingly so you can try to connect to them right away.

### Virtual devices

> Windows users: Virtual MIDI ports are not natively supported on Windows. You will need to install a third-party application such as [loopMIDI](https://www.tobias-erichsen.de/software/loopmidi.html) to create virtual MIDI ports. Sorry for this inconvenience, blame Microsoft!

Sova is capable of creating virtual MIDI and OSC devices, which can be used to route data between different applications on your computer. A MIDI port named `Sova` will always be created when starting Sova. You are free to use it or to delete it if you don't need it. You can also create additional virtual devices as needed.
