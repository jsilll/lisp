# Compute the xth fibonacci number.
fn fib(x) {
  if (x < 3) {
    1
  } else {
    fib(x - 1) + fib(x - 2)
  }
}

# This expression will compute the 40th number.
fn main() {
  fib(40)
}
