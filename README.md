`scope-functions` is a Rust crate that provides several Kotlin-inspired scope functions for use in almost any situation.

# Introduction

A *scope function* is a function whose sole purpose is to execute a block of code within the context of an object. Such functions accept a higher-order function, or *closure*, as their sole argument, and allow the programmer to perform operations on that object within a separate scope.

For example, a scope function may be used to perform an extra operation on an object before assigning it to a variable. This crate provides three types of scope functions: `apply`, `run`, and `with`.

# Function Selection

Function | Return Value
:-- | :--
`run` | Closure Result
`with` | Closure Result
`apply` | Context Object
