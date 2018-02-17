import Data.Char (digitToInt)
import System.Environment (getArgs)

-- | Sum of digits of 2^input
pe016 :: Integer -> Integer
pe016 = sum . map (fromIntegral . digitToInt) . show . (2^)

main = getArgs >>= print . pe016 . read . head
