/*! ****************************************************************************
\file             LEDs.hpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

#ifndef __RANOS_HPP
#define __RANOS_HPP

// Include Files                          //////////////////////////////////////

#include "Macro.hpp"

#include "RGB.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace RanOS
{
    /*! ************************************************************************
    \brief
        LED controller class for handling setting the values for addressable
        APA102-based LEDs.
    ***************************************************************************/
    struct Controller
    {
    public:
            /// Pointer to a Controller member function to select between different end frames.
        using EndFrame_f = void(Controller::*)() const;

        // Members              ///////////////////////

            /// The data pin for the LEDs
        uint8_t m_Data;
            /// The clock pin for the LEDs
        uint8_t m_Clock;
            /// The global brightness value for the LEDs.
        uint8_t m_Brightness;
            /// Pointer to the end frame this controller will use.
        EndFrame_f m_EndFrame;
            /// Pointer to an array of RGB values to output to the LEDs.
        RGB const * m_Buffer;
            /// The size of the RGB buffer.
        uint16_t m_BufSize;

        // Con-/De- structors   ///////////////////////

        /*! ********************************************************************
        \brief
            Constructor. Initializes all the values to allow this controller to
            operate properly.

        \param data
            The data pin for the LEDs.

        \param clock
            The clock pin for the LEDs.

        \param LEDs
            Pointer to an array of RGB values.

        \param num_panels
            The number of panels that are represented by the LEDs array.

        \param b
            The global brightness for the LEDs. Range: 0 to 31 (0x00 to 0x31).
            Can be set later with a call to Controller::Brightness.
        ***********************************************************************/
        Controller(uint8_t data, uint8_t clock, RGB const * LEDs = nullptr, uint16_t num_panels = 0, uint8_t b = 0x1F);

        inline Controller(Controller const &) = default;        ///< Default copy constructor
        inline Controller(Controller &&) noexcept = default;    ///< Default move constructor

        inline ~Controller() = default; ///< Default destructor

        // Operators            ///////////////////////

        inline Controller & operator=(Controller const &) = default;        ///< Default assignment operator
        inline Controller & operator=(Controller &&) noexcept = default;    ///< Default move operator

        // Accossors/Mutators   ///////////////////////

        /*! ********************************************************************
        \brief
            Sets the global brightness of the LEDs.

        \param b
            The new global brightness. Range of 0 to 31 (0x00 to 0x1F).
        ***********************************************************************/
        inline void Brightness(uint8_t b) { m_Brightness = b>0x1F ? 0x1F : b; };

        // Functions            ///////////////////////

        /*! ********************************************************************
        \brief
            Arranges the the RGB values and global brightness into the data
            format required by the APA102 datasheet.
        ***********************************************************************/
        void Display() const;

    private:

        // Functions            ///////////////////////

        /*! ********************************************************************
        \brief
            Function to write the start frame to the LEDs.
        ***********************************************************************/
        inline void StartFrame() const;

        /*! ********************************************************************
        \brief
            Function to write the end frame to the LEDs if there is only 1 panel
            connected to the controller.
        ***********************************************************************/
        void SmallEndFrame() const;

        /*! ********************************************************************
        \brief
            Function to write the end frame to the LEDs if there is more than
            1 panel connected to the controller.
        ***********************************************************************/
        void LargeEndFrame() const;

        /*! ********************************************************************
        \brief
            Writes a single byte to the LEDs in MSB->LSB order.

        \param byte
            The byte to write.
        ***********************************************************************/
        inline void Write(uint8_t const & byte) const;
    };  // Controller
}   // namespace RanOS

// Public Functions             ////////////////////////////////////////////////

#endif // __RANOS_HPP
