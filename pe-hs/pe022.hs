import System.Environment (getArgs)
import Data.Char (ord, toLower)
import Data.List (sort)

split :: String -> [String]
split = words . map replace . filter (/= '"')
  where replace ',' = ' '
        replace x   = x

charScore :: String -> Integer
charScore = sum . map (fromIntegral . offsetOrd)
  where offsetOrd c = ord c - ord 'A' + 1

pe022 :: [String] -> Integer
pe022 = sum . map (\(i, s) -> i * charScore s) . zip [1..] . sort

main = getArgs >>= readFile . head >>= print . pe022 . split
