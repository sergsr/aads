import System.Environment (getArgs)

pe048 :: Integer -> Integer
pe048 n = sum (map (\x -> x^x) [1..n]) `mod` 10^10

main = getArgs >>= print . pe048 . read . head
