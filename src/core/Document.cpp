//
// Created by hellhat on 2/12/24.
//

#include "Document.h"

#include <utility>
#include "UUID.h"
#include "DocumentValue.h"

using namespace MagmaDb::Core;

Document::Document() : id(UUID::generateUUID()), properties()
{
}

Document::Document(std::string id) : id(std::move(id)), properties()
{
}
std::string Document::getId() const
{
	return id;
}

DocumentValue& Document::getProperty(const std::string& propertyName)
{
	return properties[propertyName];
}

DocumentValue& Document::operator[](const std::string& propertyName)
{
	return getProperty(propertyName);
}

void Document::setId(std::string newId)
{
	this->id = std::move(newId);
}
void Document::setProperty(const std::string& propertyName, DocumentValue& value)
{
	value.setKey(propertyName);
	properties[propertyName] = value;
}

void Document::operator[](const std::string& propertyName, DocumentValue& value)
{
	setProperty(propertyName, value);
}

void Document::removeProperty(const std::string& propertyName)
{
	properties.erase(propertyName);
}

void Document::clear()
{
	properties.clear();
}

Document::~Document() = default;
