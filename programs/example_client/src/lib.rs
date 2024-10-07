use wasm_client_anchor::create_program_client;
use wasm_client_anchor::create_program_client_macro;

create_program_client!(example_program::ID_CONST, ExampleProgramClient);
create_program_client_macro!(example_program, ExampleProgramClient);
example_program_client_request_builder!(Initialize, "optional:args");
example_program_client_request_builder!(Another);
