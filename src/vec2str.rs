fn main() {
  let v = vec!["hello", "world"];
  println!();

  const THREE_SPACES: &str = "   ";

  // Join with a separator
  let s1: String = v.join(" "); // "hello world"
  println!("{} s1: {}", THREE_SPACES, s1);

  // Join with no separator
  let s2: String = v.join(""); // "helloworld"
  println!("{} s2: {}", THREE_SPACES, s2);
}
