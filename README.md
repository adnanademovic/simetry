# simetry

Rust library for interfacing with telemetry of various racing and driving sims.

The library is under active development, and the generic interface for
handling all sims is yet to be designed.

Currently supported sims:

* iRacing
* Assetto Corsa
* Assetto Corsa Competizione
* rFactor 2
* DiRT Rally 2.0

All implementations provide a `Client` that generates its own `SimState`.

Besides that, iRacing provides a `DiskClient` for reading recorded telemetry data,
and `commands` for sending commands to iRacing.

Examples of capabilities are available in `examples`.

The most generic way of use is using `simetry::connect` to connect to whatever
sim is currently running with a generic client, and querying that way.
