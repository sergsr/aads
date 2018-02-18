import System.Environment (getArgs)

pe097 :: Integer -> Integer
pe097 = mod (28433 * (2^7830457) + 1) . (10^)

main = getArgs >>= print . pe097 . read . head
