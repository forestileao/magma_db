//
// Created by hellhat on 2/12/24.
//
#include <variant>
#include <vector>
#include <string>
#include <unordered_map>
#include <memory>
#include "Document.h"

#ifndef PEDRO_DB_SRC_CORE_DOCUMENTVALUE_H_
#define PEDRO_DB_SRC_CORE_DOCUMENTVALUE_H_

#define EMPTY_KEY "empty_key"

using std::vector;
using std::unordered_map;

namespace PedroDb::Core
{
	using Array = vector<DocumentValue>;

	enum class DocumentValueType
	{
		STRING,
		INTEGER,
		DOUBLE,
		BOOL,
		ARRAY,
		DOCUMENT,
		NONE
	};

	class DocumentValue
	{
	 private:
		std::string key = EMPTY_KEY;
		DocumentValueType type;

		void checkOrThrow(DocumentValueType expectedType) const;

		[[nodiscard]] std::string getTypeString() const;
		[[nodiscard]] static std::string getTypeString(DocumentValueType type);

		// Variant to store different types of values
		std::variant<std::string, int, double, bool, Array, Document, std::monostate> value;

	 public:

		DocumentValue();
		explicit DocumentValue(const std::string& value);
		explicit DocumentValue(const char value[]);
		explicit DocumentValue(int value);
		explicit DocumentValue(double value);
		explicit DocumentValue(bool value);
		explicit DocumentValue(const Array& array);
		explicit DocumentValue(const Document& document);

		~DocumentValue();

		[[nodiscard]] DocumentValueType getType() const;

		[[nodiscard]] std::string getString() const;
		[[nodiscard]] int getInteger() const;
		[[nodiscard]] double getDouble() const;
		[[nodiscard]] bool getBool() const;
		[[nodiscard]] const Array& getArray() const;
		[[nodiscard]] const Document& getDocument() const;
		[[nodiscard]] const std::string& getKey() const;

		void setString(const std::string& stringValue);
		void setInteger(int intValue);
		void setDouble(double doubleValue);
		void setBool(bool boolValue);
		void setArray(const Array& arrayValue);
		void setDocument(const Document& documentValue);
		void setKey(const std::string& keyName);

		[[nodiscard]] bool isNone() const;
		[[nodiscard]] bool isString() const;
		[[nodiscard]] bool isInteger() const;
		[[nodiscard]] bool isDouble() const;
		[[nodiscard]] bool isBool() const;
		[[nodiscard]] bool isArray() const;
		[[nodiscard]] bool isDocument() const;
	};
}

#endif //PEDRO_DB_SRC_CORE_DOCUMENTVALUE_H_
