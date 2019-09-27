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
        Struct defining the characteristics of a single frame of an animation.
    ***************************************************************************/
    struct Frame
    {
    public:

        using Reset_f = void(*)(Frame *);
        using Update_f = void(*)(Frame *);

        // Members              ///////////////////////

            /// Reset callback function.
        Reset_f m_Reset;
            /// Update callback function.
        Update_f m_Update;
            /// The RGB values for the LED frame.
        RGB m_Colors[MAX_LEDS];
            /// The number of values in m_Frame.
        uint16_t m_Size;
            /// The duration this frame lasts in seconds.
        float m_FrameDuration;

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Constructor, sets the size and duration of the frame.

        \param reset
            Callback function that resets the frame's state.

        \param update
            Callback function that updates the frame's state.

        \param size
            The size of the array in the frame.

        \param duration
            The duration of a frame.
        ***********************************************************************/
        inline Frame(Reset_f reset, Update_f update, uint16_t size = 0, float duration = 0.f);

        inline Frame(Frame const &) = default;      ///< Default copy constructor.
        inline Frame(Frame &&) noexcept = default;  ///< Default move constructor.

        inline ~Frame() = default;                  ///< Default destructor.

        // Operators            ///////////////////////

        inline Frame & operator=(Frame const &) = default;      ///< Default assignment operator.
        inline Frame & operator=(Frame &&) noexcept = default;  ///< Default move operator.

        // Functions            ///////////////////////

        inline void Reset() { m_Reset(this); };
        inline void Update() { m_Update(this); };

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
        Controller const * m_Controller;
            /// The frame containing the data to be displayed.
        Frame m_Frame;
            /// Accumulator for the current time.
        float m_Time;

    public:

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Constructor.

        \param reset
            Callback function that resets the frame's state.

        \param update
            Callback function that updates the frame's state.

        \param num_panels
            The number of panels held in the internal array of the frame.

        \param duration
            The duration of each frame.

        \param num_panels
            The number of physical LED panels that this animation will support.

        \param data
            The data pin for the LED controller.

        \param clock
            The clock pin for the LED controller.
        ***********************************************************************/
        Animation(Frame::Reset_f reset, Frame::Update_f update, uint16_t num_panels,
                  float duration, Controller * controller);

        inline Animation(Animation const &) = default;      ///< Default copy constructor.
        inline Animation(Animation &&) noexcept = default;  ///< Default move constructor.

        inline ~Animation() = default;                      ///< Default destructor.

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
        inline Controller const * GetController() const { return m_Controller; };

        // Functions            ///////////////////////

        /*! ********************************************************************
        \brief
            Resets the state of the animation to allow the restarting of the
            animation.
        ***********************************************************************/
        void Reset();

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
            Advances the animation by one frame, updating all internal values.
        ***********************************************************************/
        void AdvanceFrame();

    };  // class Animation

}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __ANIMATION_HPP
