# simetry

Rust library for interfacing with telemetry of various racing and driving sims.

The library is under active development, and the generic interface for
handling all sims is yet to be designed.

Currently supported sims:

* iRacing
* Assetto Corsa
* Assetto Corsa Competizione

All implementations provide a `Client` that generates its own `SimState`.

Besides that, iRacing provides a `DiskClient` for reading recorded telemetry data,
and `commands` for sending commands to iRacing.

Examples of capabilities are available in `examples`.
