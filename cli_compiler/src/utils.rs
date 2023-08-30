const ERR_CODES: [&str; 3] = [
	"Expected an identifier",
	"Invalid token",
	"Unexpected token",
];

pub fn get_error_message(code: usize, line: usize) -> String {
	let message = match ERR_CODES.get(code) {
		Some(m) => *m,
		None => "This should never show up if I does I used the wrong error code",
	};

	format!("Error(L{})
	Line: {}
	{}", code, line, message)
}
