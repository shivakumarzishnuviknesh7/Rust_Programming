fn main() {
    println!("Hello, world!");

    let mut y = 4;
    println!("the value is {}", y);

    y = 4;
    println!("the value is {}", y);

    let x = 4;
    println!("the value is {}", x);

    let x = x + 1;
    println!("the value is {}", x);
    
    {
    	let x = 4;
    	println!("the value is {}", x);

    	let x = "hallow";
    	println!("the value is {}", x);
    }

    //constant
    const SECONDS: u32 = 60;
    println!("The seconds is  {}", SECONDS);

    //data types (Primitive Data Type)
    
    //scalar data type

    //i32 integer 32 signed
    let _x1: i32 = 2;

    //unsigned integer
    let _x2: u32 = 9;

    //f32 floating point 
    let _x3: f32 =10.1;

    //boolean
    let _xr: bool = true;

    //char
    let _x4: char = 'q';

{
	//compound data type

    //tuple(Immutable -not changeable)
    let tup:(i32, bool, char) = (1, true, 'a');
    

    println!("{}", tup.2);

}

{
	// array

	let mut arr:[i32;5] = [1,2,3,4,5];
	arr[4]= 2;
	println!("{}",arr[4]);

}
    


}
