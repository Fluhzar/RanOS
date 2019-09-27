/*! ****************************************************************************
\file             gen.cpp
\author           Fluhzar
\par    Email:    fluhzar\@gmail.com
\par    Project:  AnimationGenerator

\copyright        Copyright © 2018 Fluhzar
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cstdio>
#include <cstring>

#include <string>

#include "RGB.hpp"
#include "gen.hpp"

// Private Macros                         //////////////////////////////////////

#define FLOAT_TO_Q15(x) int16_t(x * (1 << 15))

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

namespace RanOS
{

  struct AnimationChunk
  {
    uint16_t FrameSize;
    int16_t SizeMask;
  };

  struct FrameChunk
  {
    int16_t FrameDuration;
    RGB * FramePixels;
  };

} // namespace RanOS

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace RanOS
{

  std::tuple<std::string,std::vector<uint8_t>> GenerateAnimationData(GeneratorOptions const & options, uint16_t index)
  {
      // Assign the hex animation index
    char buf[4] = {0};
    int c = snprintf(buf, 3, "%X", index);

      // Create the label to store the animation
    char label[5] = {'A', 'N', 'I', 'M', 0};
    strncpy(label+(4-c), buf, c);

      // Get all the animation frames
    std::vector<Frame> frames;
    for(uint16_t i = 0; i < (options.m_SizeMask & 0x7FFF); ++i)
    {
      frames.push_back(options.m_CallBack());
    }

      // Set the animation chunk's options
    AnimationChunk anim;
    anim.SizeMask = options.m_SizeMask;
    anim.FrameSize = frames[0].m_Size;

      // Convert all the animation frames to frame chunks
    std::vector<FrameChunk> frame_chunks;
    for(auto & f : frames)
    {
      FrameChunk chunk;

      chunk.FrameDuration = FLOAT_TO_Q15(f.m_FrameDuration);
      chunk.FramePixels = new RGB[anim.FrameSize];
      memcpy(chunk.FramePixels, f.m_Frame, sizeof(RGB) * anim.FrameSize);
      delete [] f.m_Frame;

      frame_chunks.push_back(chunk);
    }

      // Create a vector and initialize with the exact amount of space needed to hold the animation chunk
    std::vector<uint8_t> data_vec(4 + (anim.SizeMask & 0x7FFF) * (2 + anim.FrameSize * 3));
    uint8_t * data = &data_vec[0];

      // Copy the animation data
    memcpy(data, &(anim.FrameSize), 2);
    data += 2;
    memcpy(data, &(anim.SizeMask), 2);
    data += 2;

      // Copy all of the frames' data
    for(auto & f : frame_chunks)
    {
      memcpy(data, &(f.FrameDuration), 2);
      data += 2;

      for(uint32_t i = 0; i < anim.FrameSize; ++i)
      {
        memcpy(data++, &((f.FramePixels + i)->r), 1);
        memcpy(data++, &((f.FramePixels + i)->g), 1);
        memcpy(data++, &((f.FramePixels + i)->b), 1);
      }
    }

    return std::make_tuple(std::string(label), data_vec);
  }

} // namespace RanOS

// Private Functions                      //////////////////////////////////////
