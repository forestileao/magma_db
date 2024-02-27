//
// Created by hellhat on 2/13/24.
//

#include "UUID.h"
#include <sstream>

using namespace MagmaDb::Core;

UUIDGenerator::UUIDGenerator() : uuidv4Generator()
{
}
std::string UUIDGenerator::generateUUIDString()
{
	UUIDv4::UUID uuid = uuidv4Generator.getUUID();
	return uuid.str();
}

