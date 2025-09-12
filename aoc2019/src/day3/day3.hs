import Data.List (nub)
import Data.Set qualified as Set
import Data.Void
import System.IO
import Text.Read

main :: IO ()
main = do
  content <- readFile "../../../inputs/2019/03.txt"
  let input = inputParser content
  putStrLn $ "Part A: " ++ show (partA input)
  putStrLn $ "Part B: " ++ show (partB input)

------------ PARSER ------------
inputParser :: String -> Input
inputParser = lines

------------ TYPES ------------
type Input = [String]

type OutputA = Int

type OutputB = Int

------------ PART A ------------
partA :: Input -> OutputA
partA input = minimum $ map manhattan intersections
  where
    wire1 = pathToAllPoints $ wireToPointsWithSteps $ parseWire $ head input
    wire2 = pathToAllPoints $ wireToPointsWithSteps $ parseWire $ last input
    intersections = findIntersections wire1 wire2

------------ PART B ------------
partB :: Input -> OutputB
partB input = minimum $ map totalSteps intersections
  where
    wire1 = pathToAllPoints $ wireToPointsWithSteps $ parseWire $ head input
    wire2 = pathToAllPoints $ wireToPointsWithSteps $ parseWire $ last input
    intersections = findIntersections wire1 wire2
    totalSteps p = stepsToPoint wire1 p + stepsToPoint wire2 p

stepsToPoint :: [((Int, Int), Int)] -> (Int, Int) -> Int
stepsToPoint wire point = head [s | ((x, y), s) <- wire, (x, y) == point]

manhattan :: (Int, Int) -> Int
manhattan (x, y) = abs x + abs y

parseWire :: String -> [(Char, Int)]
parseWire input = map parseMove $ split ',' input
  where
    split c s = case break (== c) s of
      (x, []) -> [x]
      (x, _ : xs) -> x : split c xs
    parseMove x = (head x, read (tail x))

wireToPointsWithSteps :: [(Char, Int)] -> [((Int, Int), Int)]
wireToPointsWithSteps moves = tail $ scanl move ((0, 0), 0) moves
  where
    move ((x, y), steps) (dir, n) = case dir of
      'U' -> ((x, y + n), steps + n)
      'D' -> ((x, y - n), steps + n)
      'R' -> ((x + n, y), steps + n)
      'L' -> ((x - n, y), steps + n)
      _ -> error "Invalid direction"

pathToAllPoints :: [((Int, Int), Int)] -> [((Int, Int), Int)]
pathToAllPoints points = concat $ zipWith helpOp points (tail points)
  where
    helpOp ((x1, y1), s1) ((x2, y2), s2)
      | x1 == x2 = [((x1, y), s1 + abs (y - y1)) | y <- range y1 y2]
      | y1 == y2 = [((x, y1), s1 + abs (x - x1)) | x <- range x1 x2]
      | otherwise = error "Invalid"
    range a b = if a <= b then [a .. b] else reverse [b .. a]

findIntersections :: [((Int, Int), Int)] -> [((Int, Int), Int)] -> [(Int, Int)]
findIntersections wire1 wire2 =
  Set.toList $ Set.delete (0, 0) $ Set.intersection (Set.fromList (map fst wire1)) (Set.fromList (map fst wire2))
