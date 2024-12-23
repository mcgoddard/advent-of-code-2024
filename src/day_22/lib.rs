pub fn mix(secret: &i64, temp: i64) -> i64 {
  secret ^ temp
}

pub fn prune(secret: i64) -> i64 {
  secret % 16777216
}

pub fn next_secret(secret: i64) -> i64 {
  let mut secret = secret;
  let temp = secret * 64;
  secret = prune(mix(&secret, temp));
  let temp = secret / 32;
  secret = prune(mix(&secret, temp));
  let temp = secret * 2048;
  secret = prune(mix(&secret, temp));
  secret
}
