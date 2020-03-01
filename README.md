# Strigifies all values in an XML file

This library only focuses on the value of the XML, and it's main intended
use is to allow searching for values in an XML file whithout accedentially
matching on keys, attribute names or attribute values.

```xml
<outertag attribute1="hello">
<innertag>Hello world</innertag>
<\outertag>
```

Parses into:

```
Hello world
```