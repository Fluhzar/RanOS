# Fluhzar - RanOS

A document full of spit-balled ideas that will be refined over time into a more constructive planning document.

|Table of Contents                    |
|:------------------------------------|
|[Suit](#suit)                        |
|[Character Design](#character-design)|
|[Electronics](#electronics)          |
|[Programming](#programming)          |

## Suit

I'm hoping to be able to design a method of production that can expand beyond just a one-off suit. I'd like to transform this project into a business if possible and keeping that in mind during the overall design of the project will surely only be beneficial to that end. That being said, I'm quite new to all of this and don't have much in the way of funds to start off with, so I'm kinda throwing stuff together in theory first and will be improving and learning later.

* 3D printed and painted - might change to foam
* Face mask is dark clear plastic that will either be 3D printed or vacuum molded
* Figure out how to make plastic smooth without ruining it if 3D printed - try stepping through with finer and finer grit sand paper
* Finished suit needs to be mobile, and not hazardous for hugs
* Determine how to implement body lights
* Add in cooling somehow - vents and fans, shirt with water tubing and ice water, etc.
* Movable jaw/mouth

## Character design

* Add fluffy/soft bits for more pleasant hugs
* Finalize LED colors - probably move away from purple for the sake of color quality with LEDs

## Electronics

* Hardware needed:
  * LEDs - APA102C-5050 or SK9822 (clones)
  * Microphone - something with a higher SNR to remove unwanted noise.
  * Camera - wide-angled preferred
  * Cheap VR headset - Google Cardboard + phone, would need external battery to keep power for longer suiting sessions and cooling for sure
    * Look into developing own custom headset hardware - 3D printing should help with this
    * Removable from head for cleaning
* Figure out how to make lighting removable for suit cleaning
* Make sure headset and faceplate LEDs can all fit within the head
* Hosted on Raspberry Pi, LEDs connected through GPIO, mic and other processing still TBD

## Programming

* Animations:
  * Investigate possible animation storage formats - likely using some form of sparse vector with location and color data
  * Maybe design base preset and create procedural animations that morph what is already set
* Audio:
  * Audio Reactivity:
    * Figure out where to run FFT, how/what information to get out of it (beats, intensity, pitch-based) and translate them to animations (pulsing, different LEDs lighting up according to different aspects of the sound)
      * Leverage ambisonic concepts if applicable
    * Pipe microphone input to FFT for real-time audio analysis.
* Visual:
  * Figure out how to pipe video to headset - additional RPi, or other more powerful single-board computer, dedicated video trans-coder
    * if using google cardboard or other phone-based headset, ability to pipe video to phone display (GoPro live feed?).
      * Investigate to feed the video for each eye
        * Decide on the number of cameras, 2 for stereoscopic vision or just 1
      * Likely develop a custom app to work with the headset
