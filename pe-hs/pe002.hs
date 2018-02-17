import System.Environment (getArgs)

-- | Sum of all even Fibonacci terms < input.
pe2 :: Integer -> Integer
pe2 n = sum . takeWhile (<n) $ filter even fibs
  where fibs = 0 : 1 : zipWith (+) fibs (drop 1 fibs)

main = getArgs >>= print . pe2 . read . head
