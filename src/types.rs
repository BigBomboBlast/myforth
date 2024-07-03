use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

// this file defines the operations and the type system 
// The stack can hold these number types
// Floats, Integers, and usize (so memory addresses work easy)

// any positive number is automatically a usize
// operations that result in floats or negative numbers are handled accordingingly


#[derive(Copy, Clone, Debug)]
pub enum Type { // we have three types, Unsigned, Signed, and Floats
    Signed(i64),
    Float(f64),
    Unsigned(usize), // i geuss `0` is included as a positive number in this language
    // since by "positive" I really mean "unsigned number"
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Signed(n) => write!(f, "{}", n),
            Type::Float(n) => write!(f, "{}", n),
            Type::Unsigned(n) => write!(f, "{}", n),
        }
    }
}

// defining how adding and subtracting work in the type system
// it covers ever single possible pair of types
// im thinking of signed as negative numbers, and unsigned as positive numbers
// since it helps me reason about what the results should be
impl Add for Type {
    type Output = Type;
    fn add(self, other: Type) -> Type {
        match (self, other) {
            (Type::Signed(neg1), Type::Signed(neg2)) => {
                let result = neg1 + neg2;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Signed(neg), Type::Float(float)) => Type::Float((neg as f64) + float),
            (Type::Signed(neg), Type::Unsigned(pos)) => {
                let result = neg + (pos as i64);
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Float(float), Type::Signed(neg)) => Type::Float(float + (neg as f64)),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 + float2),
            (Type::Float(float), Type::Unsigned(pos)) => Type::Float(float + (pos as f64)),

            (Type::Unsigned(pos), Type::Signed(neg)) => {
                let result = (pos as i64) + neg;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Unsigned(pos), Type::Float(float)) => Type::Float((pos as f64) + float),
            (Type::Unsigned(pos1), Type::Unsigned(pos2)) => Type::Unsigned(pos1 + pos2),
        }
    }
}

impl Sub for Type {
    type Output = Type;
    fn sub(self, other: Type) -> Type {
        match (self, other) {
            (Type::Signed(neg1), Type::Signed(neg2)) => {
                let result = neg1 - neg2;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Signed(neg), Type::Float(float)) => Type::Float((neg as f64) - float),
            (Type::Signed(neg), Type::Unsigned(pos)) => {
                let result = neg - (pos as i64);
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Float(float), Type::Signed(neg)) => Type::Float(float - (neg as f64)),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 - float2),
            (Type::Float(float), Type::Unsigned(pos)) => Type::Float(float - (pos as f64)),

            (Type::Unsigned(pos), Type::Signed(neg)) => {
                let result = (pos as i64) - neg;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Unsigned(pos), Type::Float(float)) => Type::Float((pos as f64) - float),
            (Type::Unsigned(pos1), Type::Unsigned(pos2)) => {
                let result = (pos1 as i64) - (pos2 as i64);
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
        }
    }
}

impl Mul for Type {
    type Output = Type;
    fn mul(self, other: Type) -> Type {
        match (self, other) {
            (Type::Signed(neg1), Type::Signed(neg2)) => {
                let result = neg1 * neg2;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Signed(neg), Type::Float(float)) => Type::Float((neg as f64) * float),
            (Type::Signed(neg), Type::Unsigned(pos)) => {
                let result = neg * (pos as i64);
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Float(float), Type::Signed(neg)) => Type::Float(float * (neg as f64)),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 * float2),
            (Type::Float(float), Type::Unsigned(pos)) => Type::Float(float * (pos as f64)),

            (Type::Unsigned(pos), Type::Signed(neg)) => {
                let result = (pos as i64) * neg;
                if result < 0 {
                    Type::Signed(result)
                } else {
                    Type::Unsigned(result as usize)
                }
            }
            (Type::Unsigned(pos), Type::Float(float)) => Type::Float((pos as f64) * float),
            (Type::Unsigned(pos1), Type::Unsigned(pos2)) => Type::Unsigned(pos1 * pos2),
        }
    }
}

impl Div for Type {
    type Output = Type;
    fn div(self, other: Type) -> Type {
        match (self, other) {
            (Type::Signed(neg1), Type::Signed(neg2)) => Type::Float((neg1 / neg2) as f64),
            (Type::Signed(neg), Type::Float(float)) => Type::Float(neg as f64 / float),
            (Type::Signed(neg), Type::Unsigned(pos)) => Type::Float(neg as f64 / pos as f64),

            (Type::Float(float), Type::Signed(neg)) => Type::Float(float / neg as f64),
            (Type::Float(float1), Type::Float(float2)) => Type::Float(float1 / float2),
            (Type::Float(float), Type::Unsigned(pos)) => Type::Float(float / pos as f64),

            (Type::Unsigned(pos), Type::Signed(neg)) => Type::Float(pos as f64 / neg as f64),
            (Type::Unsigned(pos), Type::Float(float)) => Type::Float(pos as f64 / float),
            (Type::Unsigned(pos1), Type::Unsigned(pos2)) => Type::Float(pos1 as f64 / pos2 as f64),
        }
    }
}

impl PartialEq for Type {
    // NOTE: this takes references to the values in a comparison, 
    // that way I don't have to think about whether `==` takes ownership of the values
    // sounds like it'd be janky if it did
    fn eq(&self, other: &Type) -> bool {
        // self is the left value type, other is the right value type in an equality check
        match (self, other) {
            (Type::Signed(left), Type::Signed(right)) => left == right,
            // you have to dereference in order to do the type conversion
            // otherwise I believe you are converting the literal reference and not the value
            (Type::Signed(left), Type::Float(right)) => (*left as f64) == *right,
            (Type::Signed(left), Type::Unsigned(right)) => *left == (*right as i64),

            (Type::Float(left), Type::Signed(right)) => *left == (*right as f64),
            (Type::Float(left), Type::Float(right)) => left == right,
            (Type::Float(left), Type::Unsigned(right)) => *left == (*right as f64),

            (Type::Unsigned(left), Type::Signed(right)) => (*left as i64) == *right,
            (Type::Unsigned(left), Type::Float(right)) => (*left as f64) == *right,
            (Type::Unsigned(left), Type::Unsigned(right)) => left == right,
        }
    }
}

impl PartialOrd for Type {
    // same thing, takes references to the values in the comparison
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        // partial_cmp returns an Option<Ordering>
        // Ordering is an enum that contains if the left value was greater than/less than... whatever
        match (self, other) {
            (Type::Signed(left), Type::Signed(right)) => left.partial_cmp(right),
            (Type::Signed(left), Type::Float(right)) => (*left as f64).partial_cmp(right),
            // dereferenes to the the type conversion, then the partial_cmp will take reference to
            // the converted value
            (Type::Signed(left), Type::Unsigned(right)) => left.partial_cmp(&(*right as i64)),

            (Type::Float(left), Type::Signed(right)) => left.partial_cmp(&(*right as f64)),
            (Type::Float(left), Type::Float(right)) => left.partial_cmp(right),
            (Type::Float(left), Type::Unsigned(right)) => left.partial_cmp(&(*right as f64)),

            (Type::Unsigned(left), Type::Signed(right)) => (*left as i64).partial_cmp(right),
            (Type::Unsigned(left), Type::Float(right)) => (*left as f64).partial_cmp(right),
            (Type::Unsigned(left), Type::Unsigned(right)) => left.partial_cmp(right),
        }
    }
}
