import System.Environment (getArgs)
import Data.List (foldl1')

-- | Calculate LCM of numbers from 0 to input
pe5 :: Int -> Int
pe5 = (!!) (0 : scanl1 lcm [1..])

main = getArgs >>= print . pe5 . read . head
