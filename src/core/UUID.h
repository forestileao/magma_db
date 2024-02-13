//
// Created by hellhat on 2/13/24.
//
#include <string>

#ifndef PEDRO_DB_SRC_CORE_UUID_H_
#define PEDRO_DB_SRC_CORE_UUID_H_

namespace PedroDb::Core
{
	class UUID final
	{
	 public:
		[[nodiscard]] static std::string generateUUID();
	};
}

#endif //PEDRO_DB_SRC_CORE_UUID_H_
