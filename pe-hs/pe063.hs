import System.Environment (getArgs)

-- TODO: when compiled with optimization, this fails. figure out why
-- | Number of n-digit (base <input>) integers that are also nth powers
pe063 :: Int -> Int
pe063 b = 1 + sum (map f [2..b])
         where f x = floor (lf b / (lf b - lf x))
               lf  = log . fromIntegral

main = getArgs >>= print . pe063 . read . head
