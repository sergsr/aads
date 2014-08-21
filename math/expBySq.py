def fastPowerIsh( n, e ):
  """Exponentiation by squaring with exponent bit shifting"""
  result = 1
  multiplier = n
  while e > 0:
    if e & 1 != 0:
      result *= multiplier
    multiplier *= multiplier
    e = e >> 1
  return result

