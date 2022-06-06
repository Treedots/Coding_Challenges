function disemvowel(str) {
  return str.split(/[aeiou]+/gi).join('');
}