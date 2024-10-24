# Emulate request outcomes

# DONT_EMULATE_EXECUTION_TIME: The action will be executed immediatelly
# EMULATE_EXACT_EXECUTION_TIME: The action will always take "emulate_execution_time" amount of time
# EMULATE_RANDOM_EXECUTION_TIME: The action will randomly take between 0 and "emulated_execution_time" amount of time
uint8 DONT_EMULATE_EXECUTION_TIME = 0
uint8 EMULATE_EXACT_EXECUTION_TIME = 1
uint8 EMULATE_RANDOM_EXECUTION_TIME = 2
uint8 emulate_execution_time
int32 emulated_execution_time # milliseconds

# DONT_EMULATE_FAILURE: The action will be execute succesfully every time
# EMULATE_FAILURE_ALWAYS: The action will always fail
# EMULATE_RANDOM_FAILURE_RATE: The action will randomly fail with a "emulated_failure_rate" rate
uint8 DONT_EMULATE_FAILURE = 0
uint8 EMULATE_FAILURE_ALWAYS = 1
uint8 EMULATE_RANDOM_FAILURE_RATE = 2
uint8 emulate_failure_rate
int32 emulated_failure_rate # percentage 0..100

# DONT_EMULATE_FAILURE_CAUSE: If the action fails, it wil fail with a generic "fail" cause
# EMULATE_EXACT_FAILURE_CAUSE: Specify why the exact reason why the action fails (takes the first from the "emulated_failure_cause" list)
# EMULATE_RANDOM_FAILURE_CAUSE: The action will fail and randomly choose a cause from the "emulated_failure_cause" list
uint8 DONT_EMULATE_FAILURE_CAUSE = 0
uint8 EMULATE_EXACT_FAILURE_CAUSE = 1
uint8 EMULATE_RANDOM_FAILURE_CAUSE = 2
uint8 emulate_failure_cause
string[] emulated_failure_cause # For example: ["violation", "timeout", "collision", etc.]