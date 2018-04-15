
use std::io::stdin;

fn main()
{
	let mut input = String::new();
	let mut output_string = String::new();
	let mut teens_flag: bool = false;

	println!( "Hello World, please give me a number of numbers: " );
	stdin().read_line(&mut input).unwrap();
	
	let string_length = input.len();
	input.truncate(string_length - 1);

	for ( i, c ) in input.chars().enumerate(){

		if i == 0 && c == '-'{output_string.push_str( "negative " ); continue; }
		if i == 0 && c == '+'{output_string.push_str( "positive " ); continue; }

		let natural_pos = input.len() - i; // Get the power of the digit.

		if ! c.is_numeric() && c != '-' && c != '+' {
			println!( " {} in {} contains is not a number, please try again with only numbers ", c, input );
			std::process::exit( 1 );
		}

		if natural_pos >= 25 {
			output_string.push_str( "Lots, please specify a number less than one septillion" );
			break;
		}

		if teens_flag == false {
			if ( natural_pos + 1 ) % 3 == 0 { // True if the number is a "ten"
				if c == '1'{ // If the number is between 11-19, skip to next char and print "teen"-number.
					teens_flag = true; 
				}
				else {
					tens( c, &mut output_string );
				}
			}else{ // Print normal numbers
				numbers( c, &mut output_string );
			}
		}else{
			teens( c, &mut output_string ); 
			teens_flag = false; 
		}

		multiplies( natural_pos, &mut output_string )
	}

	println!( "The number you specified: {}", input.trim() );
	println!( "Translation is : {}", output_string.trim() );
}

fn numbers( c:char, output_string: &mut String )
{
	match c {
		'1' => { output_string.push_str( "one " ); }
		'2' => { output_string.push_str( "two " ); }
		'3' => { output_string.push_str( "three " ); }
		'4' => { output_string.push_str( "four " ); }
		'5' => { output_string.push_str( "five " ); }
		'6' => { output_string.push_str( "six " ); }
		'7' => { output_string.push_str( "seven " ); }
		'8' => { output_string.push_str( "eight " ); }
		'9' => { output_string.push_str( "nine " ); }
		_   => { }
	}
}

fn tens( c: char, output_string: &mut String )
{
	match c {
		'1' => { output_string.push_str( "ten " ); }
		'2' => { output_string.push_str( "twenty " ); }
		'3' => { output_string.push_str( "thirty " ); }
		'4' => { output_string.push_str( "fourty " ); }
		'5' => { output_string.push_str( "fifty " ); }
		'6' => { output_string.push_str( "sixty " ); }
		'7' => { output_string.push_str( "seventy " ); }
		'8' => { output_string.push_str( "eighty " ); }
		'9' => { output_string.push_str( "ninety " ); }
		_   => { }
	}	
}

fn teens( c: char, output_string: &mut String )
{
	match c {
		'1' => { output_string.push_str( "eleven " ); }
		'2' => { output_string.push_str( "twelve " ); }
		'3' => { output_string.push_str( "thirteen " ); }
		'4' => { output_string.push_str( "fourteen " ); }
		'5' => { output_string.push_str( "fifteen " ); }
		'6' => { output_string.push_str( "sixteen " ); }
		'7' => { output_string.push_str( "seventeen " ); }
		'8' => { output_string.push_str( "eighteen " ); }
		'9' => { output_string.push_str( "nineteen " ); }
		_   => { }
	}	
}

fn multiplies( natural_pos: usize, output_string: &mut String )
{
	if natural_pos % 3 == 0 { output_string.push_str( "hundred " ); }

	match natural_pos{
		4  => { output_string.push_str( "thousand, " ); }
		7  => { output_string.push_str( "million, " ); }
		10 => { output_string.push_str( "billion, " ); }
		13 => { output_string.push_str( "trillion, " ); }
		16 => { output_string.push_str( "quadrillion, " ); }
		19 => { output_string.push_str( "quintillion, " ); }
		22 => { output_string.push_str( "sextillion, " ); }
		_  => { }
	}
}