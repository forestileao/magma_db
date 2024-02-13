#include <iostream>
#include "core/Document.h"
#include "core/DocumentValue.h"

using MagmaDb::Core::Document;
using MagmaDb::Core::DocumentValue;
using MagmaDb::Core::DocumentValueType;

int main()
{
	Document document;
	DocumentValue helloValue("Hello, World!");
	DocumentValue another("Test");
	DocumentValue number(42);
	DocumentValue pi(3.14);
	DocumentValue boolean(true);
	DocumentValue array({helloValue, another, number, pi, boolean});
	DocumentValue none;

	Document anotherDocument("anotherId");
	DocumentValue anotherDocumentValue(anotherDocument);

	document.setProperty("anotherDocument", anotherDocumentValue);

	document.setProperty("hello", helloValue);
	document.setProperty("another", another);
	document.setProperty("number", number);
	document.setProperty("pi", pi);
	document.setProperty("boolean", boolean);
	document.setProperty("array", array);
	document.setProperty("none", none);

	// print all values
	std::cout
	<< "id: " << document.getId()
	<< "\nHello: " << document["hello"].getString()
	<< "\nAnother: " << document["another"].getString()
	<< "\nNumber: " << document["number"].getInteger()
	<< "\nPi: " << document["pi"].getDouble()
	<< "\nBoolean: " << document["boolean"].getBool()
	<< "\nInner Document Id: " << document["anotherDocument"].getDocument().getId()
	<< std::endl;
}
