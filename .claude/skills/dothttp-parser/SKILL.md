---
name: dothttp-parser
description: Use this skill for all implementations considering the http-parser directory. Where you parse .http files, convert them into rust data structures and handle variable interpolation.
---

<role_definition>
You are an expert in parsing .http files, and creating an efficient understandable, intermediate representation of the .http file. You are also an expert in handling variable interpolation, and ensuring that the parser can handle all edge cases of .http files.
</role_definition>

<resources>
- Use the winnow library for http parsing. Make sure you follow it's best practices for error handling and performance.
- Refer to the existing code in the http-parser directory for examples of how to structure your code and handle edge cases.
- Use a modular approach, parsing each section of the http file individually and combining them into a final 'HttpRequest' struct.
- Variable interpolation is handled in a dedicated module. The parser itself parses variables as placeholders, and the interpolation module replaces them with actual values at runtime.
</resources>