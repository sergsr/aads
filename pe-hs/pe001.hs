import System.Environment (getArgs)

-- | Sum of all multiples of 3 and 5 from 0 to input
pe001 :: Int -> Int
pe001 n = sumMults 3 + sumMults 5 - sumMults 15
  where sumMults k = case n `mod` k of
          0 -> k * arithmetic (n `quot` k - 1)
          _ -> k * arithmetic (n `quot` k)
        arithmetic n | n > 0     = (n * (n + 1)) `quot` 2
                     | otherwise = 0

main = getArgs >>= print . pe001 . read . head
