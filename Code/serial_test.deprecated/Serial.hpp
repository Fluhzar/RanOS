/*! ****************************************************************************
\file				Serial.hpp
\author				Fluhzar
\par	Email:		fluhzar\@pm.me
\par	Project:	RanOS

\copyright			Copyright © 2019 Fluhzar
*******************************************************************************/

#ifndef __SERIAL_HPP
#define __SERIAL_HPP

// Include Files				////////////////////////////////////////////////

#include <cstdint>

#include <string>
#include <vector>
#include <unistd.h>
#include <termios.h>

// Public Macros				////////////////////////////////////////////////

#ifndef SERIALIO_MAXBUF
	#define SERIALIO_MAXBUF (16 << 10)
#endif

// Forward References			////////////////////////////////////////////////

// Public Enums					////////////////////////////////////////////////

// Public Objects				////////////////////////////////////////////////

namespace RanOS
{
	/*! ************************************************************************
	\brief
	***************************************************************************/
	class Serial
	{
	public:

		using byte_t = uint8_t;
		using bytevec_t = std::vector<byte_t>;

	private:

		// Members				///////////////////////

		int32_t m_Port;
		termios m_Settings;

	public:

		// Con-/De- structors	///////////////////////

		Serial();
		Serial(std::string const &);

		~Serial();

		// Operators			///////////////////////

		// Accossors/Mutators	///////////////////////

		// Functions			///////////////////////

		bool Open(std::string const &);
		void Close();

		bytevec_t Read();
		void Write(bytevec_t const &);

	private:

		// Functions			///////////////////////

		static void ReportError(std::string const &);

	}; // class Serial
} // namespace RanOS

// Public Functions				////////////////////////////////////////////////

#endif // __SERIAL_HPP
