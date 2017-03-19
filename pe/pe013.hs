import System.Environment (getArgs)

-- | First 10 digits of sum of list of numbers
pe13 :: [Integer] -> String
pe13 = take 10 . show . sum

main = getArgs >>= readFile . head >>= putStrLn . pe13 . map read . lines
