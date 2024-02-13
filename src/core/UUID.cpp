//
// Created by hellhat on 2/13/24.
//

// This implementation was got from https://stackoverflow.com/questions/24365331/how-can-i-generate-uuid-in-c-without-using-boost-library
// TODO: create a better implementation that stores UUID bytes and check efficiency

#include "UUID.h"
#include <random>
#include <sstream>

using namespace PedroDb::Core;

std::string UUID::generateUUID()
{
	std::random_device rd;
	std::mt19937 gen(rd());
	std::uniform_int_distribution<> dis(0, 15);
	std::uniform_int_distribution<> dis2(8, 11);

	std::stringstream ss;
	int i;
	ss << std::hex;
	for (i = 0; i < 8; i++)
	{
		ss << dis(gen);
	}
	ss << "-";
	for (i = 0; i < 4; i++)
	{
		ss << dis(gen);
	}
	ss << "-4";
	for (i = 0; i < 3; i++)
	{
		ss << dis(gen);
	}
	ss << "-";
	ss << dis2(gen);
	for (i = 0; i < 3; i++)
	{
		ss << dis(gen);
	}
	ss << "-";
	for (i = 0; i < 12; i++)
	{
		ss << dis(gen);
	};
	return ss.str();
}