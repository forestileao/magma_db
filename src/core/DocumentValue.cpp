//
// Created by hellhat on 2/12/24.
//

#include "DocumentValue.h"

using namespace MagmaDb::Core;

// Default constructor
DocumentValue::DocumentValue() : type(DocumentValueType::NONE), value(std::monostate())
{
}

// Constructors for different types
DocumentValue::DocumentValue(const std::string& value) : type(DocumentValueType::STRING), value(value)
{
}

DocumentValue::DocumentValue(const char value[]) : type(DocumentValueType::STRING), value(std::string(value))
{
}

DocumentValue::DocumentValue(int value) : type(DocumentValueType::INTEGER), value(value)
{
}
DocumentValue::DocumentValue(double value) : type(DocumentValueType::DOUBLE), value(value)
{
}
DocumentValue::DocumentValue(bool value) : type(DocumentValueType::BOOL), value(value)
{
}
DocumentValue::DocumentValue(const Array& array) : type(DocumentValueType::ARRAY), value(array)
{
}
DocumentValue::DocumentValue(const Document& document) : type(DocumentValueType::DOCUMENT), value(document)
{
}

// Destructor
DocumentValue::~DocumentValue()
= default;

// Getter functions

DocumentValueType DocumentValue::getType() const
{
	return type;
}

std::string DocumentValue::getString() const
{
	checkOrThrow(DocumentValueType::STRING);
	return std::get<std::string>(value);
}
int DocumentValue::getInteger() const
{
	checkOrThrow(DocumentValueType::INTEGER);
	return std::get<int>(value);
}

double DocumentValue::getDouble() const
{
	checkOrThrow(DocumentValueType::DOUBLE);
	return std::get<double>(value);
}

bool DocumentValue::getBool() const
{
	checkOrThrow(DocumentValueType::BOOL);
	return std::get<bool>(value);
}

const Array& DocumentValue::getArray() const
{
	checkOrThrow(DocumentValueType::ARRAY);
	return std::get<Array>(value);
}

const Document& DocumentValue::getDocument() const
{
	checkOrThrow(DocumentValueType::DOCUMENT);
	return std::get<Document>(value);
}

const std::string& DocumentValue::getKey() const
{
	return key;
}

// Setter functions

void DocumentValue::setString(const std::string& stringValue)
{
	type = DocumentValueType::STRING;
	this->value = stringValue;
}

void DocumentValue::setInteger(int intValue)
{
	type = DocumentValueType::INTEGER;
	this->value = intValue;
}

void DocumentValue::setDouble(double doubleValue)
{
	type = DocumentValueType::DOUBLE;
	this->value = doubleValue;
}

void DocumentValue::setBool(bool boolValue)
{
	type = DocumentValueType::BOOL;
	this->value = boolValue;
}

void DocumentValue::setArray(const Array& arrayValue)
{
	type = DocumentValueType::ARRAY;
	this->value = arrayValue;
}

void DocumentValue::setDocument(const Document& documentValue)
{
	type = DocumentValueType::DOCUMENT;
	this->value = documentValue;
}

void DocumentValue::setKey(const std::string& keyName)
{
	this->key = keyName;
}

// Other functions
bool DocumentValue::isNone() const
{
	return type == DocumentValueType::NONE;
}
bool DocumentValue::isString() const
{
	return type == DocumentValueType::STRING;
}
bool DocumentValue::isInteger() const
{
	return type == DocumentValueType::INTEGER;
}
bool DocumentValue::isDouble() const
{
	return type == DocumentValueType::DOUBLE;
}

bool DocumentValue::isBool() const
{
	return type == DocumentValueType::BOOL;
}

bool DocumentValue::isArray() const
{
	return type == DocumentValueType::ARRAY;
}

bool DocumentValue::isDocument() const
{
	return type == DocumentValueType::DOCUMENT;
}

void DocumentValue::checkOrThrow(DocumentValueType expectedType) const
{
	if (type != expectedType)
	{
		throw std::logic_error(
			"Invalid type for " + key + ". Expected " + getTypeString(expectedType) + " but got " + getTypeString());
	}
}

std::string DocumentValue::getTypeString() const
{
	return getTypeString(type);
}

std::string DocumentValue::getTypeString(DocumentValueType type)
{
	switch (type)
	{
	case DocumentValueType::STRING:
		return "STRING";
	case DocumentValueType::INTEGER:
		return "INTEGER";
	case DocumentValueType::DOUBLE:
		return "DOUBLE";
	case DocumentValueType::BOOL:
		return "BOOL";
	case DocumentValueType::ARRAY:
		return "ARRAY";
	case DocumentValueType::DOCUMENT:
		return "DOCUMENT";
	case DocumentValueType::NONE:
		return "NONE";
	}
}
