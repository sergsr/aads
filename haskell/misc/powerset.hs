powerset :: [a] -> [[a]]
powerset x:xs = powerset xs ++ map (x:) (powerset xs)
