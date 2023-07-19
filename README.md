# simetry

Rust library for interfacing with telemetry of various racing and driving sims.

The library is under active development, and the generic interface for
handling all sims is yet to be designed.

Currently supported sims:

* iRacing
* Assetto Corsa
* Assetto Corsa Competizione
* rFactor 2 (extra steps for enabling described below)
* DiRT Rally 2.0
* Euro Truck Simulator 2 (extra steps for enabling described below)
* American Truck Simulator (extra steps for enabling described below)

Beyond that, an interface for a generic HTTP server exists, allowing you to easily emulate any sim
to see the functionality of your software that way, instead of having to run the sim and replicating
scenarios manually.

All implementations provide a `Client` which retries connections forever with `Client::connect()` and
generates its own `SimState`.

Besides that, iRacing provides a `DiskClient` for reading recorded telemetry data,
and `commands` for sending commands to iRacing.

Examples of capabilities are available in `examples`.

The most generic way of use is using `simetry::connect` to connect to whatever
sim is currently running with a generic client, and querying that way.

That generic way currently only supports some basic capabilities, and if you need more than what's
present there, you can use the interface of the individual sims.

## Extra Configuration

### rFactor 2

Requires adding the DLLs from https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin.

### Euro Truck Simulator 2 and American Truck Simulator

Requires adding the DLLs from https://github.com/RenCloud/scs-sdk-plugin.

Alternatively supporting also https://github.com/Funbit/ets2-telemetry-server
via `simetry::truck_simulator::json_client::Client`.
