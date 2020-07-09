


Marbles is a Proof of concept code snippet from one of my most recent project: Rusty Marbles.
Rusty marbles is a re-write of Marble Framework, which is a string obfuscation framework developed by the Central Intelligence Agency.





### What is Marble Framework?


Marble framework is a string obfuscation framework written by the Central Intelligene Agency to be used in their projects. This framework 
were leaked to the public in 2017, when Wikileaks released information about CIA's hacking tools and exploits, in a series called Vault7. Later, the source code of this framework were released, showing how the agency would employ anti-forensic techniques in their malware. 

This framework would ensure that the strings in a binary is properly hidden or made unreadable to whomever that are attempting analysing it. Prior to compilation, marble would analyze a header file (.h) and select which algorithms that the user have chosen. Further, the source files of the project which are to be compiled is analyzed for string values which are represented in C++, either as a character string (CARBLE) or a wide-character string (WARBLE). All pre-compilation steps are handled by a module, called Mibster and a rust implementation of this could be found here: [Rust-Mibster](TODO)

More information about the framework can be found here: https://wikileaks.org/ciav7p1/cms/page_14588467.html


### What is Rusty Marbles? 

Rusty Marbles aims to take the ideas of Marble Framework and apply it to the Rust Programming Language. This would entail that a user would have the option 
to declare strings within their code that could be hidden from static binary analysis. 

This code represents only a portion of the completed code and demonstrates how a string would be obfuscated, using a corresponding C++ obfuscation method, but in Rust. 



### How do i run this code? 

In order to run this code, I have supplied a few test functions alongside two test examples, which corresponds to one of the original Marbles found in the CIA's developed framework. The code supplied in this example corresponds to the source code file "MBL_CLASS_BUMP1.cpp" 


```bash
user@host #  cargo run -- --nocapture 
```


### Disclaimer

This code is intended to serve as an example of an obfuscation algorithm for Malware binaries written in rust. 
I must declare that I do not in any way, shape or form condone the use of this code or any iteration of, to be used in projects that sets out to break the law. 
I hereby tender to the notion that I, as the author of this re-write, distances myself from anyone who advocates 
for the use of this project, with the intent to do harm or commit a crime. 
