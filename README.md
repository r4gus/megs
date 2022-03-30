# Modular and Extensible Gate Simulator

The basic idea is to encapsulate each component (gates, input controlls, output controlls, ...)
using [WebAssembly](https://webassembly.org/) modules. Every module is responsible for:

* updating its state
* and drawing its shape using imported functions on request.

The goal is to remove the necessity to recompile the whole application if one
wants to add a new component to the application. If done correctly this should
also reduce the complexity of the application.

The app itself is only responsible for:

* detecting all modules.
* creating new instances of modules on request.
* provide IO.
* ...
