/*! ****************************************************************************
\file             Serial.cpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
*******************************************************************************/

// Include Files							////////////////////////////////////

#include <cstdint>

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#include "Serial.hpp"

// Private Macros							////////////////////////////////////

// Private Enums							////////////////////////////////////

// Private Objects							////////////////////////////////////

// Private Function Declarations			////////////////////////////////////

// Public Functions							////////////////////////////////////

int main(int argc, char * argv[])
{
	if(argc != 2)
	{
		std::cerr << "Incorrect command-line parameters given\n\n";
		std::cerr << "Usage:\n";
		std::string arg0(argv[0]);
		std::cerr << "\t" << arg0.substr(arg0.find_last_of('/')) << " /path/to/teensy/device\n";
		std::cerr << "\t\tLikely /dev/tty[teensy something]\n";

		return -1;
	}

	RanOS::Serial teensy;

	char str[] = "12345teststr";
	RanOS::Serial::bytevec_t vec(str, str+sizeof(str));

	if(!teensy.Open(argv[1]))
	{
		std::cerr << '"' << argv[1] << "\" couldn't be opened\n";
		return -1;
	}

	teensy.Write(vec);
	auto data = teensy.Read();

	if(data.size() == 0)
	{
		std::cerr << "Read failed, exiting\n";
		return -1;
	}

	std::string original(str);
	std::string returned(reinterpret_cast<char const *>(&data[0]));

	std::cout << "Original string: " << original << "\nReturned string: " << returned << '\n';

	return 0;
}

// Private Functions						////////////////////////////////////
