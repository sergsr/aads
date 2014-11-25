import Data.Ratio
import BinTree (Tree, bfs, asymUnfold)

calkinWilf :: Tree Rational
calkinWilf = asymUnfold lf rf (1%1)
  where lf r    = numerator r % sumND r
        rf r    = sumND r     % denominator r
        sumND r = numerator r + denominator r

calkinWilfSeq = bfs calkinWilf

