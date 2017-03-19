import System.Environment (getArgs)
import Data.Char (digitToInt)

-- | Sum of the digits of factorial of input
pe020 :: Integer -> Integer
pe020 = sumDigits . factorial
  where sumDigits = sum . map (fromIntegral . digitToInt) . show
        factorial n = product [1..n]

main = getArgs >>= print . pe020 . read . head
