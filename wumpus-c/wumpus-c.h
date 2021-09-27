#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A `UserHandle` is an opaque pointer an `UserData`, and so to an `Engine`.
 * This will be passed to C (and Java).
 */
typedef struct UserHandle_Engine UserHandle_Engine;

/**
 * **This is the line that will need to be changed.**
 */
typedef struct UserHandle_Engine WumpusUserHandle;

/**
 * create a [[UserData]]. This code is dependent on the app.
 * # Safety
 * data pointer must be valid
 */
WumpusUserHandle *new_engine(const char *config_json);

/**
 * deletes the main data structure. . This code is dependent on the app.
 * # Safety
 * data pointer must be valid
 */
void delete_engine(WumpusUserHandle *data);

/**
 * get the most recent string
 * # Safety
 * data pointer must be valid
 */
const char *last_string(WumpusUserHandle *data);

/**
 * execute an action (just wraps the engine call)
 * # Safety
 * data pointer must be valid
 */
void execute(WumpusUserHandle *data, const char *body);

/**
 * handle an event (just wraps the engine call)
 * # Safety
 * data pointer must be valid
 */
void handle_event(WumpusUserHandle *data, const char *body);

/**
 * creates the initial HTML  (just wraps the engine call)
 * # Safety
 * data pointer must be valid
 */
const char *initial_html(WumpusUserHandle *data);

/**
 * whether the response requires the application to be shut down
 * # Safety
 * data pointer must be valid
 */
bool is_shutdown_required(const WumpusUserHandle *data);

/**
 * creates the JSON-encoded response  (just wraps the engine call)
 * # Safety
 * data pointer must be valid
 */
const char *last_response_json(WumpusUserHandle *data);
