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

#ifndef SERIALIO_BLOCK
	#define SERIALIO_BLOCK (false)
#endif

#ifndef SERIALIO_MAXBUF
	#define SERIALIO_MAXBUF (16 << 10)
#endif

// Forward References			////////////////////////////////////////////////

// Public Enums					////////////////////////////////////////////////

// Public Objects				////////////////////////////////////////////////

#if !SERIALIO_BLOCK
	#include <thread>
	#include <mutex>
	#include <deque>
#endif

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

		#if !SERIALIO_BLOCK
			using mutex_t = std::mutex;
			using lock_t = std::unique_lock<mutex_t>;
			using thread_t = std::thread;
			using deque_t = std::deque<byte_t>;

			mutex_t m_Mutex;
			lock_t m_Lock;
			thread_t m_Thread;

			deque_t m_WriteQueue;
			deque_t m_ReadQueue;

			bool m_IsOpen;
		#endif

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

		bytevec_t PureRead();
		void PureWrite(bytevec_t const &);

		#if !SERIALIO_BLOCK
			static void ThreadMain(Serial *);
		#endif

	}; // class Serial
} // namespace RanOS

// Public Functions				////////////////////////////////////////////////

#endif // __SERIAL_HPP
