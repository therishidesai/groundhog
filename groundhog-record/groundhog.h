#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum EventType {
  SEND = 1,
  RECV,
  TRACE,
};
typedef uint32_t EventType;

void hello(void);

void init(const char *node_name);

void log_event(const char *node_name, const EventType *event_type, const char *data);
