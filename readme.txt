TITLE: Rust Warm-Up Gauntlet — 50 Mini Challenges
DESCRIPTION: A hands-on journey from “hello world” to real Rust fundamentals.
RULE: Type every line yourself. Avoid cloning unless absolutely forced.

–––––––––––––––––––––––––––
PHASE: BASIC IO & NUMBERS

1. Print “Hello Rustacean!” ✅
2. Read a number from stdin and print it back ✅
3. Parse a string to integer with `parse()` ✅
4. Add two numbers from stdin ✅
5. Multiply two floats from stdin ✅
6. Check if a number is even or odd
7. Generate Fibonacci up to N terms
8. Check if a number is prime
9. List all primes below 100
10. Compute factorial recursively
11. Armstrong number checker
12. Reverse a string
13. Count vowels in a string
14. Palindrome checker (both number & string)

–––––––––––––––––––––––––––
PHASE: OWNERSHIP & BORROWING
15. Function that takes a String and returns its length (ownership moves)
16. Borrow a string without moving ownership (`&str`)
17. Take a mutable borrow and modify the string
18. Return references safely from a function
19. Explore variable shadowing inside blocks

–––––––––––––––––––––––––––
PHASE: COLLECTIONS & ITERATORS
20. Sum all elements in a vector
21. Filter even values from a vector
22. Compute max/min in a vector without using built-ins
23. Use `.map()` and `.collect()` to square numbers
24. Count frequency of characters using `HashMap`
25. Remove duplicates while preserving order
26. Convert vector of ints to vector of strings using iterators

–––––––––––––––––––––––––––
PHASE: ERROR HANDLING
27. Read a file safely using `Result`
28. Write text to a file safely using `Result`
29. Parse integers from a file, skip invalid lines
30. Create your own simple error enum
31. Use the `?` operator instead of manual match

–––––––––––––––––––––––––––
PHASE: ENUMS & MATCH
32. Create a `TrafficLight` enum, print behavior for each variant
33. Rock-Paper-Scissors game using enum + match
34. Mini state machine: Idle → Running → Stopped
35. Implement a basic calculator using enum commands

–––––––––––––––––––––––––––
PHASE: STRUCTS, TRAITS & GENERICS
36. Create a struct `Book`, add a `new()` constructor function
37. Implement a `summary(&self)` method
38. Trait `Describable` and implement it for multiple structs
39. Write a generic function that works for multiple numeric types
40. Implement Display (`fmt::Display`) for your struct

–––––––––––––––––––––––––––
PHASE: TINY CLI UTILITIES
41. Simple TODO list (store tasks in a vector)
42. Extend TODO list: save tasks to a file
43. Word frequency counter for a text file
44. Read command-line args and print them
45. Command-line calculator (add, subtract, multiply, divide)

–––––––––––––––––––––––––––
PHASE: INTRO TO CONCURRENCY & ASYNC
46. Spawn two threads that do independent work
47. Shared counter using `Arc<Mutex<i32>>`
48. Download multiple URLs concurrently (reqwest + tokio)
49. Simulate a timer using `tokio::time::sleep`
50. Build a mini producer/consumer program using channels

–––––––––––––––––––––––––––
GOALS OF THIS GAUNTLET
• Understand ownership & borrowing
• Learn pattern matching and enums
• Use trait systems without fear
• Parse errors like an adult, not panic
• Read & write files confidently
• Dip toes into threads and async

–––––––––––––––––––––––––––
OPTIONAL RULES TO LEVEL UP
• Avoid `.clone()` unless truly necessary
• Prefer `match` over long `if/else`
• Add tests for everything logic-heavy
• Rewrite early problems using iterators once you learn them
• Try async equivalents of file/network tasks later

–––––––––––––––––––––––––––
NEXT STEPS AFTER 50
• CLI tools with real data persistence
• Mini blockchain ledger
• Networking + sockets
• Substrate or Solana smart contract projects

