/*! ****************************************************************************
\file             Serial.cpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
*******************************************************************************/

// Include Files							////////////////////////////////////

#include <errno.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <iostream>

#include "Serial.hpp"

// Private Macros							////////////////////////////////////

// Private Enums							////////////////////////////////////

// Private Objects							////////////////////////////////////

// Private Function Declarations			////////////////////////////////////

// Public Functions							////////////////////////////////////

namespace RanOS
{
	Serial::Serial(): m_Port(-1), m_Settings()
		#if !SERIALIO_BLOCK
			, m_Mutex(), m_Lock(m_Mutex), m_Thread()
		#endif
	{
	}

	Serial::Serial(std::string const & port): m_Port(-1), m_Settings()
	{
		Open(port);
	}

	Serial::~Serial()
	{
		Close();
	}

	bool Serial::Open(std::string const & port)
	{
		m_Port = open(port.c_str(), O_RDWR);

		if(m_Port < 0)
		{
			std::cerr << "Unable to open " << port << '\n';

			switch(errno)
			{
				case EACCES:
					std::cerr << "Search permission is denied\n";
					break;
				case EEXIST:
					std::cerr << "Named files exists\n";
					break;
				case EINTR:
					std::cerr << "A signal was caught during open()\n";
					break;
				case EINVAL:
					std::cerr << "Implementation does not support synchronized I/O for this file\n";
					break;
				case EIO:
					std::cerr << "The path argument names a STREAMS file and a hangup or error occurred during the open()\n";
					break;
				case EISDIR:
					std::cerr << '"' << port << "\" is a directory\n";
					break;
				case ELOOP:
					std::cerr << "A loop exists in symbolic links\n";
					break;
				case EMFILE:
					std::cerr << "The maximum file descriptors are currently open in the calling process\n";
					break;
				case ENAMETOOLONG:
					std::cerr << "The length of the path argument exceeds the maximum path name\n";
					break;
				case ENFILE:
					std::cerr << "The maximum allowable number of files is currently open in the system\n";
					break;
				case ENOENT:
					std::cerr << "File doesn't exist\n";
					break;
				case ENOSR:
					std::cerr << "The path names a STREAMS-based file and the system is unable to allocate a STREAM\n";
					break;
				case ENOSPC:
					std::cerr << "Directory within path was unable to be expanded\n";
					break;
				case ENOTDIR:
					std::cerr << "A component of the path is not a directory\n";
					break;
				case ENXIO:
					std::cerr << "O_NONBLOCK is set, the named file is a FIFO, O_WRONLY is set, and no process has the file open for reading\n";
					break;
				case EOVERFLOW:
					std::cerr << "The named file is a regular file and the size of the file cannot be represented correctly in an object of type off_t\n";
					break;
				case EROFS:
					std::cerr << "The named file resides on a read-only file system and either O_WRONLY, O_RDWR, O_CREAT (if the file does not exist), or O_TRUNC is set in the oflag argument\n";
					break;
				case EAGAIN:
					std::cerr << "The path argument names the slave side of a pseudo-terminal device that is locked\n";
					break;
				case ENOMEM:
					std::cerr << "The path argument names a STREAMS file and the system is unable to allocate resources\n";
					break;
				case ETXTBSY:
					std::cerr << "The file is a pure procedure (shared text) file that is being executed and oflag is O_WRONLY or O_RDWR\n";
					break;
			};

			return false;
		}

		tcgetattr(m_Port, &m_Settings);
		cfmakeraw(&m_Settings);
		tcsetattr(m_Port, TCSANOW, &m_Settings);

		#if !SERIALIO_BLOCK
			m_IsOpen = true;
			m_Thread = std::thread(Serial::ThreadMain, this);
		#endif

		return true;
	}

	void Serial::Close()
	{
		if(m_Port >= 0)
		{
			close(m_Port);
			m_Port = -1;

			#if !SERIALIO_BLOCK
				m_IsOpen = false;
				m_Thread.join();
			#endif
		}
	}

	Serial::bytevec_t Serial::Read()
	{
		#if SERIALIO_BLOCK
			return PureRead();
		#else
			m_Lock.lock();
			bytevec_t data(m_ReadQueue.begin(), m_ReadQueue.end());
			m_ReadQueue.clear();
			m_Lock.unlock();

			return data;
		#endif
	}

	void Serial::Write(bytevec_t const & data)
	{
		#if SERIALIO_BLOCK
			PureWrite(data);
		#else
			m_Lock.lock();
			m_WriteQueue.insert(m_WriteQueue.end(), data.begin(), data.end());
			m_Lock.unlock();
		#endif
	}
} // namespace RanOS

// Private Functions						////////////////////////////////////

namespace RanOS
{
	Serial::bytevec_t Serial::PureRead()
	{
		bytevec_t data(SERIALIO_MAXBUF);

		auto n = read(m_Port, &data[0], SERIALIO_MAXBUF);

		if(n < 1)
		{
			std::cerr << "Error reading from port " << m_Port << '\n';
			return bytevec_t();
		}

		data.erase(data.begin() + n, data.end());

		return data;
	}

	void Serial::PureWrite(bytevec_t const & data)
	{
		ssize_t n = 0;

		while(n < data.size())
		{
			auto tmp = write(m_Port, &data[0] + n, data.size() - n);
			n += tmp;
		}
	}

	#if !SERIALIO_BLOCK
		void Serial::ThreadMain(Serial * s)
		{
			while(s->m_IsOpen)
			{
				// Do some reading
				{
					bytevec_t data = s->PureRead();
					s->m_Lock.lock();
					s->m_ReadQueue.insert(s->m_ReadQueue.end(), data.begin(), data.end());
					s->m_Lock.unlock();
				}
				// Do some writing
				{
					s->m_Lock.lock();
					bytevec_t data(s->m_WriteQueue.begin(), s->m_WriteQueue.end());
					s->m_WriteQueue.clear();
					s->m_Lock.unlock();
					s->PureWrite(data);
				}
				// Let other processes run
				std::this_thread::yield();
			}
		}
	#endif
}
