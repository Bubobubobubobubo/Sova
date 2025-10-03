# OSC

> OSC (_Open Sound Control_) is a protocol for communication among computers, sound synthesizers, and other multimedia devices that is optimized for modern networking technology. OSC messages are sent over UDP or TCP, and can contain arbitrary data types. OSC is very popular in the computer music and digital arts world.

Sova can send and receive OSC messages. You can configure multiple OSC input and output devices in Sova, and use them simultaneously. Up to 32 devices can be configured through the [DeviceMap](/docs/core/devices).

By default, an OSC output named [SuperDirt](https://github.com/musikinformatik/SuperDirt) port will always be created at the start of a session and associated with the slot n°1. It allows you to communicate with SuperCollider and most specifically with SuperDirt, a popular synthesis engine for live coding.