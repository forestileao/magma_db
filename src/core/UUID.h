//
// Created by hellhat on 2/13/24.
//
#include <string>
#include <uuid_v4.h>

#ifndef MAGMA_DB_SRC_CORE_UUID_H_
#define MAGMA_DB_SRC_CORE_UUID_H_

namespace MagmaDb::Core
{
	class UUIDGenerator
	{
	 private:
		UUIDv4::UUIDGenerator<std::mt19937_64> uuidv4Generator;
	 public:
		UUIDGenerator();

		std::string generateUUIDString();
	};
}

#endif //MAGMA_DB_SRC_CORE_UUID_H_
