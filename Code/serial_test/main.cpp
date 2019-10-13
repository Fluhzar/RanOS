/*! ****************************************************************************
\file             Serial.cpp
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
*******************************************************************************/

// Include Files							////////////////////////////////////

#include <cstdint>

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
	RanOS::Serial teensy(argv[1]);

	char str[] = "12345teststr";

	RanOS::Serial::bytevec_t vec(str, str+sizeof(str));

	teensy.Write(vec);

	auto data = teensy.Read();

	std::string original(str), returned(&data[0]);

	std::cout << "Original string: " << original << "\nReturned string: " << returned << '\n';

	return 0;
}

// Private Functions						////////////////////////////////////
