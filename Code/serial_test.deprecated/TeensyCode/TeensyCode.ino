/*! ****************************************************************************
\file             TeensyCode.ino
\author           Fluhzar
\par    Email:    fluhzar\@pm.me
\par    Project:  RanOS

\copyright        Copyright © 2019 Fluhzar
\par    See LICENSE file in project root directory for more information
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include <cstdint>

#include <vector>

using byte_t = uint8_t;

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

byte_t buffer[16 << 10] {0};

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

void setup()
{
	Serial.begin(9600);
}

void loop()
{
	while(Serial.available() > 0)
	{
		Serial.write(Serial.read());
	}
}

// Private Functions                      //////////////////////////////////////
