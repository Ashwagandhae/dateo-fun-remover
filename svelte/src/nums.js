function rng(seed) {
  // Given a seed, returns a pseudorandom number using a Lehmer RNG algorithm
  return (seed * 7 ** 5) % (2 ** 31 - 1);
}

export function getFiveNums(year, month, day) {
  // Given the year, month, and day, return a list of five random numbers
  // between -20 and 20 (excluding 0).

  // We create the seed using year, month, and day so that if two people open
  // the site on the same day, they both have the same numbers.
  var seed = day + 100 * year + 1_000_000 * month;

  // While the list of nums does not have 5 elements,
  var nums = [];
  var num;
  while (nums.length < 5) {
    seed = rng(seed);
    // Convert the seed to a number between 1 and 20 (inclusive)
    num = (seed % 20) + 1;
    // With a 1/3 probability, decide if the number is negative
    if (seed % 3 == 0) {
      num = -num;
    }
    // If that number isn't in the list, add it (avoids duplicate numbers)
    if (!nums.includes(num)) {
      nums.push(num);
    }
  }

  // Sort those numbers least to greatest
  nums.sort(function (a, b) {
    return a - b;
  });

  return nums;
}
