import System.Environment (getArgs)

-- TODO: change [2..] to a wheel
-- | Largest prime divisor of input.
pe3 :: Integer -> Integer
pe3 n | abs n < 4 = abs n
pe3 n = last $ takeWhile (/= 1) $ scanl f (abs n) [2..]
  where f a x = until (\y -> mod y x /= 0) (`quot` x) a

main = getArgs >>= print . pe3 . read . head
