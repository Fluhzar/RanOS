# Fluhzar - RanOS

|Table of Contents                     |
|:-------------------------------------|
|[Suit](#-suit)                        |
|[Character Design](#-character-design)|
|[Electronics](#-electronics)          |
|[Programming](#-programming)          |

## Suit

* 3D printed and painted (might change to foam).
* Face mask is dark clear plastic (black PETG?) that will either be 3D printed or vacuum molded.
* Figure out how to make plastic smooth without ruining it if 3D painted (step through with finer and finer grit sand paper?).
* Finished suit needs to be mobile, and not hazardous for hugs.
* Decide how to implement body lights.
* Add in cooling somehow (vents and fans, shirt with water tubing and ice water, etc.).
* Jaw/mouth can move?

## Character design

* Add fluffy bits for more pleasant hugs.
* Finalize LED colors (probably move away from purple for the sake of color quality with LEDs).

## Electronics

* Hardware needed:
  * LEDs - APA102C-5050 (or SK9822 clones).
  * Microphone - something with a higher SNR to remove unwanted noise.
  * Camera (for vision, more later) - wide-angled preferred (GoPro?).
  * Cheap VR headset - Google Cardboard + phone (external battery to keep power for longer suiting sessions).
    * Removable from head?
* Figure out how to make lighting modular/removable for suit cleaning.
* Mount headset inside head with remaining space for face lights.
* Find way to house host hardware (RPi, batteries, cooling, etc.).

## Programming

* Animations:
  * Decide on animation file format, likely using some form of sparse vector with location and color data(?).
  * Maybe design base forms and create procedural animations that morph what is already set.
* Audio:
  * FT - figure out where to run FFT (on RPi most likely), how/what information to get out of it (beats, intensity, pitch-based) and translate them to animations (pulsing, different LEDs lighting up according to different aspects of the sound).
  * Pipe microphone input to FFT for real-time audio analysis.
* Visual:
  * Figure out how to pipe video to headset (additional RPi, or other more powerful single-board computer, video trans-encoder?).
    * additional RPi.
    * other more powerful single-board computer.
    * if using google cardboard or other phone-based headset, ability to pipe video to phone display (GoPro live feed?).
      * Convert video feed to stereo for each eye?
      * Use two cameras?
      * Make my own app using cardboard SDK?
