/*! ****************************************************************************
\file             Animation.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __ANIMATION_HPP
#define __ANIMATION_HPP

// Include Files                ////////////////////////////////////////////////

#include "Macro.hpp"

#include <functional>

#include "LEDs.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{
    struct RGB;

    /*! ************************************************************************
    \brief
        Class defining the characteristics of a single frame of an animation
    ***************************************************************************/
    struct Frame
    {
    public:

        // Members              ///////////////////////

            /// The RGB values for the LED frame.
        RGB m_Frame[MAX_LEDS];
            /// The number of values in m_Frame.
        uint16_t m_Size;
            /// The duration this frame lasts in seconds.
        float m_FrameDuration;

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Constructor, sets the size and duration of the frame.

        \param size
            The size of the array in the frame.

        \param duration
            The duration of a frame.
        ***********************************************************************/
        inline Frame(uint16_t size = 0, float duration = 0.f);

        inline Frame(Frame const &) = default;      ///< Default copy constructor.
        inline Frame(Frame &&) noexcept = default;  ///< Default move constructor.

        inline ~Frame() = default;                  ///< Default destructor.

        // Operators            ///////////////////////

        inline Frame & operator=(Frame const &) = default;      ///< Default assignment operator.
        inline Frame & operator=(Frame &&) noexcept = default;  ///< Default move operator.

    }; // struct Frame

    /*! ************************************************************************
    \brief
        Class defining the interface for running an animation (series of frames)
        on a LED Controller (seen in LEDs.hpp).
    ***************************************************************************/
    class Animation
    {
    public:

            /// Callback function to generate new RGB values for the next frame.
        using FrameCallback_f = void(*)(void);

    private:

        // Members              ///////////////////////

            /// LED controller that controls physical hardware.
        Controller m_Controller;
            /// Pointer to the callback function that will generate new RGB values for the next frame.
        FrameCallback_f m_Sequencer;
            /// Pointer to the frame object that the callback function updates.
        Frame const * m_Frame;
            /// Accumulator for the current time.
        float m_Time;

    public:

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Constructor.

        \param sequencer
            Pointer to the callback function that will generate new RGB values
            for the next frame.

        \param frame
            Pointer to the frame object that sequencer updates.

        \param num_panels
            The number of physical LED panels that this animation will support.

        \param data
            The data pin for the LED controller.

        \param clock
            The clock pin for the LED controller.
        ***********************************************************************/
        Animation(FrameCallback_f sequencer, Frame const * frame, uint16_t num_panels, uint8_t data, uint8_t clock);

        inline Animation(Animation const &) = default;      ///< Default copy constructor.
        inline Animation(Animation &&) noexcept = default;  ///< Default move constructor.

        inline ~Animation() = default;                  ///< Default destructor.

        // Operators            ///////////////////////

        inline Animation & operator=(Animation const &) = default;      ///< Default assignment operator.
        inline Animation & operator=(Animation &&) noexcept = default;  ///< Default move operator;

        // Accossors/Mutators   ///////////////////////

        /*! ********************************************************************
        \brief
            Returns a reference to the internal LED controller.

        \return
            A reference to the internal LED controller.
        ***********************************************************************/
        inline Controller & GetController() { return m_Controller; };

        /*! ********************************************************************
        \brief
            Returns a reference to the internal LED controller.

        \return
            A reference to the internal LED controller.
        ***********************************************************************/
        inline Controller const & GetController() const { return m_Controller; };

        // Functions            ///////////////////////

        /*! ********************************************************************
        \brief
            Updates the animation with the amount of time that has elapsed.

        \param dt
            The elapsed time in seconds since the last time this function was
            called.
        ***********************************************************************/
        void Update(float dt);

    private:

        // Functions                  ///////////////////////

        /*! ********************************************************************
        \brief
            Initializes all of the parameters for tracking the animation as it
            runs.
        ***********************************************************************/
        void Play();

        /*! ********************************************************************
        \brief
            Advances the animation by one frame, updating all internal values.
        ***********************************************************************/
        void AdvanceFrame();

    };  // class Animation

}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __ANIMATION_HPP
