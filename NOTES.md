# groundhog

A record/replay system for multi-node/process systems.

### groundhog-record

- Expose a simple C api with two functions
  - init(str node_name)
	- initializes the sqlite db
  - log(timestamp, data)
	- writes the data to the initialized sqlite db with the timestamp as the key
		- timestamp may be an unnecessary input
- Every node/process in the system links to this C library and call's log on the things they want to record
- e.g: IPC, log data sent and when it was sent. log data received and when it was received
- Instead of a sqlite DB maybe try just an append only log file

### groundhog-bundle

- In a system with multiple nodes the recording library will generate a sqliteDB/logfile for each node
- groundhog bundle is a single command to coalesce the separate db's into one useful timeseries db

### groundhog-replay

- Takes a bundle file and replays it
- visualizers can now be written to view IPC traffic graphs, and all raw logs of the system during specific time stamps
