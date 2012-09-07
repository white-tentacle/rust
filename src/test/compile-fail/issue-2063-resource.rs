// test that autoderef of a type like this does not
// cause compiler to loop.  Note that no instances
// of such a type could ever be constructed.
struct t { //~ ERROR this type cannot be instantiated
  x: x,
  to_str: (),
}

enum x = @t; //~ ERROR this type cannot be instantiated

fn main() {
}
