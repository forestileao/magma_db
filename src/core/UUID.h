//
// Created by hellhat on 2/13/24.
//
#include <string>

#ifndef MAGMA_DB_SRC_CORE_UUID_H_
#define MAGMA_DB_SRC_CORE_UUID_H_

namespace MagmaDb::Core
{
	class UUID final
	{
	 public:
		[[nodiscard]] static std::string generateUUID();
	};
}

#endif //MAGMA_DB_SRC_CORE_UUID_H_
