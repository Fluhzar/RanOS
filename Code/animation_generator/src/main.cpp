/*! ****************************************************************************
\file             main.cpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  AnimationGenerator

\copyright        Copyright © 2018 Chyler
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <iostream>
#include <fstream>

#include "gen.hpp"
#include "RGB.hpp"
#include <RIFF-Util/RIFF.hpp>

// Private Macros                         //////////////////////////////////////

#define COLOR_SIZE 1024
#define MAX_VALUE 0x80  // range: 0 to 255, 0x00 to 0xFF
#define MIN_VALUE 0x00  // range: 0 to 255, 0x00 to 0xFF

#define NUM_LEDS 9
#define NUM_FRAMES COLOR_SIZE

#define CYCLE_PERIOD (9.0f/1.0f) // measured in seconds

#ifndef RANOS_DATA_DIR
  #define RANOS_DATA_DIR "dat"
#endif
#ifndef RANOS_DEFAULT_FILE
  #define RANOS_DEFAULT_FILE "Output.bin"
#endif

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

static RanOS::RGB color_table[COLOR_SIZE];

// Private Function Declarations          //////////////////////////////////////

static RanOS::Frame GetFrame();
static void ColorInit();
static RanOS::RGB ColorLerp(RanOS::RGB const & start, RanOS::RGB const & end, uint16_t step, uint16_t size);

// Public Functions                       //////////////////////////////////////

int main(int argc, char * argv[])
{
  (void)(argc);
  (void)(argv);

  ColorInit();

  RanOS::GeneratorOptions options(GetFrame, NUM_FRAMES, true);

  RIFF::Writer riff(CONSTRUCT_BYTE_STR("ANIM"));

  auto chunk = RanOS::GenerateAnimationData(options, 0);

  riff.AddChunk(CONSTRUCT_BYTE_STR(std::get<0>(chunk).c_str()), std::get<1>(chunk));

  RIFF::ofstream_t file((RANOS_DATA_DIR RANOS_DEFAULT_FILE), std::ios::binary);

  file << riff.RIFFData();

  return 0;
};

// Private Functions                      //////////////////////////////////////

static RanOS::Frame GetFrame()
{
  static uint32_t c = 0;
  RanOS::Frame f;

  f.m_Frame = new RanOS::RGB[NUM_LEDS];
  f.m_Size = NUM_LEDS;
  f.m_FrameDuration = CYCLE_PERIOD/NUM_FRAMES;

  for(uint32_t i = 0; i < NUM_LEDS; ++i)
  {
    f.m_Frame[i] = color_table[((i * COLOR_SIZE / NUM_LEDS) + (c * COLOR_SIZE / NUM_FRAMES)) % COLOR_SIZE];
  }

  ++c;

  return f;
}

static void ColorInit()
{
  static RanOS::RGB const RED     (MAX_VALUE, MIN_VALUE, MIN_VALUE);
  static RanOS::RGB const YELLOW  (MAX_VALUE, MAX_VALUE, MIN_VALUE);
  static RanOS::RGB const GREEN   (MIN_VALUE, MAX_VALUE, MIN_VALUE);
  static RanOS::RGB const CYAN    (MIN_VALUE, MAX_VALUE, MAX_VALUE);
  static RanOS::RGB const BLUE    (MIN_VALUE, MIN_VALUE, MAX_VALUE);
  static RanOS::RGB const MAGENTA (MAX_VALUE, MIN_VALUE, MAX_VALUE);
  uint16_t i;

  for(i = 0; i < COLOR_SIZE/6; ++i)
  {
    color_table[i] = ColorLerp(RED, YELLOW, i, COLOR_SIZE/6);
  }
  for(; i < COLOR_SIZE/3; ++i)
  {
    color_table[i] = ColorLerp(YELLOW, GREEN, i - COLOR_SIZE/6, COLOR_SIZE/6);
  }
  for(; i < COLOR_SIZE/2; ++i)
  {
    color_table[i] = ColorLerp(GREEN, CYAN, i - COLOR_SIZE/3, COLOR_SIZE/6);
  }
  for(; i < 2*COLOR_SIZE/3; ++i)
  {
    color_table[i] = ColorLerp(CYAN, BLUE, i - COLOR_SIZE/2, COLOR_SIZE/6);
  }
  for(; i < 5*COLOR_SIZE/6; ++i)
  {
    color_table[i] = ColorLerp(BLUE, MAGENTA, i - 2*COLOR_SIZE/3, COLOR_SIZE/6);
  }
  for(; i < COLOR_SIZE; ++i)
  {
    color_table[i] = ColorLerp(MAGENTA, RED, i - 5*COLOR_SIZE/6, COLOR_SIZE/6);
  }
}

static RanOS::RGB ColorLerp(RanOS::RGB const & start, RanOS::RGB const & end, uint16_t step, uint16_t size)
{
  static RanOS::RGB out;

  out[0] = uint8_t((end[0]-start[0]) / float(size) * step + start[0]);
  out[1] = uint8_t((end[1]-start[1]) / float(size) * step + start[1]);
  out[2] = uint8_t((end[2]-start[2]) / float(size) * step + start[2]);

  return out;
}
