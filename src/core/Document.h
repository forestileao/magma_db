//
// Created by hellhat on 2/12/24.
//

#ifndef PEDRO_DB_SRC_CORE_DOCUMENT_H_
#define PEDRO_DB_SRC_CORE_DOCUMENT_H_

#include <uuid/uuid.h>
#include <string>
#include <unordered_map>
namespace PedroDb::Core
{
	class DocumentValue;

	class Document
	{
	 private:
		std::string id;
		std::unordered_map<std::string, DocumentValue> properties;

	 public:
		Document();
		explicit Document(std::string id);
		~Document();

		std::string getId() const;
		DocumentValue& getProperty(const std::string& name);
		DocumentValue& operator[](const std::string& name);

		void setId(std::string newId);
		void setProperty(const std::string& propertyName, DocumentValue& value);
		void operator[](const std::string& name, DocumentValue& value);

		void removeProperty(const std::string& name);
		void clear();
	};
}

#endif //PEDRO_DB_SRC_CORE_DOCUMENT_H_
