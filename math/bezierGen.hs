-- | Simple, fragile program to generate bezier curve graphs
module Main where
import Bezier
import Data.Complex
import Data.List.Split
import System.Environment (getArgs)

-- | Reads list of float literals as rectangular coords for complex numbers
readPts :: [String] -> [Complex Float]
readPts ps = zipWith (:+) (map head pairs) (map last pairs)
  where pairs = chunksOf 2 $ map read ps

-- | Turns list of control points for a Bezier curve into its graph
genPts :: [Complex Float]-> [Complex Float]
genPts ss = map (recBez ss) [0.00,0.01..1.00]

-- | Stringifies list of complex numbers as space delimited rectangular form
printPts :: [Complex Float] -> [String]
printPts = map (\z -> show (realPart z) ++ ' ' : show (imagPart z))

main = getArgs >>= mapM_ putStrLn . printPts . genPts . readPts


