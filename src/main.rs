use std::io;

fn main() {
    // println!("Hello, world!");
    println!("This program gives the information about the program that whether the given number is prime or composite");
    
    println!("PLEASE ENTER INTEGER");

    let mut y = String::new();
    io::stdin().read_line(&mut y);
    let y:i32 = y.trim().parse().unwrap();


    let condition = |x:i32|{
        if x % 2 ==0 && x > 2{
            println!("The number is not prime");
        }
        else if x % 3 ==0 && x > 3{
            println!("The number is not prime");
        }
        else if x % 5 ==0 && x > 5{
            println!("The number is not prime");
        }
        else if x % 7 ==0 && x > 7{
            println!("The number is not prime");
        }
        else {
            println!("The number is prime");
        }
    };
    println!("{:?}",condition(y));


    // let h = |g: bool|{
    //     if g {
    //         if let j % 2 ==0 && j > 2{
    //             println!("The number is not prime");
    //         };
    //         else if j % 3 ==0 && j > 3{
    //             println!("The number is not prime");
    //         }
    //         else if j % 5 ==0 && j > 5{
    //             println!("The number is not prime");
    //         }
    //         else if j % 7 ==0 && j > 7{
    //             println!("The number is not prime");
    //         }
    //         else {
    //             println!("The number is prime");
    //         }
    //     } 
    //     else {
    //         false
    //     }
    // };

    // let mut prime =2;
    // for i in 3..prime{
    //     if prime % i ==0 {
    //         println!("The number is not prime");
    //         break;
    //     }
    //     else {
    //         println!("This is prime");
    //         break;
    //     }
    // }
    // println!("{}",prime);
    // const LIMIT:u64=1000000;
    // let mut primes=vec!(2u64);

	// 'outer: for i in (3..LIMIT)
	// {
	// 	for n in primes.iter().take_while(|n|*n**n<=i)
	// 	{
	// 		if i%n==0
	// 		{
	// 			continue 'outer;
	// 		}
	// 	}
	// 	primes.push(i);
	// }

	// println!("count: {}",primes.len());
}
