# Fluhzar - RanOS

| Table of Contents                          |
|:-------------------------------------------|
| [What is RanOS](#-what-is-ranos)           |
| [LEDs](#-leds)                             |
| [Audio Processing](#-audio-processing)     |
| [Electronics](#-electronics)               |
| [Future Video Plans](#-future-video-plans) |

## What is RanOS?

* A synth character designed by Fluhzar, species by Vader-San
* A fursuit project incorporating electronics, LEDs, and audio processing

## LEDs

* A synth can have many lighting components, and RanOS is outfitted with a variety lighting components across their body, RGB LEDs are the illumination for these components.
* This includes body lights as well as the face plate

## Audio Processing

* All the lights on RanOS's body will be reactive in some way to the auditory environment around RanOS, whether it be through on-board mic(s) or a direct line from a DJ controller for example.
* Processing algorithms will include gain analysis (across different frequency bands) and beat analysis

## Electronics

* Something needs to power the LEDs and perform the audio processing
* Likely done through a Raspberry Pi 3 running headless and a Teensy 4.0 microcontroller
* Division of processing is largely still to be determined:
    * RPi will definitely handle audio input from mic and line-in
    * Teensy 4.0 will definitely handle the LED processing
    * TBD: Where the interpretation of processed audio into LED frames occurs (likely on RPi, but benchmarking needs to be done)

## Future video plans

Delve deeper into LEDs, audio processing, and electronics in individual videos. Then go through the project outline in separate video(s) and explain the project status, next steps, long-term goals, and what is needed to achieve each milestone.
