initSidebarItems({"mod":[["check_duplication","This module implements a checker for verifying that each vector in a CompiledModule contains distinct values. Successful verification implies that an index in vector can be used to uniquely name the entry at that index. Additionally, the checker also verifies the following: - struct and field definitions are consistent - the handles in struct and function definitions point to the self module index - all struct and function handles pointing to the self module index have a definition"],["code_unit_verifier","This module implements the checker for verifying correctness of function bodies. The overall verification is split between stack_usage_verifier.rs and abstract_interpreter.rs. CodeUnitVerifier simply orchestrates calls into these two files."],["constants","This module implements a checker for verifying that - a constant's type only refers to primitive types - a constant's data serializes correctly for that type"],["control_flow","This module implements a checker for verifies control flow. The following properties are ensured: - All forward jumps do not enter into the middle of a loop - All \"breaks\" (forward, loop-exiting jumps) go to the \"end\" of the loop - All \"continues\" (back jumps in a loop) are only to the current loop"],["control_flow_graph","This module defines the control-flow graph uses for bytecode verification."],["dependencies","This module contains the public APIs supported by the bytecode verifier."],["instantiation_loops","This implements an algorithm that detects loops during the instantiation of generics."],["instruction_consistency","This module defines the transfer functions for verifying type safety of a procedure body. It does not utilize control flow, but does check each block independently"],["resources","This module implements a checker for verifying that a non-resource struct does not have resource fields inside it."],["signature","This module implements a checker for verifying signature tokens used in types of function parameters, locals, and fields of structs are well-formed. References can only occur at the top-level in all tokens.  Additionally, references cannot occur at all in field types."],["struct_defs","This module provides a checker for verifying that struct definitions in a module are not recursive. Since the module dependency graph is acylic by construction, applying this checker to each module in isolation guarantees that there is no structural recursion globally."],["verifier","This module contains the public APIs supported by the bytecode verifier."]]});