/// This current value could be simplified as String.
///
/// It is needed to write this value in external system (like log file which
/// will be read by another program, or web API), and this external system
/// probably not know this type value.
///
/// It is like serializing this value, but without that the external system can
/// deserialize it. So the value should be simplified as String, to be generic.
///
/// Numerical values (like `i32`, `f32`, or `bool`) will be casted in String,
/// but external system should be able to detect them and cast them in its own
/// numerical system (if any).
///
/// # Example
///
/// See an example of implementation of `ValueAsString` on `enum Level` in
/// [/examples/full_demo.rs](https://github.com/Jimskapt/charlie_buffalo/blob/master/examples/full_demo.rs)
/// in source repository.
///
/// # See also
///
/// [`Attribute`](struct.Attribute.html)
pub trait ValueAsString {
	fn as_string(&self) -> String;
}

// IMPLEMENTATION ON SOME PRIMITIVES :

impl ValueAsString for String {
	fn as_string(&self) -> String {
		format!(r#""{}""#, self.replace("\"", "\\\""))
	}
}

impl ValueAsString for Vec<String> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(",");
		result += "]";

		return result;
	}
}

impl ValueAsString for &str {
	fn as_string(&self) -> String {
		format!(r#""{}""#, self.replace("\"", "\\\""))
	}
}

impl ValueAsString for str {
	fn as_string(&self) -> String {
		format!(r#""{}""#, self.replace("\"", "\\\""))
	}
}

impl ValueAsString for Vec<&str> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(",");
		result += "]";

		return result;
	}
}

impl ValueAsString for char {
	fn as_string(&self) -> String {
		if *self == '\'' {
			String::from("'\''")
		} else {
			format!("'{}'", self)
		}
	}
}

impl ValueAsString for Vec<char> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(",");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&char> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(",");
		result += "]";

		return result;
	}
}

impl ValueAsString for bool {
	fn as_string(&self) -> String {
		if *self {
			return String::from("true");
		} else {
			return String::from("false");
		}
	}
}

impl ValueAsString for u8 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &u8 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<u8> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&u8> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for u16 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &u16 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<u16> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&u16> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for u32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &u32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<u32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&u32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for u64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &u64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<u64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&u64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for u128 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &u128 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<u128> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&u128> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for usize {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &usize {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<usize> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&usize> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for i8 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &i8 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<i8> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&i8> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for i16 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &i16 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<i16> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&i16> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for i32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &i32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<i32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&i32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for i64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &i64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<i64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&i64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for i128 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &i128 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<i128> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&i128> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for isize {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &isize {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<isize> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&isize> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for f32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &f32 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<f32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&f32> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for f64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for &f64 {
	fn as_string(&self) -> String {
		format!("{}", self)
	}
}

impl ValueAsString for Vec<f64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}

impl ValueAsString for Vec<&f64> {
	fn as_string(&self) -> String {
		let mut result = String::from("[");
		result += &self
			.iter()
			.map(|i| i.as_string())
			.collect::<Vec<String>>()
			.join(", ");
		result += "]";

		return result;
	}
}
