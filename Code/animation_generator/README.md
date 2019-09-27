# Animation Generator

## Purpose

This project is designed with the intention of creating animations that will be loaded onto a MicroSD card that will be inserted into the Teensy 3.6 microcontroller running the LEDs on RanOS

## Data Format

The output of the generator is a binary format storing the animation frames with the goal of creating a small file size to lessen the load on the microcontroller during operation.

For the purposes of this project and RanOS as a whole, it'd be best to define a few terms:

* Animation frame - A linear array of 8-bit RGB color values and an associated duration (currently measured in seconds [might possibly move to milliseconds]) for the frame. The individual frames that make up an animation sequence.
* Animation - A collection of animation frames that define a animated sequence

Now that that's out of the way, the format for the file storing the animations will be RIFF style as I believe it is an easy way to keep the overall file size to a minimal while still keeping useful information like data size and label

Indivdual frame-sub chunks will be in the following format:

```
[Frame sub-chunk]
   |- FrameDuration: Q.15 fixed-point number containing the duration of this frame
   |- FramePixels: All RGB values listed in linearly increasing order, written
                   using operator<< of the RGB struct. Should be
                   sizeof(RGB) * Size bytes long
```

NOTE: the frame sub-chunk does not contain any RIFF-like header as the needed information is provided here and in the Animation chunk. Thus, frames are written in sequential order within the animation

For an entire animation, the format is as follows:
```
[Animation RIFF header]
     |- FrameSize: An unsigned 16-bit integer size number of LEDs/pixels (defined
     |             here as it is the same for all frames in the animation)
     |- SizeMask: A signed 16-bit integer size containing the number of frames
     |            with the sign bit masked with the looping option (1 for
     |            looping, 0 for is not looping). Sign bit is masked to 0 before
     |            saving the size within the animation object.
     |- [Frame sub-chunk]
     |- [Frame sub-chunk]
     |- [Frame sub-chunk]

     ...

     |- [Frame sub-chunk]
```

These are then organized into the file in their own separate RIFF chunks:
```
[RIFF header]
      |- [Animation RIFF chunk]
      |   |- Label: "ANI0"
      |- [Animation RIFF chunk]
      |- [Animation RIFF chunk]

      ...

      |- [Animation RIFF chunk]
      |   |- Label: "ANAC"  // Animation number 0xAC = 172 in base 10
      |- [Animation RIFF chunk]
[EOF]
```

As each animation is its own chunk, and each chunk requires a unique label, the labeling shall be made as follows:

* Animations are labeled as "ANI#", "AN##", or "A###" with '###' replaced by the animation number, with respect to how large the value is in hexadecimal. This allows for up to 4096 unique animations in a single RIFF file, given that the total file size fits within the 32-bit size requirement (roughly 4GB).
