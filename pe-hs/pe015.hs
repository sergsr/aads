import System.Environment (getArgs)

-- | # of paths (right, down moves only) top left to bottom right of square grid
pe015 :: Integer -> Integer
pe015 k = product [k + 1 .. 2*k] `quot` product [1..k]

main = getArgs >>= print . pe015 . read . head
