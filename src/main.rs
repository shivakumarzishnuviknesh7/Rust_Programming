use std::io;

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
    
{
	println!("input and output in console");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("failed to read Line");
	println!("{}",input );

}

{
	let x =9;
	let y = 10;
	println!("{}",x + y);
}


{
	let x: i8 =9; //  0 to 255
	let y: i8 = 10;
	let z = x - y;
	println!("{}",z);
}

{
	let x = 9 as f32; 
	let y: f32 = 10.2;
	let z = x + y;
	println!("{}",z);
}

{
	println!("integer adding in console");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("check");

	let int_input: i64 = input.trim().parse().unwrap();

	println!("{}", int_input + 3);

}

// conditions and control flow
{
	let cond = 2 < 3;
	println!("{}", cond );
}

{
	let cond = (2 as f32) <=  3.3;
	println!("{}", cond );	
}

{
	// and operator
	let cond = true && true;
	println!("{}", cond );	
}


{
	//or operator
	let cond = true || false;
	println!("{}", cond );	
}


{
	// not operator
	let cond = !(true && true);
	println!("{}", cond );	

	let condition = true && !true;
	println!("{}", condition );	
}

{
	// if condition

	let food = "pizza";
	if food == "pizza" 
	{
		println!("condition satisfied");
	} else 
	{
		println!("else part");
	}

	if food != "pizza" 
	{
		println!("condition satisfied");
	} else 
	{
		println!("else part");
	}

}
{
	//else if

	let food1 = "bread";

	if food1 == "bread" 
	{
		println!("condition satisfied");
	} else if food1 == "bread"
	{
		println!("else if part");
	} else {
		println!("else part");
	}
}

	// functions
{

	fn test_one(){
		println!("test_one is called");

	}
	test_one();
}

fn add_number(x: i32 , y:i32){
	println!("{}",x + y );
}

add_number(1,2);

// statement returns nothing so _q is a statement
let _q = 10; 

//expression returns a value so num is an expression
let	num = {
	let x = 3;
	x + 1
};

println!("{}",num );

{

	let result = addno(20,20);
	println!("{}",result);

	fn addno(x: i32, y:i32) -> i32{
	x + y //no semicolon because of expression

}
{


	let result = addno(30,20);
	println!("{}",result);

	fn addno(x: i32, y:i32) -> i32{
	return x + y;
	}
}


{


	let result = addno(1,3);
	println!("{}",result);

	fn addno(x: i32, y:i32) -> i32{
	let result = x + y;
	if result > 10{
		return result -10;
	}
	result
	}
}








// cargo run  - to run the code
}

}
