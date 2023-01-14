# simetry

Rust library for interfacing with telemetry of various racing and driving sims.

The library is under active development, and the generic interface for
handling all sims is yet to be designed.

Current capabilities are available in `examples`. 

Currently supported sims:

## iRacing

The `iracing` module contains handling for sending commands to iRacing using `commands`, reading active
sim data using `Client`, or reading telemetry using `DiskClient`.
