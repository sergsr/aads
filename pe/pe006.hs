import System.Environment (getArgs)

pe6 :: Int -> Int
pe6 n | n < 1     = 0
      | otherwise = ((n*(n + 1)) `quot` 2)^2 - (n*(n + 1)*(2*n + 1) `quot` 6)

main = getArgs >>= print . pe6 . read . head
