# PrimeNumbersInRust
This program is a very basic level program uses closure and give you the use of closure.

###### Std::io;
`use std::io` is used for the take a library form the rust origins


After that there are 2 `println` lines are mention that simply prints these lines on console

###### line no 9 to 11 of main.rs file
```
    let mut y = String::new();
    io::stdin().read_line(&mut y);
    let y:i32 = y.trim().parse().unwrap();
 ```    
 these line are used to take user input must be integer if it is not integer the `rust` will pe panicked
 
 ### DEFINING CLOSURE
 
 
 ###### line no 14 of main.rs file
 `let condition = |x:i32|` the `x:i32` written in straight line ` |   | ` these lines are called closure these are a
type of function without any name and it make `condtion` a name of a veriable and **function** now.


